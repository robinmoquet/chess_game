mod pawn;

use crate::{
    check::pawn::is_check_pawn,
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
}

pub fn _is_checkmate(_game: &GameState) -> bool {
    todo!("checkmate logical")
}
