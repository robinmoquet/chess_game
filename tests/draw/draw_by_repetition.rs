use chess_game::{
    GameStatus, do_action, initialize, print_fen, print_game_result, print_pgn_content,
};

#[test]
fn test_draw() {
    let game = initialize(Some(
        "8/3k4/1n3p2/4p3/4P3/1B3P2/4K3/8 w - - 1 1".to_string(),
    ));
    let (_, game) = do_action("Bc2".to_string(), game);
    let (_, game) = do_action("Nc4".to_string(), game);
    let (_, game) = do_action("Bb3".to_string(), game);
    let (_, game) = do_action("Nb6".to_string(), game);

    let (_, game) = do_action("Bc2".to_string(), game);
    let (_, game) = do_action("Nc4".to_string(), game);
    let (_, game) = do_action("Bb3".to_string(), game);
    let (_, game) = do_action("Nb6".to_string(), game);

    assert_eq!(
        "8/3k4/1n3p2/4p3/4P3/1B3P2/4K3/8 w - - 9 5".to_string(),
        print_fen(&game)
    );
    assert_eq!(GameStatus::Draw, game.status);
    assert_eq!(
        "1.Bc2 Nc4 2.Bb3 Nb6 3.Bc2 Nc4 4.Bb3 Nb6".to_string(),
        print_pgn_content(&game)
    );
    assert_eq!("1/2-1/2", print_game_result(&game));
}
