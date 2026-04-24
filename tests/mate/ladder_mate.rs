use chess_game::{
    Color, GameStatus, do_action, initialize, print_fen, print_game_result, print_pgn_content,
};

#[test]
fn white_ladder_mate() {
    let game = initialize(Some("8/8/5k2/R7/1R6/8/8/2K5 w - - 0 1".to_string()));
    let (_, game) = do_action("Rb6".to_string(), game);
    assert_eq!(GameStatus::Check(Color::Black), game.status);

    let (_, game) = do_action("Ke7".to_string(), game);
    let (_, game) = do_action("Ra7".to_string(), game);
    let (_, game) = do_action("Kd8".to_string(), game);
    let (_, game) = do_action("Rb8".to_string(), game);

    assert_eq!(
        "1R1k4/R7/8/8/8/8/8/2K5 b - - 5 3".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::Black), game.status);
    assert_eq!(
        "1.Rb6+ Ke7 2.Ra7+ Kd8 3.Rb8#".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("1-0", print_game_result(&game));
}

#[test]
fn black_ladder_mate() {
    let game = initialize(Some("8/8/5K2/r7/1r6/8/8/2k5 b - - 0 1".to_string()));
    let (_, game) = do_action("Rb6".to_string(), game);
    assert_eq!(GameStatus::Check(Color::White), game.status);

    let (_, game) = do_action("Ke7".to_string(), game);
    let (_, game) = do_action("Ra7".to_string(), game);
    let (_, game) = do_action("Kd8".to_string(), game);
    let (_, game) = do_action("Rb8".to_string(), game);

    assert_eq!(
        "1r1K4/r7/8/8/8/8/8/2k5 w - - 5 4".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Checkmate(Color::White), game.status);
    assert_eq!(
        "1.Rb6+ Ke7 2.Ra7+ Kd8 3.Rb8#".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("0-1", print_game_result(&game));
}
