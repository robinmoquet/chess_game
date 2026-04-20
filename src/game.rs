use dialoguer::{Confirm, Input};

use crate::{
    check::is_check,
    errors::ActionError,
    fen,
    move_compute::{backward_one_square, forward_one_square, move_delta, move_possibilities},
    printer, san,
    types::{
        Action, ActionKind, Board, Color, DisambigPosition, GameState, GameStatus, Piece,
        PieceKind, Position, Square,
    },
    utils::{is_empty_square, str_to_castling},
};

pub fn initialize(init_fen: Option<String>) -> GameState {
    let fen = fen::parse(
        init_fen.unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
    )
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
        loop {
            let action = ask_action();
            let (res, current_game) = do_action(action, game);
            game = current_game;
            match res {
                Ok(_) => break,
                Err(e) => {
                    println!("{}", e.message)
                }
            }
        }

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

pub fn do_action(action: String, game: GameState) -> (Result<(), ActionError>, GameState) {
    let action = san::parse(&action, Some(game.current_player));
    if let Err(e) = action {
        return (Err(e), game);
    }
    let action = action.unwrap();
    if action.kind == ActionKind::Surrend && confirm("Do you really want to give up ?") {
        return (Ok(()), surrend(game));
    }

    do_move(action, game)
}

pub fn surrend(mut game: GameState) -> GameState {
    game.status = GameStatus::Checkmate(game.current_player);
    game
}

pub fn confirm(prompt: &str) -> bool {
    Confirm::new().with_prompt(prompt).interact().unwrap()
}

pub fn do_move(mut action: Action, mut game: GameState) -> (Result<(), ActionError>, GameState) {
    let piece = Piece::new(action.piece_kind, game.current_player);
    let to = action.to;
    let from = get_current_pos(&game, &action);
    if let Err(e) = from {
        return (Err(e), game);
    }
    let from = from.unwrap();

    if action.kind != ActionKind::Capture && is_capture(&game, &to, &piece) {
        action.kind = ActionKind::Capture;
        if piece.kind == PieceKind::Pawn {
            action.from = Some(DisambigPosition::new(Some(from.col), None));
        }
    }

    game.board.squares[from.row as usize][from.col as usize] = Square::new(None);
    game.board.squares[to.row as usize][to.col as usize] = Square::new(Some(piece));
    manage_special_moves(&piece, &mut game, &action);

    if piece.kind == PieceKind::Pawn && move_delta(&from, &to) == 2 && from.col == to.col {
        game.en_passant_target = Some(forward_one_square(&from, &piece.color));
    } else {
        game.en_passant_target = None;
    }

    action.check = is_check(&game, &game.current_player);
    game.history.push(action);
    game.current_player = if game.current_player == Color::White {
        Color::Black
    } else {
        Color::White
    };
    (Ok(()), game)
}

/// The goal of this function is to provide the current position of the
/// piece the user wants to move.
///
/// For example, if the user provides "e4", they are implicitly saying
/// "I want to move my pawn from e2 (or e3) to e4".
/// "get_current_pos" must find all pieces of the same type as the "piece"
/// parameter and find which one can move to "e4".
pub fn get_current_pos(game: &GameState, action: &Action) -> Result<Position, ActionError> {
    // Find all pieces with same PieceKind an Color
    let piece = Piece::new(action.piece_kind, game.current_player);
    let mut pieces_possibilities: Vec<Position> = Vec::new();
    for (r, row) in game.board.squares.iter().enumerate() {
        for (c, square) in row.iter().enumerate() {
            if square.piece == Some(piece) {
                pieces_possibilities.push(Position::new(c as u8, r as u8));
            }
        }
    }

    let mut res = Vec::new();

    // For each piece_possibility, we check the available moves. If we find
    // the target position, we store the piece.
    for piece_possibility in pieces_possibilities {
        let moves = move_possibilities(game, &piece, &piece_possibility);
        let find = moves.iter().find(|m| **m == action.to);
        if find.is_some() {
            res.push(piece_possibility);
        }
    }

    if res.len() == 0 {
        return Err(ActionError::new("No initial position find, res == 0"));
    }

    // if single possibility return it, else we must find the good one
    if res.len() == 1 {
        return Ok(res[0]);
    }

    if action.from.is_none() {
        return Err(ActionError::new("No disambig position"));
    }
    let from = action.from.unwrap();
    let res = res.iter().find(|p| {
        if from.col.is_some() && from.row.is_some() {
            return p.col == from.col.unwrap() && p.row == from.row.unwrap();
        } else if from.col.is_some() {
            return p.col == from.col.unwrap();
        }

        return p.row == from.row.unwrap();
    });

    if res.is_some() {
        return Ok(res.unwrap().clone());
    }

    println!("{:?}", res);
    return Err(ActionError::new("No initial position find, res > 1"));
}

pub fn manage_special_moves(piece: &Piece, game: &mut GameState, action: &Action) {
    let to = action.to;

    if is_en_passant_capture(game, &to, piece) {
        let pawn = backward_one_square(&to, &piece.color);
        game.board.squares[pawn.row as usize][pawn.col as usize] = Square::new(None);
    }

    if piece.kind == PieceKind::King && piece.color == Color::White && to.col == 6 {
        game.board.squares[7][7] = Square::new(None);
        game.board.squares[7][5] = Square::new(Some(Piece::new(PieceKind::Rook, Color::White)));
    }
    if piece.kind == PieceKind::King && piece.color == Color::Black && to.col == 6 {
        game.board.squares[0][7] = Square::new(None);
        game.board.squares[0][5] = Square::new(Some(Piece::new(PieceKind::Rook, Color::Black)));
    }
    if piece.kind == PieceKind::King && piece.color == Color::White && to.col == 2 {
        game.board.squares[7][0] = Square::new(None);
        game.board.squares[7][3] = Square::new(Some(Piece::new(PieceKind::Rook, Color::White)));
    }
    if piece.kind == PieceKind::King && piece.color == Color::Black && to.col == 2 {
        game.board.squares[0][0] = Square::new(None);
        game.board.squares[0][3] = Square::new(Some(Piece::new(PieceKind::Rook, Color::Black)));
    }
}

pub fn is_capture(game: &GameState, to: &Position, piece: &Piece) -> bool {
    if is_en_passant_capture(game, to, piece) {
        return true;
    }
    !is_empty_square(&game.board.squares, to)
}

pub fn is_en_passant_capture(game: &GameState, to: &Position, piece: &Piece) -> bool {
    piece.kind == PieceKind::Pawn
        && game.en_passant_target != None
        && *to == game.en_passant_target.unwrap()
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
    use crate::types::{Color, GameStatus, Square, Squares};

    #[test]
    fn default_initialize() {
        let game = initialize(None);
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
        assert_eq!(game.history, Vec::<Action>::new());
        assert_eq!(game.status, GameStatus::InProgress);
        assert_eq!(game.board, Board::new(squares));
    }
}
