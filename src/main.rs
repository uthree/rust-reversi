mod reversi;

use crate::reversi::{Board, Color, Position};

fn main() {
    let mut board = Board::<8, 8>::new();
    loop {
        board.place(Color::Black, board.placeable_positions(Color::Black)[0]);
        println!("{}", board.visualize());

        board.place(Color::White, board.placeable_positions(Color::White)[0]);

        println!("{}", board.visualize());
    }
}
