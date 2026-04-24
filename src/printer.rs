use crate::{
    san,
    types::{Color, GameState, GameStatus, Piece, PieceKind, Square},
    utils::{castling_to_str, pos_to_str},
};

pub fn print_board(game: &GameState) {
    println!("     a b c d e f g h");
    println!("-------------------------");

    for (i, row) in game.board.squares.iter().enumerate() {
        let row_num = 8 - i;
        println!(
            "{} |  {}  | {}",
            row_num,
            row.map(|square| print_square(square)).join(" "),
            row_num
        )
    }

    println!("-------------------------");
    println!("     a b c d e f g h\n");
}

pub fn print_fen(game: &GameState) -> String {
    let mut board: Vec<String> = vec![];
    for row in game.board.squares {
        let mut line = String::new();
        let mut c = 0;
        for (i, square) in row.iter().enumerate() {
            if square.piece == None {
                c += 1;
                if i == row.len() - 1 {
                    line.push_str(&c.to_string());
                }
            } else {
                if c != 0 {
                    line.push_str(&c.to_string());
                    c = 0;
                }
                let mut piece: String = print_piece_kind(&square.piece.unwrap().kind).to_string();
                if square.piece.unwrap().color == Color::Black {
                    piece = piece.to_lowercase();
                }
                line.push_str(&piece);
            }
        }
        board.push(line);
    }

    let color = if game.current_player == Color::White {
        "w"
    } else {
        "b"
    };
    let en_passant_target = game.en_passant_target;
    let en_passant_target = if en_passant_target == None {
        String::from("-")
    } else {
        pos_to_str(&en_passant_target.unwrap())
    };

    format!(
        "{} {} {} {} {} {}",
        board.join("/"),
        color,
        castling_to_str(&game.castling_possibilities),
        en_passant_target,
        game.halfmove_clock,
        game.fullmove_number
    )
}

pub fn print_pgn_content(game: &GameState) -> String {
    let pgn_content_str: String = game
        .history
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i % 2 == 0 {
                format!("{}.{}", (i / 2) + 1, san::to_string(a))
            } else {
                format!("{}", san::to_string(a))
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    format!("{}", pgn_content_str)
}

pub fn print_piece_kind(kind: &PieceKind) -> &'static str {
    match kind {
        PieceKind::Rook => "R",
        PieceKind::Knight => "N",
        PieceKind::Bishop => "B",
        PieceKind::King => "K",
        PieceKind::Queen => "Q",
        PieceKind::Pawn => "P",
    }
}

pub fn print_square(square: Square) -> &'static str {
    if None == square.piece {
        return ".";
    }

    print_piece(&square.piece.unwrap())
}

pub fn print_piece(piece: &Piece) -> &'static str {
    match piece.kind {
        PieceKind::Rook => {
            if piece.color == Color::White {
                "♜"
            } else {
                "♖"
            }
        }
        PieceKind::Knight => {
            if piece.color == Color::White {
                "♞"
            } else {
                "♘"
            }
        }
        PieceKind::Bishop => {
            if piece.color == Color::White {
                "♝"
            } else {
                "♗"
            }
        }
        PieceKind::King => {
            if piece.color == Color::White {
                "♚"
            } else {
                "♔"
            }
        }
        PieceKind::Queen => {
            if piece.color == Color::White {
                "♛"
            } else {
                "♕"
            }
        }
        PieceKind::Pawn => {
            if piece.color == Color::White {
                "🨅"
            } else {
                "♙"
            }
        }
    }
}

pub fn print_game_result(game: &GameState) -> &str {
    if game.status == GameStatus::InProgress {
        return "";
    }
    let mut status = "1/2-1/2";
    if let GameStatus::Checkmate(color) = game.status {
        if color == Color::White {
            status = "0-1"
        } else {
            status = "1-0"
        }
    }
    status
}
