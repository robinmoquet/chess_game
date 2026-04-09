use regex::Regex;

use crate::{
    errors::ActionError,
    types::{Action, ActionKind, Color, DisambigPosition, PieceKind, Position},
    utils::{char_to_piece, str_to_disambig, str_to_pos},
};

pub fn parse(san: &str, color: Option<Color>) -> Result<Action, ActionError> {
    if !Regex::new("^([a-h](x?[a-h])?[1-8](=[QRBN])?|[KQRBN]([a-h]|[1-8]|[a-h][1-8])?x?[a-h][1-8]|O-O(-O)?)[+#]?$").unwrap().is_match(san) {
        return Err(ActionError::new("SAN bad format !"));
    }
    let pattern = r"(?x)
        ^
        (?:
            # Pawn (ex: e4, dxe4, a8=Q)
            (?:(?P<p_from>[a-h])?(?P<p_cap>x)?(?P<p_to>[a-h][1-8])(?P<p_prom>=[QRBN])?)
            |
            # Pieces (ex: Nf3, Ng1f3, Nxf3)
            (?:(?P<piece>[KQRBN])(?P<disambig>[a-h]|[1-8]|[a-h][1-8])?(?P<cap>x)?(?P<to>[a-h][1-8]))
            |
            # Castling
            (?P<castle>O-O(?:-O)?)
        )
        (?P<check>[+\#])?
        $
    ";
    let re = Regex::new(pattern).ok().unwrap();
    let caps = re.captures(san).unwrap();
    let mut action = Action::new(
        Position::new(0, 0),
        ActionKind::Move,
        PieceKind::Pawn,
        None,
        false,
        false,
        None,
    );

    if let Some(c) = caps.name("check") {
        if c.as_str() == "#" {
            action.checkmate = true;
        } else {
            action.check = true;
        }
    }

    if let Some(c) = caps.name("castle") {
        action.piece_kind = PieceKind::King;
        if c.as_str() == "O-O-O" {
            action.kind = ActionKind::Queencastling;
            action.to = if color == Some(Color::White) {
                str_to_pos("c1").unwrap()
            } else {
                str_to_pos("c8").unwrap()
            }
        } else {
            action.kind = ActionKind::Kingcastling;
            action.to = if color == Some(Color::White) {
                str_to_pos("g1").unwrap()
            } else {
                str_to_pos("g8").unwrap()
            }
        }

        return Ok(action);
    }

    if let Some(c) = caps.name("piece") {
        action.piece_kind = char_to_piece(c.as_str().chars().last().unwrap()).kind;
        if caps.name("cap").is_some() {
            action.kind = ActionKind::Capture
        }
        if let Some(t) = caps.name("to") {
            action.to = str_to_pos(t.as_str()).unwrap();
        }
        if let Some(d) = caps.name("disambig") {
            action.from = Some(str_to_disambig(d.as_str()).unwrap());
        }
    } else {
        if caps.name("p_cap").is_some() {
            action.kind = ActionKind::Capture
        }
        if let Some(t) = caps.name("p_to") {
            action.to = str_to_pos(t.as_str()).unwrap();
        }
        if let Some(pr) = caps.name("p_prom") {
            action.promotion =
                Some(char_to_piece(pr.as_str().replace("=", "").chars().last().unwrap()).kind);
        }
        if let Some(d) = caps.name("p_from") {
            action.from = Some(str_to_disambig(d.as_str()).unwrap());
        }
    }

    Ok(action)
}

#[cfg(test)]
mod tests {
    use crate::{types::DisambigPosition, utils::str_to_pos};

    use super::*;

    #[test]
    fn parse_invalid() {
        assert!(parse("", None).is_err());
        assert!(parse("foobar", None).is_err());
        assert!(parse("Ni3", None).is_err());
    }

    #[test]
    fn parse_pawn() {
        assert_eq!(
            Action::new(
                str_to_pos("e4").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                false,
                false,
                None
            ),
            parse("e4", None).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("e4").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                true,
                false,
                None
            ),
            parse("e4+", None).unwrap()
        );
        let e5 = str_to_pos("e5").unwrap();
        let d1 = str_to_pos("d1").unwrap();
        assert_eq!(
            Action::new(
                e5,
                ActionKind::Capture,
                PieceKind::Pawn,
                Some(DisambigPosition::new(Some(d1.col), None)),
                false,
                false,
                None
            ),
            parse("dxe5", None).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("a8").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                false,
                false,
                Some(PieceKind::Queen)
            ),
            parse("a8=Q", None).unwrap()
        );
    }

    #[test]
    fn parse_piece() {
        assert_eq!(
            Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Move,
                PieceKind::Knight,
                None,
                false,
                false,
                None
            ),
            parse("Nf3", None).unwrap()
        );
        let f3 = str_to_pos("f3").unwrap();
        let g1 = str_to_pos("g1").unwrap();
        assert_eq!(
            Action::new(
                f3,
                ActionKind::Move,
                PieceKind::Knight,
                Some(DisambigPosition::new(Some(g1.col), Some(g1.row))),
                false,
                false,
                None
            ),
            parse("Ng1f3", None).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Capture,
                PieceKind::Knight,
                None,
                false,
                false,
                None
            ),
            parse("Nxf3", None).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Move,
                PieceKind::Queen,
                None,
                false,
                true,
                None
            ),
            parse("Qf3#", None).unwrap()
        );
    }

    #[test]
    fn parse_castling() {
        assert_eq!(
            Action::new(
                str_to_pos("g1").unwrap(),
                ActionKind::Kingcastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ),
            parse("O-O", Some(Color::White)).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("c1").unwrap(),
                ActionKind::Queencastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ),
            parse("O-O-O", Some(Color::White)).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("g8").unwrap(),
                ActionKind::Kingcastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ),
            parse("O-O", Some(Color::Black)).unwrap()
        );
        assert_eq!(
            Action::new(
                str_to_pos("c8").unwrap(),
                ActionKind::Queencastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ),
            parse("O-O-O", Some(Color::Black)).unwrap()
        );
    }
}
