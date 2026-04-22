use crate::{
    check::is_check,
    game::update_squares,
    types::{Color, GameState, Piece, Position},
};

pub fn forward_one_square(pos: &Position, color: &Color) -> Position {
    if *color == Color::White {
        return Position::new(pos.col, pos.row - 1);
    } else {
        return Position::new(pos.col, pos.row + 1);
    }
}

pub fn backward_one_square(pos: &Position, color: &Color) -> Position {
    if *color == Color::White {
        return Position::new(pos.col, pos.row + 1);
    } else {
        return Position::new(pos.col, pos.row - 1);
    }
}

pub fn forward_left_one_square(pos: &Position, color: &Color) -> Position {
    if *color == Color::White {
        return Position::new(pos.col - 1, pos.row - 1);
    } else {
        return Position::new(pos.col + 1, pos.row + 1);
    }
}

pub fn forward_right_one_square(pos: &Position, color: &Color) -> Position {
    if *color == Color::White {
        return Position::new(pos.col + 1, pos.row - 1);
    } else {
        return Position::new(pos.col - 1, pos.row + 1);
    }
}

pub fn move_delta(a: &Position, b: &Position) -> u8 {
    let mut res: u8 = 0;

    if a.col < b.col {
        res += b.col - a.col
    } else {
        res += a.col - b.col
    }

    if a.row < b.row {
        res += b.row - a.row
    } else {
        res += a.row - b.row
    }

    res
}

pub fn filter_moves_in_check(
    game: &GameState,
    piece: &Piece,
    from: &Position,
    moves: Vec<Position>,
) -> Vec<Position> {
    moves
        .into_iter()
        .filter(|pos| {
            // Simulate the move and check if the king is in check
            let mut squares = game.board.squares.clone();
            update_squares(&mut squares, from, pos, piece, &None);

            !is_check(&squares, &piece.color)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::str_to_pos;

    use super::*;

    #[test]
    fn move_forward() {
        assert_eq!(
            str_to_pos("e4").unwrap(),
            forward_one_square(&str_to_pos("e3").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("e2").unwrap(),
            forward_one_square(&str_to_pos("e3").unwrap(), &Color::Black)
        );
        assert_eq!(
            str_to_pos("a7").unwrap(),
            forward_one_square(&str_to_pos("a6").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("a5").unwrap(),
            forward_one_square(&str_to_pos("a6").unwrap(), &Color::Black)
        );
    }

    #[test]
    fn move_backward() {
        assert_eq!(
            str_to_pos("e2").unwrap(),
            backward_one_square(&str_to_pos("e3").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("e4").unwrap(),
            backward_one_square(&str_to_pos("e3").unwrap(), &Color::Black)
        );
        assert_eq!(
            str_to_pos("a5").unwrap(),
            backward_one_square(&str_to_pos("a6").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("a7").unwrap(),
            backward_one_square(&str_to_pos("a6").unwrap(), &Color::Black)
        );
    }

    #[test]
    fn move_forward_left() {
        assert_eq!(
            str_to_pos("d4").unwrap(),
            forward_left_one_square(&str_to_pos("e3").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("f2").unwrap(),
            forward_left_one_square(&str_to_pos("e3").unwrap(), &Color::Black)
        );
        assert_eq!(
            str_to_pos("a7").unwrap(),
            forward_left_one_square(&str_to_pos("b6").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("b5").unwrap(),
            forward_left_one_square(&str_to_pos("a6").unwrap(), &Color::Black)
        );
    }

    #[test]
    fn move_forward_right() {
        assert_eq!(
            str_to_pos("f4").unwrap(),
            forward_right_one_square(&str_to_pos("e3").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("d2").unwrap(),
            forward_right_one_square(&str_to_pos("e3").unwrap(), &Color::Black)
        );
        assert_eq!(
            str_to_pos("c7").unwrap(),
            forward_right_one_square(&str_to_pos("b6").unwrap(), &Color::White)
        );
        assert_eq!(
            str_to_pos("a5").unwrap(),
            forward_right_one_square(&str_to_pos("b6").unwrap(), &Color::Black)
        );
    }

    #[test]
    fn valid_move_delta() {
        assert_eq!(
            2,
            move_delta(&str_to_pos("e2").unwrap(), &str_to_pos("e4").unwrap())
        );
        assert_eq!(
            1,
            move_delta(&str_to_pos("e3").unwrap(), &str_to_pos("e4").unwrap())
        );
        assert_eq!(
            4,
            move_delta(&str_to_pos("a3").unwrap(), &str_to_pos("c5").unwrap())
        );
        assert_eq!(
            12,
            move_delta(&str_to_pos("a2").unwrap(), &str_to_pos("h7").unwrap())
        );
    }
}
