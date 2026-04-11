use regex::Regex;

use crate::{
    errors::ActionError,
    printer::print_piece_kind,
    types::{Action, ActionKind, Color, PieceKind, Position},
    utils::{char_to_piece, disambig_to_str, pos_to_str, str_to_disambig, str_to_pos},
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

pub fn to_string(action: &Action) -> String {
    if action.kind == ActionKind::Kingcastling {
        return "O-O".to_string();
    }
    if action.kind == ActionKind::Queencastling {
        return "O-O-O".to_string();
    }

    let to = pos_to_str(&action.to);
    let mut prom: Option<String> = None;
    if action.promotion.is_some() {
        prom = Some(format!("={}", print_piece_kind(&action.promotion.unwrap())));
    }
    let mut piece: Option<String> = None;
    if action.piece_kind != PieceKind::Pawn {
        piece = Some(print_piece_kind(&action.piece_kind).to_string());
    }
    let mut disambig: Option<String> = None;
    if action.from.is_some() {
        disambig = Some(disambig_to_str(&action.from.unwrap()).to_string());
    }

    format!(
        "{}{}{}{}{}{}{}",
        piece.as_deref().unwrap_or(""),
        disambig.as_deref().unwrap_or(""),
        if action.kind == ActionKind::Capture {
            "x".to_string()
        } else {
            "".to_string()
        },
        to,
        prom.as_deref().unwrap_or(""),
        if action.check {
            "+".to_string()
        } else {
            "".to_string()
        },
        if action.checkmate {
            "#".to_string()
        } else {
            "".to_string()
        },
    )
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

    #[test]
    fn to_string_pawn() {
        assert_eq!(
            "e4".to_string(),
            to_string(&Action::new(
                str_to_pos("e4").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                false,
                false,
                None
            )),
        );
        assert_eq!(
            "e4+".to_string(),
            to_string(&Action::new(
                str_to_pos("e4").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                true,
                false,
                None
            )),
        );
        let e5 = str_to_pos("e5").unwrap();
        let d1 = str_to_pos("d1").unwrap();
        assert_eq!(
            "dxe5".to_string(),
            to_string(&Action::new(
                e5,
                ActionKind::Capture,
                PieceKind::Pawn,
                Some(DisambigPosition::new(Some(d1.col), None)),
                false,
                false,
                None
            )),
        );
        assert_eq!(
            "a8=Q".to_string(),
            to_string(&Action::new(
                str_to_pos("a8").unwrap(),
                ActionKind::Move,
                PieceKind::Pawn,
                None,
                false,
                false,
                Some(PieceKind::Queen)
            )),
        );
    }

    #[test]
    fn to_string_piece() {
        assert_eq!(
            "Nf3".to_string(),
            to_string(&Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Move,
                PieceKind::Knight,
                None,
                false,
                false,
                None
            )),
        );
        let f3 = str_to_pos("f3").unwrap();
        let g1 = str_to_pos("g1").unwrap();
        assert_eq!(
            "Ng1f3".to_string(),
            to_string(&Action::new(
                f3,
                ActionKind::Move,
                PieceKind::Knight,
                Some(DisambigPosition::new(Some(g1.col), Some(g1.row))),
                false,
                false,
                None
            )),
        );
        assert_eq!(
            "Nxf3".to_string(),
            to_string(&Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Capture,
                PieceKind::Knight,
                None,
                false,
                false,
                None
            )),
        );
        assert_eq!(
            "Qf3#".to_string(),
            to_string(&Action::new(
                str_to_pos("f3").unwrap(),
                ActionKind::Move,
                PieceKind::Queen,
                None,
                false,
                true,
                None
            )),
        );
    }

    #[test]
    fn to_string_castling() {
        assert_eq!(
            "O-O".to_string(),
            to_string(&Action::new(
                str_to_pos("g1").unwrap(),
                ActionKind::Kingcastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ))
        );
        assert_eq!(
            "O-O-O".to_string(),
            to_string(&Action::new(
                str_to_pos("c1").unwrap(),
                ActionKind::Queencastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            )),
        );
        assert_eq!(
            "O-O".to_string(),
            to_string(&Action::new(
                str_to_pos("g8").unwrap(),
                ActionKind::Kingcastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ))
        );
        assert_eq!(
            "O-O-O".to_string(),
            to_string(&Action::new(
                str_to_pos("c8").unwrap(),
                ActionKind::Queencastling,
                PieceKind::King,
                None,
                false,
                false,
                None
            ))
        );
    }
}
