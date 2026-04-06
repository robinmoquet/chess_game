use crate::types::{Color, Position, Squares};

#[derive(Debug)]
pub struct FEN {
    pub squares: Squares,
    pub active_color: Color,
    pub castling_possibilities: Option<String>,
    pub en_passant_target: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}
