mod reversi;

use crate::reversi::{Game, ManualPlayer, RandomPlayer};

fn main() {
    let mut p1 = ManualPlayer::<8, 8>::new();
    let mut p2 = RandomPlayer::<8, 8>::new();
    let mut game = Game::<8, 8>::new(&mut p1, &mut p2);
    game.run();
}
