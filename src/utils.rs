use regex::Regex;

use crate::types::{Castling, Color, Piece, PieceKind, Position, Squares};

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

pub fn pos_to_str(position: &Position) -> String {
    let col = (position.col + b'a') as char;
    let row = 8 - position.row;
    format!("{}{}", col.to_string(), row)
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

pub fn str_to_castling(str: &str) -> Castling {
    let mut k = (false, false);
    let mut q = (false, false);

    if str.contains('K') {
        k.0 = true
    }
    if str.contains('k') {
        k.1 = true
    }
    if str.contains('Q') {
        q.0 = true
    }
    if str.contains('q') {
        q.1 = true
    }

    Castling::new(k, q)
}
pub fn castling_to_str(castling: &Castling) -> String {
    let mut r = String::new();

    if castling.kingside.0 {
        r.push('K');
    }
    if castling.queenside.0 {
        r.push('Q');
    }
    if castling.kingside.1 {
        r.push('k');
    }
    if castling.queenside.1 {
        r.push('q');
    }

    if r.len() == 0 {
        r.push('-');
    }
    r
}

pub fn is_empty_square(squares: &Squares, pos: &Position) -> bool {
    squares[pos.row as usize][pos.col as usize].piece.is_none()
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
        assert_eq!("a8".to_string(), pos_to_str(&Position::new(0, 0)));
        assert_eq!("a7".to_string(), pos_to_str(&Position::new(0, 1)));
        assert_eq!("a6".to_string(), pos_to_str(&Position::new(0, 2)));
        assert_eq!("a5".to_string(), pos_to_str(&Position::new(0, 3)));
        assert_eq!("a4".to_string(), pos_to_str(&Position::new(0, 4)));
        assert_eq!("a3".to_string(), pos_to_str(&Position::new(0, 5)));
        assert_eq!("a2".to_string(), pos_to_str(&Position::new(0, 6)));
        assert_eq!("a1".to_string(), pos_to_str(&Position::new(0, 7)));

        assert_eq!("b8".to_string(), pos_to_str(&Position::new(1, 0)));
        assert_eq!("b7".to_string(), pos_to_str(&Position::new(1, 1)));
        assert_eq!("b6".to_string(), pos_to_str(&Position::new(1, 2)));
        assert_eq!("b5".to_string(), pos_to_str(&Position::new(1, 3)));
        assert_eq!("b4".to_string(), pos_to_str(&Position::new(1, 4)));
        assert_eq!("b3".to_string(), pos_to_str(&Position::new(1, 5)));
        assert_eq!("b2".to_string(), pos_to_str(&Position::new(1, 6)));
        assert_eq!("b1".to_string(), pos_to_str(&Position::new(1, 7)));

        assert_eq!("c8".to_string(), pos_to_str(&Position::new(2, 0)));
        assert_eq!("c7".to_string(), pos_to_str(&Position::new(2, 1)));
        assert_eq!("c6".to_string(), pos_to_str(&Position::new(2, 2)));
        assert_eq!("c5".to_string(), pos_to_str(&Position::new(2, 3)));
        assert_eq!("c4".to_string(), pos_to_str(&Position::new(2, 4)));
        assert_eq!("c3".to_string(), pos_to_str(&Position::new(2, 5)));
        assert_eq!("c2".to_string(), pos_to_str(&Position::new(2, 6)));
        assert_eq!("c1".to_string(), pos_to_str(&Position::new(2, 7)));

        assert_eq!("d8".to_string(), pos_to_str(&Position::new(3, 0)));
        assert_eq!("d7".to_string(), pos_to_str(&Position::new(3, 1)));
        assert_eq!("d6".to_string(), pos_to_str(&Position::new(3, 2)));
        assert_eq!("d5".to_string(), pos_to_str(&Position::new(3, 3)));
        assert_eq!("d4".to_string(), pos_to_str(&Position::new(3, 4)));
        assert_eq!("d3".to_string(), pos_to_str(&Position::new(3, 5)));
        assert_eq!("d2".to_string(), pos_to_str(&Position::new(3, 6)));
        assert_eq!("d1".to_string(), pos_to_str(&Position::new(3, 7)));

        assert_eq!("e8".to_string(), pos_to_str(&Position::new(4, 0)));
        assert_eq!("e7".to_string(), pos_to_str(&Position::new(4, 1)));
        assert_eq!("e6".to_string(), pos_to_str(&Position::new(4, 2)));
        assert_eq!("e5".to_string(), pos_to_str(&Position::new(4, 3)));
        assert_eq!("e4".to_string(), pos_to_str(&Position::new(4, 4)));
        assert_eq!("e3".to_string(), pos_to_str(&Position::new(4, 5)));
        assert_eq!("e2".to_string(), pos_to_str(&Position::new(4, 6)));
        assert_eq!("e1".to_string(), pos_to_str(&Position::new(4, 7)));

        assert_eq!("f8".to_string(), pos_to_str(&Position::new(5, 0)));
        assert_eq!("f7".to_string(), pos_to_str(&Position::new(5, 1)));
        assert_eq!("f6".to_string(), pos_to_str(&Position::new(5, 2)));
        assert_eq!("f5".to_string(), pos_to_str(&Position::new(5, 3)));
        assert_eq!("f4".to_string(), pos_to_str(&Position::new(5, 4)));
        assert_eq!("f3".to_string(), pos_to_str(&Position::new(5, 5)));
        assert_eq!("f2".to_string(), pos_to_str(&Position::new(5, 6)));
        assert_eq!("f1".to_string(), pos_to_str(&Position::new(5, 7)));

        assert_eq!("g8".to_string(), pos_to_str(&Position::new(6, 0)));
        assert_eq!("g7".to_string(), pos_to_str(&Position::new(6, 1)));
        assert_eq!("g6".to_string(), pos_to_str(&Position::new(6, 2)));
        assert_eq!("g5".to_string(), pos_to_str(&Position::new(6, 3)));
        assert_eq!("g4".to_string(), pos_to_str(&Position::new(6, 4)));
        assert_eq!("g3".to_string(), pos_to_str(&Position::new(6, 5)));
        assert_eq!("g2".to_string(), pos_to_str(&Position::new(6, 6)));
        assert_eq!("g1".to_string(), pos_to_str(&Position::new(6, 7)));

        assert_eq!("h8".to_string(), pos_to_str(&Position::new(7, 0)));
        assert_eq!("h7".to_string(), pos_to_str(&Position::new(7, 1)));
        assert_eq!("h6".to_string(), pos_to_str(&Position::new(7, 2)));
        assert_eq!("h5".to_string(), pos_to_str(&Position::new(7, 3)));
        assert_eq!("h4".to_string(), pos_to_str(&Position::new(7, 4)));
        assert_eq!("h3".to_string(), pos_to_str(&Position::new(7, 5)));
        assert_eq!("h2".to_string(), pos_to_str(&Position::new(7, 6)));
        assert_eq!("h1".to_string(), pos_to_str(&Position::new(7, 7)));
    }

    #[test]
    fn valid_str_to_pos() {
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

    #[test]
    fn valid_str_to_castling() {
        assert_eq!(
            Castling::new((false, false), (false, false)),
            str_to_castling("-")
        );
        assert_eq!(
            Castling::new((true, false), (false, false)),
            str_to_castling("K")
        );
        assert_eq!(
            Castling::new((false, false), (true, false)),
            str_to_castling("Q")
        );
        assert_eq!(
            Castling::new((false, true), (false, false)),
            str_to_castling("k")
        );
        assert_eq!(
            Castling::new((false, false), (false, true)),
            str_to_castling("q")
        );
        assert_eq!(
            Castling::new((true, false), (true, false)),
            str_to_castling("KQ")
        );
        assert_eq!(
            Castling::new((false, true), (false, true)),
            str_to_castling("kq")
        );
        assert_eq!(
            Castling::new((true, true), (true, false)),
            str_to_castling("KQk")
        );
        assert_eq!(
            Castling::new((true, false), (true, true)),
            str_to_castling("KQq")
        );
        assert_eq!(
            Castling::new((true, true), (true, true)),
            str_to_castling("KQkq")
        );
    }

    #[test]
    fn valid_castling_to_str() {
        assert_eq!(
            "-",
            castling_to_str(&Castling::new((false, false), (false, false)))
        );
        assert_eq!(
            "K",
            castling_to_str(&Castling::new((true, false), (false, false)))
        );
        assert_eq!(
            "Kk",
            castling_to_str(&Castling::new((true, true), (false, false)))
        );
        assert_eq!(
            "Qq",
            castling_to_str(&Castling::new((false, false), (true, true)))
        );
        assert_eq!(
            "KQkq",
            castling_to_str(&Castling::new((true, true), (true, true)))
        );
    }
}
