use chess_game::{clear, initialize, print};

fn main() {
    clear();
    let game = initialize();
    print(&game);
}
