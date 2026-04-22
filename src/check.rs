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

pub fn _is_checkmate(_game: &GameState) -> bool {
    todo!("checkmate logical")
}
