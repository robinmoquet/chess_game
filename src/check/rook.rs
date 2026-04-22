use std::usize;

use crate::{
    types::{Color, PieceKind, Position, Squares},
    utils::is_in_board,
};

pub fn is_check_rook(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (dcol, drow) in directions {
        let (mut col, mut row) = (king_pos.col as i8, king_pos.row as i8);
        loop {
            col = col + dcol;
            row = row + drow;

            if !is_in_board(col, row) {
                break;
            }

            let piece = squares[row as usize][col as usize].piece;
            if piece.is_none() {
                continue;
            }

            let piece = piece.unwrap();
            if piece.kind == PieceKind::Rook && piece.color != *color {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{
        game::do_action,
        initialize,
        types::{Piece, PieceKind},
        utils::find_piece,
    };

    use super::*;

    #[test]
    fn check_simple_rook() {
        let game = initialize(Some("7r/6k1/r7/8/8/1R6/3K4/7R w - - 1 1".to_string()));
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
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Rg1".to_string(), game);

        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kf8".to_string(), game);
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Rf3".to_string(), game);
        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
        let (_, game) = do_action("Rf6".to_string(), game);
        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
        let (_, game) = do_action("Rxf6".to_string(), game);

        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
        let (_, game) = do_action("Ke7".to_string(), game);
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );
        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Rgf1".to_string(), game);
        let (_, game) = do_action("Rh2".to_string(), game);

        assert_eq!(
            true,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("R1f2".to_string(), game);
        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Rxf2".to_string(), game);
        assert_eq!(
            true,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Rxf2".to_string(), game);
        assert_eq!(
            false,
            is_check_rook(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_rook(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
}
