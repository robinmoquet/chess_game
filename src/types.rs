#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub col: u8,
    pub row: u8,
}

impl Position {
    pub fn new(col: u8, row: u8) -> Self {
        Position { col, row }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Piece { kind, color }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Square { piece }
    }
}

pub type Squares = [[Square; 8]; 8];

#[derive(Debug, PartialEq)]
pub struct Board {
    pub squares: Squares,
}

impl Board {
    pub fn new(squares: Squares) -> Self {
        Board { squares }
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    // pub promotion: Option<PieceKind>,
}

impl Move {
    pub fn new(piece: Piece, from: Position, to: Position) -> Self {
        Move { piece, from, to }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    InProgress,
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}

/// Castling posibilities representation
/// First is for white and second for black
///
/// Note : This representation is only for track Rook or King move, not if King is in Check or else
///
/// Exemple :
/// kingside: (true, false) -> white can kingside castling and black cannot
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Castling {
    pub kingside: (bool, bool),
    pub queenside: (bool, bool),
}

impl Castling {
    pub fn new(kingside: (bool, bool), queenside: (bool, bool)) -> Self {
        Castling {
            kingside,
            queenside,
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub moves_history: Vec<Move>,
    pub status: GameStatus,
    pub en_passant_target: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub castling_posibilities: Castling,
}

impl GameState {
    pub fn new(
        board: Board,
        current_player: Option<Color>,
        status: Option<GameStatus>,
        moves_history: Option<Vec<Move>>,
        en_passant_target: Option<Position>,
        halfmove_clock: Option<u8>,
        fullmove_number: Option<u16>,
        castling_posibilities: Option<Castling>,
    ) -> Self {
        let current_player = current_player.unwrap_or_else(|| Color::White);
        let status = status.unwrap_or_else(|| GameStatus::InProgress);
        let moves_history = moves_history.unwrap_or_else(|| Vec::new());
        let halfmove_clock = halfmove_clock.unwrap_or_else(|| 0);
        let fullmove_number = fullmove_number.unwrap_or_else(|| 1);
        let castling_posibilities =
            castling_posibilities.unwrap_or_else(|| Castling::new((true, true), (true, true)));

        GameState {
            board,
            current_player,
            moves_history,
            status,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            castling_posibilities,
        }
    }
}
