use chess_game::{initialize, play};

fn main() {
    play(initialize(None));
    // play(initialize(Some(
    //     "8/8/3ppp2/6k1/4K3/4PP2/8/8 b - - 0 1".to_string(),
    // )));
}
