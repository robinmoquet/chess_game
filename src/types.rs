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

#[derive(Debug, PartialEq)]
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
    pub from: Position,
    pub to: Position,
    // pub promotion: Option<PieceKind>,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    InProgress,
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub moves_history: Vec<Move>,
    pub status: GameStatus,
}

impl GameState {
    pub fn new(
        board: Board,
        current_player: Option<Color>,
        status: Option<GameStatus>,
        moves_history: Option<Vec<Move>>,
    ) -> Self {
        let current_player = current_player.unwrap_or_else(|| Color::White);
        let status = status.unwrap_or_else(|| GameStatus::InProgress);
        let moves_history = moves_history.unwrap_or_else(|| Vec::new());

        GameState {
            board,
            current_player,
            moves_history,
            status,
        }
    }
}
