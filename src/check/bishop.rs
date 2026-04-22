use std::usize;

use crate::{
    types::{Color, PieceKind, Position, Squares},
    utils::is_in_board,
};

pub fn is_check_bishop(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    let directions = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
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
            if (piece.kind == PieceKind::Bishop || piece.kind == PieceKind::Queen)
                && piece.color != *color
            {
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
    fn check_simple_bishop() {
        let game = initialize(Some("8/b2k1b2/8/8/8/B2K1B2/8/8 w - - 1 1".to_string()));
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
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Bg4".to_string(), game);

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kd8".to_string(), game);
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Be7".to_string(), game);

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            true,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kxe7".to_string(), game);
        let bking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::Black),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Bf3".to_string(), game);
        let (_, game) = do_action("Bg6".to_string(), game);

        assert_eq!(
            true,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Be4".to_string(), game);

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Bxe4".to_string(), game);

        assert_eq!(
            true,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );

        let (_, game) = do_action("Kxe4".to_string(), game);
        let wking_pos = find_piece(
            &Piece::new(PieceKind::King, Color::White),
            &game.board.squares,
        );

        assert_eq!(
            false,
            is_check_bishop(&wking_pos.unwrap(), &Color::White, &game.board.squares)
        );
        assert_eq!(
            false,
            is_check_bishop(&bking_pos.unwrap(), &Color::Black, &game.board.squares)
        );
    }
}
