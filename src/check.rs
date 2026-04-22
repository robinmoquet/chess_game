mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use crate::{
    check::bishop::is_check_bishop,
    check::king::is_check_king,
    check::knight::is_check_knight,
    check::pawn::is_check_pawn,
    check::queen::is_check_queen,
    check::rook::is_check_rook,
    types::{Color, GameState, Piece, PieceKind},
    utils::find_piece,
};

pub fn is_check(game: &GameState, color: &Color) -> bool {
    let king_pos = find_piece(
        &Piece::new(PieceKind::King, game.current_player),
        &game.board.squares,
    );
    if king_pos.is_none() {
        panic!("Board must have a king");
    }

    is_check_pawn(&king_pos.unwrap(), color, &game.board.squares)
        || is_check_knight(&king_pos.unwrap(), color, &game.board.squares)
        || is_check_bishop(&king_pos.unwrap(), color, &game.board.squares)
        || is_check_rook(&king_pos.unwrap(), color, &game.board.squares)
        || is_check_queen(&king_pos.unwrap(), color, &game.board.squares)
        || is_check_king(&king_pos.unwrap(), color, &game.board.squares)
}

pub fn _is_checkmate(_game: &GameState) -> bool {
    todo!("checkmate logical")
}
