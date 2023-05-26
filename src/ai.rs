use crate::reversi::*;

pub struct MinMaxPlayer<const WIDTH: usize, const HEIGHT: usize> {
    color: Color,
}

impl<const WIDTH: usize, const HEIGHT: usize> Player<WIDTH, HEIGHT>
    for MinMaxPlayer<WIDTH, HEIGHT>
{
    fn decide_position(&mut self, board: &Board<WIDTH, HEIGHT>) -> Vector2<i8> {
        Self::highest_score_position(self.color, board, 4)
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

    fn evaluate_board(color: Color, board: &Board<WIDTH, HEIGHT>) -> f32 {
        let mut score: f32 = 0.0;
        score += board.count(color) as f32 / 64f32;
        if board.count(color) + board.count(color.opponent()) < 48 {
            if board.data[0][0] == color.into() {
                score += 1.0f32;
            }
            if board.data[7][0] == color.into() {
                score += 1.0f32;
            }
            if board.data[0][7] == color.into() {
                score += 1.0f32;
            }
            if board.data[7][7] == color.into() {
                score += 1.0f32;
            }
        }

        score
    }

    fn evaluate_position(
        color: Color,
        board: &Board<WIDTH, HEIGHT>,
        position: Position,
        depth: usize,
    ) -> f32 {
        if depth <= 0 {
            return 0f32;
        }
        let mut score: f32 = 0.0;
        let mut b = (*board).clone();
        if b.check_placeable(color, position) {
            b.place(color, position).unwrap();
            score += Self::evaluate_board(color, &b);
            if b.check_placeable_somewhere(color.opponent()) {
                let p = Self::highest_score_position(color.opponent(), &b, depth - 1);
                b.place(color.opponent(), p).unwrap();
                score -= Self::evaluate_board(color, &b) * 0.98;
            } else {
                return 1.0f32;
            }
        } else {
            return -1.0f32;
        }

        score
    }

    fn highest_score_position(
        color: Color,
        board: &Board<WIDTH, HEIGHT>,
        depth: usize,
    ) -> Position {
        let mut max_score: f32 = f32::MIN;
        let mut best_position = Position::new(-1, -1); // empty data
        board.placeable_positions(color).iter().for_each(|p| {
            let s = Self::evaluate_position(color, board, *p, depth);
            if max_score < s {
                max_score = s;
                best_position = *p;
            }
        });
        best_position
    }
}
