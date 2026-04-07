use crate::types::{GameState, Piece, PieceKind, Position};

pub fn move_posibilities(game: &GameState, piece: &Piece, pos: Position) -> Vec<Position> {
    match piece.kind {
        PieceKind::Pawn => pawn_move_posibilities(game, piece, pos),
        PieceKind::Rook => rook_move_posibilities(game, piece, pos),
        PieceKind::Knight => knight_move_posibilities(game, piece, pos),
        PieceKind::Bishop => bishop_move_posibilities(game, piece, pos),
        PieceKind::King => king_move_posibilities(game, piece, pos),
        PieceKind::Queen => queen_move_posibilities(game, piece, pos),
    }
}

pub fn pawn_move_posibilities(_game: &GameState, _piece: &Piece, _pos: Position) -> Vec<Position> {
    vec![Position::new(0, 0)]
}

pub fn rook_move_posibilities(_game: &GameState, _piece: &Piece, _pos: Position) -> Vec<Position> {
    vec![Position::new(0, 0)]
}

pub fn knight_move_posibilities(
    _game: &GameState,
    _piece: &Piece,
    _pos: Position,
) -> Vec<Position> {
    vec![Position::new(0, 0)]
}

pub fn bishop_move_posibilities(
    _game: &GameState,
    _piece: &Piece,
    _pos: Position,
) -> Vec<Position> {
    vec![Position::new(0, 0)]
}

pub fn king_move_posibilities(_game: &GameState, _piece: &Piece, _pos: Position) -> Vec<Position> {
    vec![Position::new(0, 0)]
}

pub fn queen_move_posibilities(_game: &GameState, _piece: &Piece, _pos: Position) -> Vec<Position> {
    vec![Position::new(0, 0)]
}
