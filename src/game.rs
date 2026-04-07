use dialoguer::{Confirm, Input};
use regex::Regex;

use crate::{
    fen,
    move_posibilities::move_posibilities,
    printer,
    types::{Board, Color, GameState, GameStatus, Move, Piece, PieceKind, Position, Square},
    utils::{char_to_piece, str_to_castling, str_to_pos},
};

pub fn initialize() -> GameState {
    let fen = fen::parse(String::from(
        // "1B6/2n5/p1N1P2R/P1K3N1/4Pk2/1Q2p2p/6nP/1B4R1 w - - 0 1",
        // "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ))
    .unwrap();

    let board = Board::new(fen.squares);
    GameState::new(
        board,
        Some(fen.active_color),
        None,
        None,
        fen.en_passant_target,
        Some(fen.halfmove_clock),
        Some(fen.fullmove_number),
        Some(str_to_castling(
            &fen.castling_possibilities
                .unwrap_or_else(|| String::from("-")),
        )),
    )
}

pub fn play(mut game: GameState) {
    loop {
        clear();
        print(&game);
        let mut action = String::new();
        while !(valid_action(&action)) {
            action = ask_action();
            println!(
                "Move invalid : {}, (ex: e4, g6, Nf3, surrend, draw)",
                action
            );
        }
        game = do_action(action, game);
        break;

        if let GameStatus::Checkmate(_) = game.status {
            // end();
            break;
        }
    }
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
}

pub fn ask_action() -> String {
    Input::new()
        .with_prompt("Next move ?")
        .interact_text()
        .unwrap()
}

pub fn valid_action(action: &String) -> bool {
    if action == "surrend" {
        return true;
    }

    Regex::new(r"^([a-h][1-8])|([RNBQK][a-h][1-8])$")
        .unwrap()
        .is_match(action)
}

pub fn do_action(action: String, game: GameState) -> GameState {
    if action == "surrend" && confirm("Do you really want to give up ?") {
        return surrend(game);
    }

    do_move(&action, game)
}

pub fn surrend(mut game: GameState) -> GameState {
    game.status = GameStatus::Checkmate(game.current_player);
    game
}

pub fn confirm(prompt: &str) -> bool {
    Confirm::new().with_prompt(prompt).interact().unwrap()
}

pub fn do_move(action: &String, mut game: GameState) -> GameState {
    let piece_str = if action.len() == 3 {
        action.clone().chars().nth(0).unwrap()
    } else {
        'p'
    };
    let piece_str = if game.current_player == Color::White {
        piece_str.to_ascii_uppercase()
    } else {
        piece_str
    };
    let piece: Piece = char_to_piece(piece_str);
    let to_str: &String = if action.len() == 3 {
        &action[1..].to_string()
    } else {
        action
    };
    let to = str_to_pos(to_str).unwrap();
    let from = get_current_pos(&game, &piece, &to);

    game.board.squares[from.row as usize][from.col as usize] = Square::new(None);
    game.board.squares[to.row as usize][to.col as usize] = Square::new(Some(piece));
    game.moves_history.push(Move::new(piece, from, to));

    println!("{:?}", piece);
    println!("{:?}", to);

    game
}

/// We must provide a destination (to: &Position) for identify Bishop, Knight, Rook
pub fn get_current_pos(game: &GameState, piece: &Piece, to: &Position) -> Position {
    let mut posibilities: Vec<Position> = Vec::new();
    for (r, row) in game.board.squares.iter().enumerate() {
        for (c, square) in row.iter().enumerate() {
            if square.piece == Some(*piece) {
                posibilities.push(Position::new(c as u8, r as u8));
            }
        }
    }

    println!("{:?}", posibilities);

    for posibility in posibilities {
        let moves = move_posibilities(game, piece, posibility);
        let find = moves.iter().find(|m| *m == to);
        if find.is_some() {
            return posibility;
        }
    }

    panic!("No initial position find");
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
