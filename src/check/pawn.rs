use std::usize;

use crate::{
    types::{Color, PieceKind, Position, Squares},
    utils::is_in_board,
};

pub fn is_check_pawn(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    let mut targets: [(i8, i8); 2] = [(-1, -1), (1, -1)];
    if *color == Color::Black {
        targets = [(-1, 1), (1, 1)];
    }

    for target in targets {
        let (col, row) = (king_pos.col as i8 + target.0, king_pos.row as i8 + target.1);
        if !is_in_board(col, row) {
            continue;
        }

        let piece = squares[row as usize][col as usize].piece;
        if piece.is_none() {
            continue;
        }

        let piece = piece.unwrap();
        if piece.kind == PieceKind::Pawn && piece.color != *color {
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
    fn check_simple_pawn() {
        let game = initialize(Some("8/8/3ppp2/6k1/4K3/4PP2/8/8 b - - 0 1".to_string()));
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
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("d5".to_string(), game);

        assert_eq!(
            true,
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kd4".to_string(), game);
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
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kf5".to_string(), game);
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
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("e4".to_string(), game);

        assert_eq!(
            false,
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("dxe4".to_string(), game);

        assert_eq!(
            false,
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("fxe4".to_string(), game);

        assert_eq!(
            false,
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kf4".to_string(), game);
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
            is_check_pawn(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_pawn(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
}
