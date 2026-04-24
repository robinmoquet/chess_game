use chess_game::{
    Color, GameStatus, do_action, initialize, print_fen, print_game_result, print_pgn_content,
};

#[test]
fn white_scholar_mate() {
    let game = initialize(None);
    let (_, game) = do_action("e4".to_string(), game);
    let (_, game) = do_action("e5".to_string(), game);
    let (_, game) = do_action("Bc4".to_string(), game);
    let (_, game) = do_action("Nc6".to_string(), game);
    let (_, game) = do_action("Qh5".to_string(), game);
    let (_, game) = do_action("Nf6".to_string(), game);
    let (_, game) = do_action("Qxf7".to_string(), game);

    assert_eq!(
        "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::Black), game.status);
    assert_eq!(
        "1.e4 e5 2.Bc4 Nc6 3.Qh5 Nf6 4.Qxf7#".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("1-0", print_game_result(&game));
}

#[test]
fn black_scholar_mate() {
    let game = initialize(None);
    let (_, game) = do_action("e4".to_string(), game);
    let (_, game) = do_action("e5".to_string(), game);
    let (_, game) = do_action("Nc3".to_string(), game);
    let (_, game) = do_action("Bc5".to_string(), game);
    let (_, game) = do_action("d3".to_string(), game);
    let (_, game) = do_action("Qh4".to_string(), game);
    let (_, game) = do_action("Nf3".to_string(), game);
    let (_, game) = do_action("Qf2".to_string(), game);

    assert_eq!(
        "rnb1k1nr/pppp1ppp/8/2b1p3/4P3/2NP1N2/PPP2qPP/R1BQKB1R w KQkq - 0 5".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::White), game.status);
    assert_eq!(
        "1.e4 e5 2.Nc3 Bc5 3.d3 Qh4 4.Nf3 Qxf2#".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("0-1", print_game_result(&game));
}
