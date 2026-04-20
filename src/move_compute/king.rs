use crate::types::{Color, GameState, Piece, Position};

pub fn king_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    let mut res = Vec::new();
    let possibilities: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for (col, row) in possibilities {
        let (col, row) = (pos.col as i8 + col, pos.row as i8 + row);

        let is_in_board = 0 <= col && col < 8 && 0 <= row && row < 8;
        if is_in_board
            && (game.board.squares[row as usize][col as usize]
                .piece
                .is_none()
                || game.board.squares[row as usize][col as usize]
                    .piece
                    .unwrap()
                    .color
                    != piece.color)
        {
            res.push(Position::new(col as u8, row as u8));
        }
    }

    if piece.color == Color::White && white_kingside_castling(game) {
        res.push(Position::new(6, 7));
    }
    if piece.color == Color::White && white_queenside_castling(game) {
        res.push(Position::new(2, 7));
    }
    if piece.color == Color::Black && black_kingside_castling(game) {
        res.push(Position::new(6, 0));
    }
    if piece.color == Color::Black && black_queenside_castling(game) {
        res.push(Position::new(2, 0));
    }

    res
}

fn black_kingside_castling(game: &GameState) -> bool {
    if !game.castling_possibilities.kingside.1 {
        return false;
    }
    game.board.squares[0][5].piece.is_none() && game.board.squares[0][6].piece.is_none()
}

fn white_kingside_castling(game: &GameState) -> bool {
    if !game.castling_possibilities.kingside.0 {
        return false;
    }

    game.board.squares[7][5].piece.is_none() && game.board.squares[7][6].piece.is_none()
}

fn black_queenside_castling(game: &GameState) -> bool {
    if !game.castling_possibilities.queenside.1 {
        return false;
    }
    game.board.squares[0][1].piece.is_none()
        && game.board.squares[0][2].piece.is_none()
        && game.board.squares[0][3].piece.is_none()
}

fn white_queenside_castling(game: &GameState) -> bool {
    if !game.castling_possibilities.queenside.0 {
        return false;
    }
    game.board.squares[7][1].piece.is_none()
        && game.board.squares[7][2].piece.is_none()
        && game.board.squares[7][3].piece.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initialize,
        types::{Color, PieceKind},
        utils::str_to_pos,
    };

    #[test]
    fn bishop_init_moves() {
        let game = initialize(None);
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            Vec::<Position>::new(),
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }

    #[test]
    fn king_all_moves() {
        let game = initialize(Some("8/8/8/2k2K2/8/8/8/8 w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("b6").unwrap(),
                str_to_pos("c6").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("b4").unwrap(),
                str_to_pos("b5").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("c5").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e6").unwrap(),
                str_to_pos("f6").unwrap(),
                str_to_pos("g6").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("e4").unwrap(),
                str_to_pos("e5").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("f5").unwrap())
        );
    }

    #[test]
    fn king_side_moves() {
        let game = initialize(Some("8/8/8/k6K/8/8/8/8 w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("g6").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("g5").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("h5").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("a6").unwrap(),
                str_to_pos("b6").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("b4").unwrap(),
                str_to_pos("a4").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("a5").unwrap())
        );
    }

    #[test]
    fn king_capture_moves() {
        let game = initialize(Some("8/1R6/2k5/2Pn4/8/5KN1/4np2/8 w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("e4").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("g2").unwrap(),
                str_to_pos("f2").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("e3").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("f3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("b7").unwrap(),
                str_to_pos("c7").unwrap(),
                str_to_pos("d7").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("b6").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("c6").unwrap())
        );
    }

    #[test]
    fn king_castling_moves() {
        let game = initialize(Some(
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 1 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("f1").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("g1").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("g8").unwrap(),
                str_to_pos("c8").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }

    #[test]
    fn king_no_castling_moves() {
        let game = initialize(Some(
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w - - 1 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![str_to_pos("f1").unwrap(), str_to_pos("d1").unwrap(),],
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("f8").unwrap(), str_to_pos("d8").unwrap(),],
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }

    #[test]
    fn king_king_castling_moves() {
        let game = initialize(Some(
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w Kk - 1 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("f1").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("g1").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("g8").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }

    #[test]
    fn king_queen_castling_moves() {
        let game = initialize(Some(
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w Qq - 1 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("f1").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("c8").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }

    #[test]
    fn king_part_castling_moves() {
        let game = initialize(Some(
            "rn2k2r/pppppppp/8/8/8/8/PPPPPPPP/R3KB1R w KQkq - 1 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::King, Color::White);
        let bpiece = Piece::new(PieceKind::King, Color::Black);

        assert_eq!(
            vec![str_to_pos("d1").unwrap(), str_to_pos("c1").unwrap(),],
            king_move_possibilities(&game, &wpiece, &str_to_pos("e1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("g8").unwrap(),
            ],
            king_move_possibilities(&game, &bpiece, &str_to_pos("e8").unwrap())
        );
    }
}
