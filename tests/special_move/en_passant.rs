use chess_game::{do_action, initialize, print_fen, str_to_pos};

#[test]
fn white_en_passant() {
    let game = initialize(None);
    let (_, game) = do_action("e4".to_string(), game);
    let (_, game) = do_action("d5".to_string(), game);
    let (_, game) = do_action("e5".to_string(), game);
    let (_, game) = do_action("f5".to_string(), game);
    assert_eq!(Some(str_to_pos("f6").unwrap()), game.en_passant_target);

    let (_, game) = do_action("exf6".to_string(), game);
    assert_eq!(
        "rnbqkbnr/ppp1p1pp/5P2/3p4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3".to_string(),
        print_fen(&game)
    );
}

#[test]
fn black_en_passant() {
    let game = initialize(None);
    let (_, game) = do_action("d4".to_string(), game);
    let (_, game) = do_action("c5".to_string(), game);
    let (_, game) = do_action("d5".to_string(), game);
    let (_, game) = do_action("c4".to_string(), game);
    let (_, game) = do_action("b4".to_string(), game);
    assert_eq!(Some(str_to_pos("b3").unwrap()), game.en_passant_target);

    let (_, game) = do_action("cxb3".to_string(), game);
    assert_eq!(
        "rnbqkbnr/pp1ppppp/8/3P4/8/1p6/P1P1PPPP/RNBQKBNR w KQkq - 0 4".to_string(),
        print_fen(&game)
    );
}
