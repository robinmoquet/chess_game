use crate::{
    move_compute::utils::{
        filter_moves_in_check, forward_left_one_square, forward_one_square,
        forward_right_one_square,
    },
    types::{Color, GameState, Piece, Position},
    utils::is_empty_square,
};

pub fn pawn_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    let mut res: Vec<Position> = Vec::new();
    let color = piece.color;
    let is_start_pos = if color == Color::White {
        pos.row == 6
    } else {
        pos.row == 1
    };
    let is_end_pos = if color == Color::White {
        pos.row == 0
    } else {
        pos.row == 7
    };
    let squares = game.board.squares;

    if is_end_pos {
        return res;
    }

    let front_square = forward_one_square(pos, &color);
    if is_empty_square(&squares, &front_square) {
        res.push(front_square);
    }

    let second_square = forward_one_square(&front_square, &color);
    if is_start_pos
        && is_empty_square(&squares, &front_square)
        && is_empty_square(&squares, &second_square)
    {
        res.push(second_square);
    }

    let is_col_h = pos.col == 7;
    let is_col_a = pos.col == 0;

    if (!is_col_a && color == Color::White) || (!is_col_h && color == Color::Black) {
        let left_side_square = forward_left_one_square(pos, &color);
        let piece = squares[left_side_square.row as usize][left_side_square.col as usize].piece;
        if (piece.is_some() && piece.unwrap().color != color)
            || game.en_passant_target == Some(left_side_square)
        {
            res.push(left_side_square);
        }
    }

    if (!is_col_h && color == Color::White) || (!is_col_a && color == Color::Black) {
        let right_side_square = forward_right_one_square(pos, &color);
        let piece = squares[right_side_square.row as usize][right_side_square.col as usize].piece;
        if (piece.is_some() && piece.unwrap().color != color)
            || game.en_passant_target == Some(right_side_square)
        {
            res.push(right_side_square);
        }
    }

    filter_moves_in_check(game, piece, pos, res)
}

#[cfg(test)]
mod tests {
    use crate::{
        initialize,
        types::{Color, PieceKind, Square},
        utils::str_to_pos,
    };

    use super::*;

    #[test]
    fn white_pawn_default_moves() {
        let mut game = initialize(None);
        let piece = Piece::new(PieceKind::Pawn, Color::White);

        // Test default move
        assert_eq!(
            vec![str_to_pos("a3").unwrap(), str_to_pos("a4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a2").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("h3").unwrap(), str_to_pos("h4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("h2").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("e4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e3").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("e3").unwrap(), str_to_pos("e4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e2").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("e8").unwrap())
        );

        let d7 = str_to_pos("d7").unwrap();
        let d5 = str_to_pos("d5").unwrap();
        let e7 = str_to_pos("e7").unwrap();
        let e5 = str_to_pos("e5").unwrap();
        let f7 = str_to_pos("f7").unwrap();
        let f5 = str_to_pos("f5").unwrap();
        game.board.squares[e7.row as usize][e7.col as usize] = Square::new(None);
        game.board.squares[e5.row as usize][e5.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black)));

        assert_eq!(
            vec![str_to_pos("e4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e3").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("e4").unwrap())
        );

        game.board.squares[d7.row as usize][d7.col as usize] = Square::new(None);
        game.board.squares[d5.row as usize][d5.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black)));

        assert_eq!(
            vec![str_to_pos("d5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e4").unwrap())
        );

        game.board.squares[f7.row as usize][f7.col as usize] = Square::new(None);
        game.board.squares[f5.row as usize][f5.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black)));

        assert_eq!(
            vec![str_to_pos("d5").unwrap(), str_to_pos("f5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e4").unwrap())
        );

        game.board.squares[e5.row as usize][e5.col as usize] = Square::new(None);
        game.board.squares[e7.row as usize][e7.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black)));

        assert_eq!(
            vec![
                str_to_pos("e5").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("f5").unwrap(),
            ],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e4").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("a5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a4").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("h5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("h4").unwrap())
        );
    }

    #[test]
    fn black_pawn_default_moves() {
        let mut game = initialize(None);
        let piece = Piece::new(PieceKind::Pawn, Color::Black);

        // Test default move
        assert_eq!(
            vec![str_to_pos("a6").unwrap(), str_to_pos("a5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a7").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("h6").unwrap(), str_to_pos("h5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("h7").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("e5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e6").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("e6").unwrap(), str_to_pos("e5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e7").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("e1").unwrap())
        );

        let d2 = str_to_pos("d2").unwrap();
        let d4 = str_to_pos("d4").unwrap();
        let e2 = str_to_pos("e2").unwrap();
        let e4 = str_to_pos("e4").unwrap();
        let f2 = str_to_pos("f2").unwrap();
        let f4 = str_to_pos("f4").unwrap();
        game.board.squares[e2.row as usize][e2.col as usize] = Square::new(None);
        game.board.squares[e4.row as usize][e4.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!(
            vec![str_to_pos("e5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e6").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("e5").unwrap())
        );

        game.board.squares[d2.row as usize][d2.col as usize] = Square::new(None);
        game.board.squares[d4.row as usize][d4.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!(
            vec![str_to_pos("d4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e5").unwrap())
        );

        game.board.squares[f2.row as usize][f2.col as usize] = Square::new(None);
        game.board.squares[f4.row as usize][f4.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!(
            vec![str_to_pos("f4").unwrap(), str_to_pos("d4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e5").unwrap())
        );

        game.board.squares[e4.row as usize][e4.col as usize] = Square::new(None);
        game.board.squares[e2.row as usize][e2.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!(
            vec![
                str_to_pos("e4").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("d4").unwrap(),
            ],
            pawn_move_possibilities(&game, &piece, &str_to_pos("e5").unwrap())
        );
    }

    #[test]
    fn white_pawn_en_passant_move() {
        let mut game = initialize(None);
        let piece = Piece::new(PieceKind::Pawn, Color::White);

        assert_eq!(
            vec![str_to_pos("a6").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a5").unwrap())
        );

        let b7 = str_to_pos("b7").unwrap();
        let b5 = str_to_pos("b5").unwrap();
        game.board.squares[b7.row as usize][b7.col as usize] = Square::new(None);
        game.board.squares[b5.row as usize][b5.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black)));

        assert_eq!(
            vec![str_to_pos("a6").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a5").unwrap())
        );

        game.en_passant_target = Some(str_to_pos("b6").unwrap());

        assert_eq!(
            vec![str_to_pos("a6").unwrap(), str_to_pos("b6").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("a5").unwrap())
        );
    }

    #[test]
    fn black_pawn_en_passant_move() {
        let mut game = initialize(None);
        let piece = Piece::new(PieceKind::Pawn, Color::Black);

        assert_eq!(
            vec![str_to_pos("d3").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("d4").unwrap())
        );

        let c2 = str_to_pos("c2").unwrap();
        let c4 = str_to_pos("c4").unwrap();
        game.board.squares[c2.row as usize][c2.col as usize] = Square::new(None);
        game.board.squares[c4.row as usize][c4.col as usize] =
            Square::new(Some(Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!(
            vec![str_to_pos("d3").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("d4").unwrap())
        );

        game.en_passant_target = Some(str_to_pos("c3").unwrap());

        assert_eq!(
            vec![str_to_pos("d3").unwrap(), str_to_pos("c3").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("d4").unwrap())
        );
    }

    #[test]
    fn black_pawn_prevent_illegal_moves() {
        let game = initialize(Some(
            "8/3k1p1R/2p1p3/3p4/Q3P1B1/8/8/3R3K b - - 0 1".to_string(),
        ));
        let piece = Piece::new(PieceKind::Pawn, Color::Black);

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("c6").unwrap())
        );
        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("e6").unwrap())
        );
        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("f7").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("d4").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("d5").unwrap())
        );
    }

    #[test]
    fn white_pawn_prevent_illegal_moves() {
        let game = initialize(Some("3q4/8/b7/4b3/1pPPPP2/r1PK4/8/8 w - - 0 1".to_string()));
        let piece = Piece::new(PieceKind::Pawn, Color::White);

        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("c3").unwrap())
        );
        assert_eq!(
            Vec::<Position>::new(),
            pawn_move_possibilities(&game, &piece, &str_to_pos("c4").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("d5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("d4").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("f5").unwrap(), str_to_pos("e5").unwrap()],
            pawn_move_possibilities(&game, &piece, &str_to_pos("f4").unwrap())
        );
    }
}
