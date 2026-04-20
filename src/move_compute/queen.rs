use crate::{
    move_compute::{bishop::bishop_move_possibilities, rook::rook_move_possibilities},
    types::{GameState, Piece, PieceKind, Position},
};

pub fn queen_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    let mut r = bishop_move_possibilities(game, &Piece::new(PieceKind::Bishop, piece.color), pos);
    r.extend(rook_move_possibilities(
        game,
        &Piece::new(PieceKind::Rook, piece.color),
        pos,
    ));
    r
}
