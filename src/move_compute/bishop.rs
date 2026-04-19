use crate::types::{GameState, Piece, Position};

pub fn bishop_move_possibilities(game: &GameState, piece: &Piece, pos: &Position) -> Vec<Position> {
    let mut res: Vec<Position> = Vec::new();

    let directions = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

    for (dcol, drow) in directions {
        let (mut col, mut row) = (pos.col as i8, pos.row as i8);
        loop {
            col = col + dcol;
            row = row + drow;

            let is_in_board = 0 <= col && col < 8 && 0 <= row && row < 8;
            if !is_in_board {
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

    res
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
        let wpiece = Piece::new(PieceKind::Bishop, Color::White);
        let bpiece = Piece::new(PieceKind::Bishop, Color::Black);

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("c1").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("f1").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("c8").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("f8").unwrap())
        );
    }

    #[test]
    fn bishop_all_moves() {
        let game = initialize(Some("8/8/2b2b1k/8/8/2B2B1K/8/8 w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Bishop, Color::White);
        let bpiece = Piece::new(PieceKind::Bishop, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("b4").unwrap(),
                str_to_pos("a5").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("f6").unwrap(),
                str_to_pos("d2").unwrap(),
                str_to_pos("e1").unwrap(),
                str_to_pos("b2").unwrap(),
                str_to_pos("a1").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("c3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e4").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("c6").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("h5").unwrap(),
                str_to_pos("g2").unwrap(),
                str_to_pos("h1").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("d1").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("f3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("b7").unwrap(),
                str_to_pos("a8").unwrap(),
                str_to_pos("d7").unwrap(),
                str_to_pos("e8").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("e4").unwrap(),
                str_to_pos("f3").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("a4").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("c6").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e7").unwrap(),
                str_to_pos("d8").unwrap(),
                str_to_pos("g7").unwrap(),
                str_to_pos("h8").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("h4").unwrap(),
                str_to_pos("e5").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("c3").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("f6").unwrap())
        );
    }

    #[test]
    fn bishop_side_moves() {
        let game = initialize(Some("7k/8/b6b/8/8/B6B/8/7K w - - 0 1".to_string()));
        let wpiece = Piece::new(PieceKind::Bishop, Color::White);
        let bpiece = Piece::new(PieceKind::Bishop, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("b4").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("e7").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("b2").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("a3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("g4").unwrap(),
                str_to_pos("f5").unwrap(),
                str_to_pos("e6").unwrap(),
                str_to_pos("d7").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("g2").unwrap(),
                str_to_pos("f1").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("h3").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("b7").unwrap(),
                str_to_pos("c8").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("d3").unwrap(),
                str_to_pos("e2").unwrap(),
                str_to_pos("f1").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("a6").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("g7").unwrap(),
                str_to_pos("f8").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("f4").unwrap(),
                str_to_pos("e3").unwrap(),
                str_to_pos("d2").unwrap(),
                str_to_pos("c1").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("h6").unwrap())
        );
    }

    #[test]
    fn bishop_london_entry_moves() {
        let game = initialize(Some(
            "r1bqk2r/pppn1ppp/1b2pn2/3p4/1P1P1B2/P1N1PN2/2P2PPP/R2QKB1R w KQkq - 1 8".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Bishop, Color::White);
        let bpiece = Piece::new(PieceKind::Bishop, Color::Black);

        assert_eq!(
            vec![
                str_to_pos("e2").unwrap(),
                str_to_pos("d3").unwrap(),
                str_to_pos("c4").unwrap(),
                str_to_pos("b5").unwrap(),
                str_to_pos("a6").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("f1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e5").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("c7").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("g3").unwrap(),
            ],
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("f4").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("c5").unwrap(),
                str_to_pos("d4").unwrap(),
                str_to_pos("a5").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("b6").unwrap())
        );

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("c8").unwrap())
        );
    }

    #[test]
    fn bishop_marshall_moves() {
        let game = initialize(Some(
            "2kr1b2/pp1n3n/2p3p1/3p1bq1/3QpN2/2N2P2/PPPP3P/R1B2R1K b - - 4 16".to_string(),
        ));
        let wpiece = Piece::new(PieceKind::Bishop, Color::White);
        let bpiece = Piece::new(PieceKind::Bishop, Color::Black);

        assert_eq!(
            Vec::<Position>::new(),
            bishop_move_possibilities(&game, &wpiece, &str_to_pos("c1").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e6").unwrap(),
                str_to_pos("g4").unwrap(),
                str_to_pos("h3").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("f5").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("g7").unwrap(),
                str_to_pos("h6").unwrap(),
                str_to_pos("e7").unwrap(),
                str_to_pos("d6").unwrap(),
                str_to_pos("c5").unwrap(),
                str_to_pos("b4").unwrap(),
                str_to_pos("a3").unwrap(),
            ],
            bishop_move_possibilities(&game, &bpiece, &str_to_pos("f8").unwrap())
        );
    }

    #[test]
    fn bishop_squares_moves() {
        let game = initialize(Some(
            "7k/8/P2PP2P/8/2B2B2/8/P2PP2P/7K w - - 0 1".to_string(),
        ));
        let piece = Piece::new(PieceKind::Bishop, Color::White);

        assert_eq!(
            vec![
                str_to_pos("b5").unwrap(),
                str_to_pos("d5").unwrap(),
                str_to_pos("d3").unwrap(),
                str_to_pos("b3").unwrap(),
            ],
            bishop_move_possibilities(&game, &piece, &str_to_pos("c4").unwrap())
        );

        assert_eq!(
            vec![
                str_to_pos("e5").unwrap(),
                str_to_pos("g5").unwrap(),
                str_to_pos("g3").unwrap(),
                str_to_pos("e3").unwrap(),
            ],
            bishop_move_possibilities(&game, &piece, &str_to_pos("f4").unwrap())
        );
    }
}
