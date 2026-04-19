mod bishop;
mod knight;
mod pawn;
mod utils;

use crate::{
    move_compute::bishop::bishop_move_possibilities,
    move_compute::knight::knight_move_possibilities,
    move_compute::pawn::pawn_move_possibilities,
    types::{GameState, Piece, PieceKind, Position},
};

pub use utils::{backward_one_square, forward_one_square, move_delta};

pub fn move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    match piece.kind {
        PieceKind::Pawn => pawn_move_possibilities(game, piece, pos),
        // PieceKind::Rook => rook_move_possibilities(game, piece, pos),
        PieceKind::Knight => knight_move_possibilities(game, piece, pos),
        PieceKind::Bishop => bishop_move_possibilities(game, piece, pos),
        // PieceKind::King => king_move_possibilities(game, piece, pos),
        // PieceKind::Queen => queen_move_possibilities(game, piece, pos),
        _ => todo!(),
    }
}
