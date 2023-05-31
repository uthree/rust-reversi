mod reversi;
mod vector2;

use reversi::{Board, Color, Position};

fn main() {
    let board = Board::new();
    println!(
        "{}",
        board.visualize_for_tui(Color::Black, Position::new(0, 0))
    );
}
