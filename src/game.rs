use dialoguer::Input;

use crate::{
    fen, printer,
    types::{Board, Color, GameState, Move, Piece, PieceKind, Position},
    utils::str_to_castling,
};

pub fn initialize() -> GameState {
    let fen = fen::parse(String::from(
        // "1B6/2n5/p1N1P2R/P1K3N1/4Pk2/1Q2p2p/6nP/1B4R1 w - - 0 1",
        "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1",
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ))
    .unwrap();

    let board = Board::new(fen.squares);
    GameState::new(
        board,
        Some(fen.active_color),
        None,
        Some(vec![
            Move::new(
                Piece::new(PieceKind::Pawn, Color::White),
                Position::new(4, 6),
                Position::new(4, 4),
            ),
            Move::new(
                Piece::new(PieceKind::Pawn, Color::Black),
                Position::new(4, 1),
                Position::new(4, 3),
            ),
            Move::new(
                Piece::new(PieceKind::Knight, Color::White),
                Position::new(4, 7),
                Position::new(5, 5),
            ),
            Move::new(
                Piece::new(PieceKind::Knight, Color::Black),
                Position::new(1, 0),
                Position::new(2, 2),
            ),
        ]),
        fen.en_passant_target,
        Some(fen.halfmove_clock),
        Some(fen.fullmove_number),
        Some(str_to_castling(
            &fen.castling_possibilities
                .unwrap_or_else(|| String::from("-")),
        )),
    )
}

pub fn print(game: &GameState) {
    let color = if game.current_player == Color::White {
        "White"
    } else {
        "Black"
    };
    println!("Chess Game : {} to move\n", color);
    printer::print_board(game);
    println!("{}", printer::print_fen(game));
    println!("{}\n", printer::print_pgn_content(game));
    let action = ask_action();
    println!("{}", action);
}

pub fn ask_action() -> String {
    Input::new()
        .with_prompt("Next move ?")
        .interact_text()
        .unwrap()
}

pub fn clear() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Color, GameStatus, Move, Square, Squares};

    #[test]
    fn default_initialize() {
        let game = initialize();
        let squares: Squares = [
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::King, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::Black))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::Black))),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
                Square::new(None),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Pawn, Color::White))),
            ],
            [
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Queen, Color::White))),
                Square::new(Some(Piece::new(PieceKind::King, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Bishop, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Knight, Color::White))),
                Square::new(Some(Piece::new(PieceKind::Rook, Color::White))),
            ],
        ];

        assert_eq!(game.current_player, Color::White);
        assert_eq!(game.moves_history, Vec::<Move>::new());
        assert_eq!(game.status, GameStatus::InProgress);
        assert_eq!(game.board, Board::new(squares));
    }
}
