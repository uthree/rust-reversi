use crate::reversi::*;

pub struct MinMaxPlayer<const WIDTH: usize, const HEIGHT: usize> {
    color: Color,
}

impl<const WIDTH: usize, const HEIGHT: usize> Player<WIDTH, HEIGHT>
    for MinMaxPlayer<WIDTH, HEIGHT>
{
    fn decide_position(&mut self, board: &Board<WIDTH, HEIGHT>) -> Vector2<i8> {
        todo!()
    }

    fn tell_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> MinMaxPlayer<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        MinMaxPlayer {
            color: Color::Black,
        }
    }
}
