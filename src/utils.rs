use regex::Regex;

use crate::types::{Color, Piece, PieceKind, Position};

pub fn str_to_pos(str: &str) -> Result<Position, String> {
    let err = String::from("String must be a valid chess position. Ex: e4, g6, ...");
    if str.len() != 2 {
        return Err(err);
    }
    if !Regex::new(r"^([a-h][1-8])|(-)$").unwrap().is_match(str) {
        return Err(err);
    }
    let mut chars = str.chars().into_iter();
    let col = (chars.next().unwrap() as u8) - b'a';
    let row = chars.next().unwrap().to_string().parse::<i8>();
    if row.is_err() {
        return Err(err);
    }

    Ok(Position::new(col, (row.unwrap() - 8).abs() as u8))
}

pub fn char_to_piece(char: char) -> Piece {
    let kind = match char.to_ascii_lowercase() {
        'r' => PieceKind::Rook,
        'n' => PieceKind::Knight,
        'b' => PieceKind::Bishop,
        'k' => PieceKind::King,
        'q' => PieceKind::Queen,
        _ => PieceKind::Pawn,
    };
    let color = if char == char.to_ascii_uppercase() {
        Color::White
    } else {
        Color::Black
    };
    Piece::new(kind, color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(str_to_pos("").is_err());
    }

    #[test]
    fn str_not_valid() {
        assert!(str_to_pos("a").is_err());
        assert!(str_to_pos("abc").is_err());
        assert!(str_to_pos("foobar").is_err());
        assert!(str_to_pos("i4").is_err());
        assert!(str_to_pos("a9").is_err());
        assert!(str_to_pos("5a").is_err());
    }

    #[test]
    fn valid_pos_to_str() {
        assert_eq!(str_to_pos("a8"), Ok(Position::new(0, 0)));
        assert_eq!(str_to_pos("a7"), Ok(Position::new(0, 1)));
        assert_eq!(str_to_pos("a6"), Ok(Position::new(0, 2)));
        assert_eq!(str_to_pos("a5"), Ok(Position::new(0, 3)));
        assert_eq!(str_to_pos("a4"), Ok(Position::new(0, 4)));
        assert_eq!(str_to_pos("a3"), Ok(Position::new(0, 5)));
        assert_eq!(str_to_pos("a2"), Ok(Position::new(0, 6)));
        assert_eq!(str_to_pos("a1"), Ok(Position::new(0, 7)));

        assert_eq!(str_to_pos("b8"), Ok(Position::new(1, 0)));
        assert_eq!(str_to_pos("b7"), Ok(Position::new(1, 1)));
        assert_eq!(str_to_pos("b6"), Ok(Position::new(1, 2)));
        assert_eq!(str_to_pos("b5"), Ok(Position::new(1, 3)));
        assert_eq!(str_to_pos("b4"), Ok(Position::new(1, 4)));
        assert_eq!(str_to_pos("b3"), Ok(Position::new(1, 5)));
        assert_eq!(str_to_pos("b2"), Ok(Position::new(1, 6)));
        assert_eq!(str_to_pos("b1"), Ok(Position::new(1, 7)));

        assert_eq!(str_to_pos("c8"), Ok(Position::new(2, 0)));
        assert_eq!(str_to_pos("c7"), Ok(Position::new(2, 1)));
        assert_eq!(str_to_pos("c6"), Ok(Position::new(2, 2)));
        assert_eq!(str_to_pos("c5"), Ok(Position::new(2, 3)));
        assert_eq!(str_to_pos("c4"), Ok(Position::new(2, 4)));
        assert_eq!(str_to_pos("c3"), Ok(Position::new(2, 5)));
        assert_eq!(str_to_pos("c2"), Ok(Position::new(2, 6)));
        assert_eq!(str_to_pos("c1"), Ok(Position::new(2, 7)));

        assert_eq!(str_to_pos("d8"), Ok(Position::new(3, 0)));
        assert_eq!(str_to_pos("d7"), Ok(Position::new(3, 1)));
        assert_eq!(str_to_pos("d6"), Ok(Position::new(3, 2)));
        assert_eq!(str_to_pos("d5"), Ok(Position::new(3, 3)));
        assert_eq!(str_to_pos("d4"), Ok(Position::new(3, 4)));
        assert_eq!(str_to_pos("d3"), Ok(Position::new(3, 5)));
        assert_eq!(str_to_pos("d2"), Ok(Position::new(3, 6)));
        assert_eq!(str_to_pos("d1"), Ok(Position::new(3, 7)));

        assert_eq!(str_to_pos("e8"), Ok(Position::new(4, 0)));
        assert_eq!(str_to_pos("e7"), Ok(Position::new(4, 1)));
        assert_eq!(str_to_pos("e6"), Ok(Position::new(4, 2)));
        assert_eq!(str_to_pos("e5"), Ok(Position::new(4, 3)));
        assert_eq!(str_to_pos("e4"), Ok(Position::new(4, 4)));
        assert_eq!(str_to_pos("e3"), Ok(Position::new(4, 5)));
        assert_eq!(str_to_pos("e2"), Ok(Position::new(4, 6)));
        assert_eq!(str_to_pos("e1"), Ok(Position::new(4, 7)));

        assert_eq!(str_to_pos("f8"), Ok(Position::new(5, 0)));
        assert_eq!(str_to_pos("f7"), Ok(Position::new(5, 1)));
        assert_eq!(str_to_pos("f6"), Ok(Position::new(5, 2)));
        assert_eq!(str_to_pos("f5"), Ok(Position::new(5, 3)));
        assert_eq!(str_to_pos("f4"), Ok(Position::new(5, 4)));
        assert_eq!(str_to_pos("f3"), Ok(Position::new(5, 5)));
        assert_eq!(str_to_pos("f2"), Ok(Position::new(5, 6)));
        assert_eq!(str_to_pos("f1"), Ok(Position::new(5, 7)));

        assert_eq!(str_to_pos("g8"), Ok(Position::new(6, 0)));
        assert_eq!(str_to_pos("g7"), Ok(Position::new(6, 1)));
        assert_eq!(str_to_pos("g6"), Ok(Position::new(6, 2)));
        assert_eq!(str_to_pos("g5"), Ok(Position::new(6, 3)));
        assert_eq!(str_to_pos("g4"), Ok(Position::new(6, 4)));
        assert_eq!(str_to_pos("g3"), Ok(Position::new(6, 5)));
        assert_eq!(str_to_pos("g2"), Ok(Position::new(6, 6)));
        assert_eq!(str_to_pos("g1"), Ok(Position::new(6, 7)));

        assert_eq!(str_to_pos("h8"), Ok(Position::new(7, 0)));
        assert_eq!(str_to_pos("h7"), Ok(Position::new(7, 1)));
        assert_eq!(str_to_pos("h6"), Ok(Position::new(7, 2)));
        assert_eq!(str_to_pos("h5"), Ok(Position::new(7, 3)));
        assert_eq!(str_to_pos("h4"), Ok(Position::new(7, 4)));
        assert_eq!(str_to_pos("h3"), Ok(Position::new(7, 5)));
        assert_eq!(str_to_pos("h2"), Ok(Position::new(7, 6)));
        assert_eq!(str_to_pos("h1"), Ok(Position::new(7, 7)));
    }

    #[test]
    fn valid_piece() {
        assert_eq!(
            Piece::new(PieceKind::Pawn, Color::White),
            char_to_piece('P')
        );
        assert_eq!(
            Piece::new(PieceKind::Rook, Color::White),
            char_to_piece('R')
        );
        assert_eq!(
            Piece::new(PieceKind::Knight, Color::White),
            char_to_piece('N')
        );
        assert_eq!(
            Piece::new(PieceKind::Bishop, Color::White),
            char_to_piece('B')
        );
        assert_eq!(
            Piece::new(PieceKind::King, Color::White),
            char_to_piece('K')
        );
        assert_eq!(
            Piece::new(PieceKind::Queen, Color::White),
            char_to_piece('Q')
        );
        assert_eq!(
            Piece::new(PieceKind::Pawn, Color::Black),
            char_to_piece('p')
        );
        assert_eq!(
            Piece::new(PieceKind::Rook, Color::Black),
            char_to_piece('r')
        );
        assert_eq!(
            Piece::new(PieceKind::Knight, Color::Black),
            char_to_piece('n')
        );
        assert_eq!(
            Piece::new(PieceKind::Bishop, Color::Black),
            char_to_piece('b')
        );
        assert_eq!(
            Piece::new(PieceKind::King, Color::Black),
            char_to_piece('k')
        );
        assert_eq!(
            Piece::new(PieceKind::Queen, Color::Black),
            char_to_piece('q')
        );
    }
}
