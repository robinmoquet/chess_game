use crate::{
    fen::{errors::ParseFenError, types::FEN, validate::is_valid},
    types::{Color, Square, Squares},
    utils::{char_to_piece, str_to_pos},
};

/// Parse string Forsyth-Edwards Notation
pub fn parse(fen: String) -> Result<FEN, ParseFenError> {
    if !is_valid(&fen) {
        return Err(ParseFenError::new("Not a valid FEN string !"));
    }
    let parts: Vec<&str> = fen.split(" ").collect();

    Ok(FEN {
        squares: str_to_squares(parts[0])?,
        active_color: if parts[1] == "w" {
            Color::White
        } else {
            Color::Black
        },
        castling_possibilities: if parts[2] == "-" {
            None
        } else {
            Some(parts[2].to_string())
        },
        en_passant_target: if parts[3] == "-" {
            None
        } else {
            Some(str_to_pos(parts[3]).unwrap())
        },
        halfmove_clock: parts[4].to_string().parse::<u8>().unwrap(),
        fullmove_number: parts[5].to_string().parse::<u16>().unwrap(),
    })
}

pub fn str_to_squares(str: &str) -> Result<Squares, ParseFenError> {
    let rows: Vec<&str> = str.split("/").collect();
    if rows.len() != 8 {
        return Err(ParseFenError::new("Not valid board representation !"));
    }

    let mut squares: Squares = [[Square::new(None); 8]; 8];

    for (r, row) in rows.iter().enumerate() {
        let mut c: usize = 0;
        for char in row.chars() {
            if char.is_ascii_digit() {
                c += char.to_digit(10).unwrap() as usize;
            } else {
                squares[r][c] = Square::new(Some(char_to_piece(char)));
                c += 1;
            }
        }
    }

    Ok(squares)
}

#[cfg(test)]
mod tests {
    use crate::types::{Piece, PieceKind, Position, Square};

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
        assert_eq!(Some(String::from("KQkq")), fen.castling_possibilities);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(1, fen.fullmove_number);

        let squares: Squares = [
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::King, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::White))),
                Square::new(Some(Piece::new(PieceKind::King, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
            ],
        ];

        assert_eq!(squares, fen.squares)
    }

    #[test]
    fn opening() {
        let fen = parse(String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::White, fen.active_color);
        assert_eq!(Some(String::from("KQkq")), fen.castling_possibilities);
        assert_eq!(Some(Position::new(2, 2)), fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(2, fen.fullmove_number);

        let squares: Squares = [
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::King, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::White))),
                Square::new(Some(Piece::new(PieceKind::King, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
            ],
        ];

        assert_eq!(squares, fen.squares);
    }

    #[test]
    fn midgame() {
        let fen = parse(String::from(
            "1B6/2n5/p1N1P2R/P1K3N1/4Pk2/1Q2p2p/6nP/1B4R1 w - - 0 1",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::White, fen.active_color);
        assert_eq!(None, fen.castling_possibilities);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(0, fen.halfmove_clock);
        assert_eq!(1, fen.fullmove_number);

        let squares: Squares = [
            [
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::King, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::King, Color::Black))),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
            ],
            [
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
                Square::new(None),
            ],
        ];

        assert_eq!(squares, fen.squares);
    }

    #[test]
    fn endgame() {
        let fen = parse(String::from(
            "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
        ));

        assert!(fen.is_ok());
        let fen = fen.unwrap();
        assert_eq!(Color::Black, fen.active_color);
        assert_eq!(None, fen.castling_possibilities);
        assert_eq!(None, fen.en_passant_target);
        assert_eq!(99, fen.halfmove_clock);
        assert_eq!(50, fen.fullmove_number);

        let squares: Squares = [
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::King, Color::Black))),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(None),
                Square::new(Some(Piece::new(PieceKind::King, Color::White))),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
        ];

        assert_eq!(squares, fen.squares);
    }
}
