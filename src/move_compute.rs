mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod utils;

use crate::{
    move_compute::{
        bishop::bishop_move_possibilities, king::king_move_possibilities,
        knight::knight_move_possibilities, pawn::pawn_move_possibilities,
        queen::queen_move_possibilities, rook::rook_move_possibilities,
    },
    types::{Color, GameState, Piece, PieceKind, Position},
};

pub use utils::{backward_one_square, forward_one_square};

pub fn move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    match piece.kind {
        PieceKind::Pawn => pawn_move_possibilities(game, piece, pos),
        PieceKind::Rook => rook_move_possibilities(game, piece, pos),
        PieceKind::Knight => knight_move_possibilities(game, piece, pos),
        PieceKind::Bishop => bishop_move_possibilities(game, piece, pos),
        PieceKind::King => king_move_possibilities(game, piece, pos),
        PieceKind::Queen => queen_move_possibilities(game, piece, pos),
    }
}

pub fn all_move_possibilities(game: &GameState, color: &Color) -> Vec<(Piece, Vec<Position>)> {
    let mut res: Vec<(Piece, Vec<Position>)> = Vec::new();
    let mut pieces: Vec<(Piece, Position)> = Vec::new();

    for (r, row) in game.board.squares.iter().enumerate() {
        for (c, square) in row.iter().enumerate() {
            if let Some(piece) = square.piece {
                if piece.color == *color {
                    pieces.push((piece, Position::new(c as u8, r as u8)));
                }
            }
        }
    }

    for (piece, pos) in pieces {
        res.push((piece, move_possibilities(game, &piece, &pos)));
    }

    res
}
