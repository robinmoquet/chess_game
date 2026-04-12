use crate::types::{GameState, Piece, Position};

pub fn knight_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    vec![Position::new(0, 0)]
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
    fn white_knight_init_moves() {
        let game = initialize(None);
        let piece = Piece::new(PieceKind::Knight, Color::White);

        assert_eq!(
            vec![str_to_pos("a3").unwrap(), str_to_pos("c3").unwrap()],
            knight_move_possibilities(&game, &piece, &str_to_pos("b1").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("f3").unwrap(), str_to_pos("h3").unwrap()],
            knight_move_possibilities(&game, &piece, &str_to_pos("g1").unwrap())
        );
    }

    #[test]
    fn black_knight_init_moves() {
        let game = initialize(None);
        let piece = Piece::new(PieceKind::Knight, Color::Black);

        assert_eq!(
            vec![str_to_pos("a6").unwrap(), str_to_pos("c6").unwrap()],
            knight_move_possibilities(&game, &piece, &str_to_pos("b8").unwrap())
        );

        assert_eq!(
            vec![str_to_pos("f6").unwrap(), str_to_pos("h6").unwrap()],
            knight_move_possibilities(&game, &piece, &str_to_pos("g8").unwrap())
        );
    }

    #[test]
    fn white_knight_all_moves() {
        let mut game = initialize(Some("7k/8/3n4/8/8/3N4/8/7K w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Knight, Color::White);
        let bpiece = Piece::new(PieceKind::Knight, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("b4").unwrap(),
                str_to_pos("b2").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("c1").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("e1").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("f2").unwrap()
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("d3").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("b7").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("e8").unwrap(),
                str_to_pos("e4").unwrap(),
                str_to_pos("f7").unwrap(),
                str_to_pos("f5").unwrap()
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("d6").unwrap())
        );
    }

    #[test]
    fn white_knight_side_moves() {
        let game = initialize(Some("3k4/8/8/n6n/N6N/8/8/3K4 w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Knight, Color::White);
        let bpiece = Piece::new(PieceKind::Knight, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("b6").unwrap(),
                str_to_pos("b2").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("c3").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("a4").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("g6").unwrap(),
                str_to_pos("g2").unwrap(),
                str_to_pos("f5").unwrap(),
                str_to_pos("f3").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("h4").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("b3").unwrap(),
                str_to_pos("b7").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("c6").unwrap()
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("a5").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("g3").unwrap(),
                str_to_pos("g7").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("f6").unwrap()
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("h5").unwrap())
        );
    }

    #[test]
    fn knight_london_entry_moves() {
        let game = initialize(Some(
            "r1bqk2r/pppn1ppp/1b2pn2/3p4/1P1P1B2/P1N1PN2/2P2PPP/R2QKB1R w KQkq - 1 8".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Knight, Color::White);
        let bpiece = Piece::new(PieceKind::Knight, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("a4").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("e4").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("b1").unwrap(),
                str_to_pos("a2").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("c3").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("e5").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("g1").unwrap(),
                str_to_pos("d2").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("f3").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("g8").unwrap(),
                str_to_pos("h5").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("e4").unwrap(),
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("f6").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("b8").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("c5").unwrap(),
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("d7").unwrap())
        );
    }

    #[test]
    fn knight_marshall_moves() {
        let game = initialize(Some(
            "2kr1b2/pp1n3n/2p3p1/3p1bq1/3QpN2/2N2P2/PPPP3P/R1B2R1K b - - 4 16".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Knight, Color::White);
        let bpiece = Piece::new(PieceKind::Knight, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("a4").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("e4").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("d1").unwrap(),
                str_to_pos("b1").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("c3").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("d5").unwrap(),
                str_to_pos("e6").unwrap(),
                str_to_pos("g6").unwrap(),
                str_to_pos("h5").unwrap(),
                str_to_pos("h3").unwrap(),
                str_to_pos("g2").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("d3").unwrap(),
            ],
            knight_move_possibilities(&game, &wpiece, &str_to_pos("f4").unwrap())
        );
        assert_eq!(
            vec![
                str_to_pos("b8").unwrap(),
                str_to_pos("f6").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("b6").unwrap(),
                str_to_pos("b8").unwrap(),
            ],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("d7").unwrap())
        );
        assert_eq!(
            vec![str_to_pos("f6").unwrap(),],
            knight_move_possibilities(&game, &bpiece, &str_to_pos("h7").unwrap())
        );
    }
}
