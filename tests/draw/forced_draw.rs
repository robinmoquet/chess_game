use chess_game::{
    GameStatus, do_action, initialize, print_fen, print_game_result, print_pgn_content,
};

#[test]
fn king_knight() {
    let game = initialize(Some("8/8/1kn5/8/8/8/1B1K1N2/8 w - - 0 1".to_string()));
    let (_, game) = do_action("Bd4+".to_string(), game);
    let (_, game) = do_action("Nxd4".to_string(), game);
    let (_, game) = do_action("Kd3".to_string(), game);
    let (_, game) = do_action("Kb5".to_string(), game);
    let (_, game) = do_action("Kxd4".to_string(), game);

    assert_eq!(
        "8/8/8/1k6/3K4/8/5N2/8 b - - 0 3".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Draw, game.status);
    assert_eq!(
        "1.Bd4+ Nxd4 2.Kd3 Kb5 3.Kxd4".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("1/2-1/2", print_game_result(&game));
}

#[test]
fn king_bishop() {
    let game = initialize(Some("8/4b3/2k5/8/8/2N5/3K4/8 b - - 1 1".to_string()));
    let (_, game) = do_action("Bb4".to_string(), game);
    let (_, game) = do_action("Ke2".to_string(), game);
    let (_, game) = do_action("Bxc3".to_string(), game);

    assert_eq!(
        "8/8/2k5/8/8/2b5/4K3/8 w - - 0 3".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Draw, game.status);
    assert_eq!("1.Bb4 Ke2 2.Bxc3".to_string(), print_pgn_content(&game));
    assert_eq!("1/2-1/2", print_game_result(&game));
}
