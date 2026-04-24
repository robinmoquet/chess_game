mod bishop;
mod king;
mod knight;
mod pawn;
mod rook;

use crate::{
    check::{
        bishop::is_check_bishop, king::is_check_king, knight::is_check_knight, pawn::is_check_pawn,
        rook::is_check_rook,
    },
    move_compute::all_move_possibilities,
    types::{Color, GameState, Piece, PieceKind, Squares},
    utils::find_piece,
};

pub fn is_check(squares: &Squares, color: &Color) -> bool {
    let king_pos = find_piece(&Piece::new(PieceKind::King, *color), squares);
    if king_pos.is_none() {
        panic!("Board must have a king");
    }

    is_check_pawn(&king_pos.unwrap(), color, squares)
        || is_check_knight(&king_pos.unwrap(), color, squares)
        || is_check_bishop(&king_pos.unwrap(), color, squares)
        || is_check_rook(&king_pos.unwrap(), color, squares)
        || is_check_king(&king_pos.unwrap(), color, squares)
}

pub fn is_checkmate(game: &GameState, color: &Color) -> bool {
    let moves = all_move_possibilities(game, color);
    for (_, possibilities) in moves {
        if possibilities.len() != 0 {
            return false;
        }
    }

    true
}
