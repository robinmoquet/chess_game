use crate::{
    fen::{errors::ParseFenError, types::FEN, validate::is_valid},
    types::Color,
};

/// Parse string Forsyth-Edwards Notation
pub fn parse(fen: String) -> Result<FEN, ParseFenError> {
    if !is_valid(&fen) {
        return Err(ParseFenError::new("Not a valid FEN string !"));
    }

    Ok(FEN {
        active_color: Color::White,
        castling_possibility: Some(String::new()),
        en_passant_target: None,
        halfmove_clock: 0,
        fullmove_number: 0,
    })
}

#[cfg(test)]
mod tests {
    use crate::types::Position;

    use super::*;
    #[test]
    fn empty() {
        assert!(parse(String::new()).is_err());
    }

    #[test]
    fn bad_format() {
        assert!(parse(String::from("foobar")).is_err());
        assert!(parse(String::from("foo bar")).is_err());
        assert!(parse(String::from("foo bar foo bar foo bar")).is_err());
    }

    #[test]
    fn starting_position() {
        let fen = parse(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::White, fen.active_color);
        assert_eq!(Some(String::from("KQkq")), fen.castling_possibility);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(0, fen.fullmove_number);
    }

    #[test]
    fn opening() {
        let fen = parse(String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::White, fen.active_color);
        assert_eq!(Some(String::from("KQkq")), fen.castling_possibility);
        assert_eq!(Some(Position::new(2, 2)), fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(2, fen.fullmove_number);
    }

    #[test]
    fn midgame() {
        let fen = parse(String::from(
            "1B6/2n5/p1N1P2R/P1K3N1/4Pk2/1Q2p2p/6nP/1B4R1 w - - 0 1",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::White, fen.active_color);
        assert_eq!(None, fen.castling_possibility);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(1, fen.fullmove_number);
    }

    #[test]
    fn endgame() {
        let fen = parse(String::from(
            "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::Black, fen.active_color);
        assert_eq!(None, fen.castling_possibility);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(99, fen.halfmove_clock);
        assert_eq!(50, fen.fullmove_number);
    }
}
