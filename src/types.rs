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

/// Use for SAN parser
/// Ex: Ngf3 -> Knight in g col move to f3
/// col and row is optional, but one of two must be write.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DisambigPosition {
    pub col: Option<u8>,
    pub row: Option<u8>,
}

impl DisambigPosition {
    pub fn new(col: Option<u8>, row: Option<u8>) -> Self {
        if col.is_none() && row.is_none() {
            panic!("You must provide a valid col OR row");
        }
        DisambigPosition { col, row }
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
pub enum GameStatus {
    InProgress,
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}

/// Castling possibilities representation
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

#[derive(Debug, PartialEq, Clone)]
pub enum ActionKind {
    Surrend,
    Move,
    Capture,
    Kingcastling,
    Queencastling,
    Draw,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Action {
    pub kind: ActionKind,
    pub to: Position,
    pub from: Option<DisambigPosition>,
    pub piece_kind: PieceKind,
    pub check: bool,
    pub checkmate: bool,
    pub promotion: Option<PieceKind>,
}

impl Action {
    pub fn new(
        kind: ActionKind,
        to: Position,
        piece_kind: PieceKind,
        from: Option<DisambigPosition>,
        check: bool,
        checkmate: bool,
        promotion: Option<PieceKind>,
    ) -> Self {
        Action {
            to,
            kind,
            piece_kind,
            from,
            check,
            checkmate,
            promotion,
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub history: Vec<Action>,
    pub status: GameStatus,
    pub en_passant_target: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub castling_possibilities: Castling,
}

impl GameState {
    pub fn new(
        board: Board,
        current_player: Option<Color>,
        status: Option<GameStatus>,
        history: Option<Vec<Action>>,
        en_passant_target: Option<Position>,
        halfmove_clock: Option<u8>,
        fullmove_number: Option<u16>,
        castling_possibilities: Option<Castling>,
    ) -> Self {
        let current_player = current_player.unwrap_or_else(|| Color::White);
        let status = status.unwrap_or_else(|| GameStatus::InProgress);
        let history = history.unwrap_or_else(|| Vec::new());
        let halfmove_clock = halfmove_clock.unwrap_or_else(|| 0);
        let fullmove_number = fullmove_number.unwrap_or_else(|| 1);
        let castling_possibilities =
            castling_possibilities.unwrap_or_else(|| Castling::new((true, true), (true, true)));

        GameState {
            board,
            current_player,
            history,
            status,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            castling_possibilities,
        }
    }
}
