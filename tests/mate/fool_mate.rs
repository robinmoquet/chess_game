use chess_game::{
    Color, GameStatus, do_action, initialize, print_fen, print_game_result, print_pgn_content,
};

#[test]
fn white_fool_mate() {
    let game = initialize(None);

    let (_, game) = do_action("e4".to_string(), game);
    let (_, game) = do_action("f6".to_string(), game);
    let (_, game) = do_action("d4".to_string(), game);
    let (_, game) = do_action("g5".to_string(), game);
    let (_, game) = do_action("Qh5".to_string(), game);

    assert_eq!(
        "rnbqkbnr/ppppp2p/5p2/6pQ/3PP3/8/PPP2PPP/RNB1KBNR b KQkq - 1 3".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::Black), game.status);
    assert_eq!(
        "1.e4 f6 2.d4 g5 3.Qh5#".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("1-0", print_game_result(&game));
}

#[test]
fn black_fool_mate() {
    let game = initialize(None);

    let (_, game) = do_action("f3".to_string(), game);
    let (_, game) = do_action("e6".to_string(), game);
    let (_, game) = do_action("g4".to_string(), game);
    let (_, game) = do_action("Qh4".to_string(), game);

    assert_eq!(
        "rnb1kbnr/pppp1ppp/4p3/8/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::White), game.status);
    assert_eq!("1.f3 e6 2.g4 Qh4#".to_string(), print_pgn_content(&game));
    assert_eq!("0-1", print_game_result(&game));
}
