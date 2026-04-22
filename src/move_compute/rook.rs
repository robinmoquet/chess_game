use crate::{
    move_compute::utils::filter_moves_in_check,
    types::{GameState, Piece, Position},
    utils::is_in_board,
};

pub fn rook_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    let mut res: Vec<Position> = Vec::new();

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (dcol, drow) in directions {
        let (mut col, mut row) = (pos.col as i8, pos.row as i8);
        loop {
            col = col + dcol;
            row = row + drow;

            if !is_in_board(col, row) {
                break;
            }

            if game.board.squares[row as usize][col as usize]
                .piece
                .is_none()
            {
                res.push(Position::new(col as u8, row as u8));
                continue;
            }

            if game.board.squares[row as usize][col as usize]
                .piece
                .unwrap()
                .color
                != piece.color
            {
                res.push(Position::new(col as u8, row as u8));
            }

            break;
        }
    }

    filter_moves_in_check(game, piece, pos, res)
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
    fn rook_init_moves() {
        let game = initialize(None);
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &wpiece, &str_to_pos("a1").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &wpiece, &str_to_pos("h1").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &bpiece, &str_to_pos("a8").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &bpiece, &str_to_pos("h8").unwrap())
        );
    }

    #[test]
    fn bishop_all_moves() {
        let game = initialize(Some("7k/8/2r2r2/8/8/2R2R2/8/7K w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("c4").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("c6").unwrap(),
                str_to_pos("d3").unwrap(),
                str_to_pos("e3").unwrap(),
                str_to_pos("c2").unwrap(),
                str_to_pos("c1").unwrap(),
                str_to_pos("b3").unwrap(),
                str_to_pos("a3").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("c3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f4").unwrap(),
                str_to_pos("f5").unwrap(),
                str_to_pos("f6").unwrap(),
                str_to_pos("g3").unwrap(),
                str_to_pos("h3").unwrap(),
                str_to_pos("f2").unwrap(),
                str_to_pos("f1").unwrap(),
                str_to_pos("e3").unwrap(),
                str_to_pos("d3").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("f3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("c7").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("e6").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("c3").unwrap(),
                str_to_pos("b6").unwrap(),
                str_to_pos("a6").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("c6").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f7").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("g6").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("f5").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("f3").unwrap(),
                str_to_pos("e6").unwrap(),
                str_to_pos("d6").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("f6").unwrap())
        );
    }

    #[test]
    fn rook_side_moves() {
        let game = initialize(Some("r6r/8/3k4/8/8/3K4/8/R6R w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("a2").unwrap(),
                str_to_pos("a3").unwrap(),
                str_to_pos("a4").unwrap(),
                str_to_pos("a5").unwrap(),
                str_to_pos("a6").unwrap(),
                str_to_pos("a7").unwrap(),
                str_to_pos("a8").unwrap(),
                str_to_pos("b1").unwrap(),
                str_to_pos("c1").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("e1").unwrap(),
                str_to_pos("f1").unwrap(),
                str_to_pos("g1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("a1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("h2").unwrap(),
                str_to_pos("h3").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("h5").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("h7").unwrap(),
                str_to_pos("h8").unwrap(),
                str_to_pos("g1").unwrap(),
                str_to_pos("f1").unwrap(),
                str_to_pos("e1").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("c1").unwrap(),
                str_to_pos("b1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("h1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("b8").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("e8").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("g8").unwrap(),
                str_to_pos("a7").unwrap(),
                str_to_pos("a6").unwrap(),
                str_to_pos("a5").unwrap(),
                str_to_pos("a4").unwrap(),
                str_to_pos("a3").unwrap(),
                str_to_pos("a2").unwrap(),
                str_to_pos("a1").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("a8").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("h7").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("h5").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("h3").unwrap(),
                str_to_pos("h2").unwrap(),
                str_to_pos("h1").unwrap(),
                str_to_pos("g8").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("e8").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("b8").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("h8").unwrap())
        );
    }

    #[test]
    fn rook_london_entry_moves() {
        let game = initialize(Some(
            "r1bqk2r/pppn1ppp/1b2pn2/3p4/1P1P1B2/P1N1PN2/2P2PPP/R2QKB1R w KQkq - 1 8".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("a2").unwrap(),
                str_to_pos("b1").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("a1").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("g1").unwrap(),],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("h1").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("b8").unwrap(),],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("a8").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("g8").unwrap(), str_to_pos("f8").unwrap(),],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("h8").unwrap())
        );
    }

    #[test]
    fn rook_marshall_moves() {
        let game = initialize(Some(
            "2kr1b2/pp1n3n/2p3p1/3p1bq1/3QpN2/2N2P2/PPPP3P/R1B2R1K b - - 4 16".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            vec![str_to_pos("b1").unwrap(),],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("a1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("f2").unwrap(),
                str_to_pos("g1").unwrap(),
                str_to_pos("e1").unwrap(),
                str_to_pos("d1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("f1").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("e8").unwrap(),],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("d8").unwrap())
        );
    }

    #[test]
    fn rook_fisher_moves() {
        let game = initialize(Some(
            "3r1k2/1p3pp1/p1p2b2/2Pr3p/PPRPR3/6P1/1B3KP1/8 w - - 0 1".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("c3").unwrap(),
                str_to_pos("c2").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("c4").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e5").unwrap(),
                str_to_pos("e6").unwrap(),
                str_to_pos("e7").unwrap(),
                str_to_pos("e8").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("e3").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("e1").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("e4").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("d6").unwrap(),
                str_to_pos("d7").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("f5").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("c5").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("d5").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e8").unwrap(),
                str_to_pos("d7").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("b8").unwrap(),
                str_to_pos("a8").unwrap(),
            ],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("d8").unwrap())
        );
    }

    #[test]
    fn white_rook_prevent_illegal_moves() {
        let game = initialize(Some("8/3k4/3r4/b2R4/8/2R5/3K1R1q/8 w - - 1 1".to_string()));
        let wpiece = Piece::new(PieceKind::Rook, Color::White);
        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &wpiece, &str_to_pos("c3").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("d6").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("d3").unwrap()
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("d5").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("g2").unwrap(),
                str_to_pos("h2").unwrap(),
                str_to_pos("e2").unwrap(),
            ],
            rook_move_possibilities(&game, &wpiece, &str_to_pos("f2").unwrap())
        );
    }

    #[test]
    fn black_rook_prevent_illegal_moves() {
        let game = initialize(Some("8/3krQ2/2r5/3r4/B2R4/8/3K4/8 b - - 1 1".to_string()));
        let bpiece = Piece::new(PieceKind::Rook, Color::Black);
        assert_eq!(
            Vec::<Position>::new(),
            rook_move_possibilities(&game, &bpiece, &str_to_pos("c6").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("f7").unwrap(),],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("e7").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("d6").unwrap(), str_to_pos("d4").unwrap(),],
            rook_move_possibilities(&game, &bpiece, &str_to_pos("d5").unwrap())
        );
    }
}
