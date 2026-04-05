use crate::{
    types::{Color, GameState, Move, Piece, PieceKind, Square},
    utils::pos_to_str,
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

pub fn print_fen(_game: &GameState) {}

pub fn print_pgn_content(game: &GameState) {
    let pgn_content_str: String = game
        .moves_history
        .iter()
        .enumerate()
        .map(|(i, m)| {
            if i % 2 == 0 {
                format!("{}. {} ", (i / 2) + 1, print_move(&m))
            } else {
                format!("{} ", print_move(&m))
            }
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}\n", pgn_content_str)
}

pub fn print_move(m: &Move) -> String {
    let piece = m.piece;
    if piece.kind == PieceKind::Pawn {
        return pos_to_str(&m.to).to_string();
    }
    let piece_kind = match piece.kind {
        PieceKind::Rook => "R",
        PieceKind::Knight => "N",
        PieceKind::Bishop => "B",
        PieceKind::King => "K",
        PieceKind::Queen => "Q",
        PieceKind::Pawn => "P",
    };

    format!("{}{}", piece_kind, pos_to_str(&m.to))
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
