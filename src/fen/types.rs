use crate::types::{Color, Position};

#[derive(Debug)]
pub struct FEN {
    // pub pieces: Squares,
    pub active_color: Color,
    pub castling_possibility: Option<String>,
    pub en_passant_target: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}
