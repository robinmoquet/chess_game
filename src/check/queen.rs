use crate::{
    check::{bishop::is_check_bishop, rook::is_check_rook},
    types::{Color, Position, Squares},
};

// That can be optimized by checking Queen check in bishop and rook check functions, but I think it's not a big deal for now
pub fn is_check_queen(king_pos: &Position, color: &Color, squares: &Squares) -> bool {
    is_check_bishop(king_pos, color, squares) || is_check_rook(king_pos, color, squares)
}
