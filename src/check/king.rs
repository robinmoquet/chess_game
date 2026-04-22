use std::usize;

use crate::{
    types::{Color, PieceKind, Position, Squares},
    utils::is_in_board,
};

pub fn is_check_king(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    let possibilities: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for (col, row) in possibilities {
        let (col, row) = (king_pos.col as i8 + col, king_pos.row as i8 + row);

        if !is_in_board(col, row) {
            continue;
        }

        let piece = squares[row as usize][col as usize].piece;
        if piece.is_none() {
            continue;
        }

        let piece = piece.unwrap();
        if piece.kind == PieceKind::King && piece.color != *color {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{
        initialize,
        types::{Piece, PieceKind},
        utils::find_piece,
    };

    use super::*;

    #[test]
    fn no_check_king() {
        let game = initialize(Some("8/8/8/4k3/8/4K3/8/8 w - - 1 1".to_string()));
        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_king(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_king(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
    #[test]
    fn top_check_king() {
        let game = initialize(Some("8/8/8/8/5k2/4K3/8/8 w - - 1 1".to_string()));
        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            true,
            is_check_king(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_king(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
    #[test]
    fn bottom_check_king() {
        let game = initialize(Some("8/8/8/5K2/5k2/8/8/8 w - - 1 1".to_string()));
        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            true,
            is_check_king(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_king(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
}
