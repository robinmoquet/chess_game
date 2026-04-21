use std::usize;

use crate::{
    types::{Color, PieceKind, Position, Squares},
    utils::is_in_board,
};

pub fn is_check_knight(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    let possibilities: [(i8, i8); 8] = [
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
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
        if piece.kind == PieceKind::Knight && piece.color != *color {
            return true;
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
    fn check_simple_knight() {
        let game = initialize(Some("8/2k5/8/2n1N3/8/3K4/8/8 w - - 1 1".to_string()));
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
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kd4".to_string(), game);
        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Nb3".to_string(), game);

        assert_eq!(
            true,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kd5".to_string(), game);

        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Nd2".to_string(), game);
        let (_, game) = do_action("Nc4".to_string(), game);
        let (_, game) = do_action("Nf1".to_string(), game);
        let (_, game) = do_action("Nd6".to_string(), game);
        let (_, game) = do_action("Ng3".to_string(), game);
        let (_, game) = do_action("Ng3".to_string(), game);

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Ne8".to_string(), game);

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kd7".to_string(), game);
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Nf6".to_string(), game);

        assert_eq!(
            false,
            is_check_knight(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_knight(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
}
