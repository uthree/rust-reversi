use rand::seq::IteratorRandom;

#[derive(Clone, Copy)]
pub struct Vector2<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
}

pub type Position = Vector2<i8>;

impl<T> std::ops::Add<Vector2<T>> for Vector2<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn add(self, other: Vector2<T>) -> Self::Output {
        Vector2 {
            x: other.x + self.x,
            y: other.y + self.y,
        }
    }
}

impl<T> std::ops::Mul<T> for Vector2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn mul(self, scaler: T) -> Self::Output {
        Vector2 {
            x: self.x * scaler,
            y: self.y * scaler,
        }
    }
}

impl<T> Vector2<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    None,
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl From<Color> for Cell {
    fn from(color: Color) -> Cell {
        match color {
            Color::Black => Cell::Black,
            Color::White => Cell::White,
        }
    }
}

impl Color {
    fn opponent(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    data: [[Cell; WIDTH]; HEIGHT],
}

const VALID_DIRECTIONS: [Vector2<i8>; 8] = [
    Vector2 { x: 0, y: 1 },
    Vector2 { x: 1, y: 1 },
    Vector2 { x: 1, y: 0 },
    Vector2 { x: -1, y: -1 },
    Vector2 { x: -1, y: 1 },
    Vector2 { x: 1, y: -1 },
    Vector2 { x: 0, y: -1 },
    Vector2 { x: -1, y: 0 },
];

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let mut data = [[Cell::None; WIDTH]; HEIGHT];
        let center_x = WIDTH / 2 - 1;
        let center_y = HEIGHT / 2 - 1;
        data[center_y][center_x] = Cell::White;
        data[center_y + 1][center_x + 1] = Cell::White;
        data[center_y + 1][center_x] = Cell::Black;
        data[center_y][center_x + 1] = Cell::Black;

        Board { data }
    }

    pub fn visualize(&self) -> String {
        let mut out = " abcdefgh\n".to_string();
        for (i, line) in self.data.iter().enumerate() {
            out += &(i + 1).to_string();
            for cell in line {
                match cell {
                    Cell::None => out += ".",
                    Cell::White => out += "w",
                    Cell::Black => out += "b",
                }
            }
            out += "\n";
        }
        out
    }

    fn placeable_directions(&self, color: Color, position: Vector2<i8>) -> Vec<Vector2<i8>> {
        let (x, y) = (position.x, position.y);
        let mut result = vec![];
        if self.data[y as usize][x as usize] != Cell::None {
            return result;
        }
        for d in VALID_DIRECTIONS {
            let mut flag_opponent_color = false;
            for s in 1..std::cmp::max(WIDTH, HEIGHT) as i8 {
                let p = position + d * s;
                if !Self::check_valid_position(p) {
                    break;
                }
                let c = self.data[p.y as usize][p.x as usize];
                if c == color.into() && !flag_opponent_color {
                    break;
                } else if c == color.opponent().into() {
                    flag_opponent_color = true;
                } else if c == color.into() && flag_opponent_color {
                    result.push(d);
                    break;
                } else if c == Cell::None {
                    break;
                }
            }
        }
        result
    }

    pub fn place(&mut self, color: Color, position: Vector2<i8>) -> Result<(), &str> {
        if !self.check_placeable(color, position) {
            return Err("Can't place here!");
        }
        let dirs = self.placeable_directions(color, position);
        self.data[position.y as usize][position.x as usize] = color.into();
        for d in dirs {
            for s in 1..(std::cmp::max(WIDTH, HEIGHT) as i8) {
                let p = position + d * s;
                if !Self::check_valid_position(p) {
                    break;
                }
                if self.data[p.y as usize][p.x as usize] == color.into() {
                    break;
                }
                self.data[p.y as usize][p.x as usize] = color.into()
            }
        }
        Ok(())
    }

    pub fn check_placeable(&self, color: Color, position: Vector2<i8>) -> bool {
        Self::check_valid_position(position) && !self.placeable_directions(color, position).is_empty()
    }

    pub fn check_placeable_somewhere(&self, color: Color) -> bool {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.check_placeable(color, Position::new(x as i8, y as i8)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn placeable_positions(&self, color: Color) -> Vec<Vector2<i8>> {
        let mut result = vec![];
        for y in 0..WIDTH {
            for x in 0..HEIGHT {
                if self.check_placeable(color, Position::new(x as i8, y as i8)) {
                    result.push(Position::new(x as i8, y as i8));
                }
            }
        }
        result
    }

    fn check_valid_position(position: Vector2<i8>) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < WIDTH as i8 && position.y < HEIGHT as i8
    }

    pub fn count(&self, color: Color) -> usize {
        let mut count = 0;
        for y in 0..(WIDTH - 1) {
            for x in 0..(HEIGHT - 1) {
                if self.data[y][x] == color.into() {
                    count += 1
                }
            }
        }
        count
    }
}

pub trait Player<const WIDTH: usize, const HEIGHT: usize> {
    fn tell(&mut self, content: String) {
        println!("{}", content);
    }
    fn tell_color(&mut self, _color: Color) {}
    fn decide_position(&mut self, board: &Board<WIDTH, HEIGHT>) -> Vector2<i8>;
}

pub struct Game<'a, const WIDTH: usize, const HEIGHT: usize> {
    player1: &'a mut dyn Player<WIDTH, HEIGHT>,
    player2: &'a mut dyn Player<WIDTH, HEIGHT>,
    board: Board<WIDTH, HEIGHT>,
}

impl<'a, const WIDTH: usize, const HEIGHT: usize> Game<'a, WIDTH, HEIGHT> {
    pub fn new(
        player1: &'a mut dyn Player<WIDTH, HEIGHT>,
        player2: &'a mut dyn Player<WIDTH, HEIGHT>,
    ) -> Self {
        Game {
            player1,
            player2,
            board: Board::new(),
        }
    }

    pub fn run(&mut self) {
        self.broadcast("Game started!".to_string());
        self.player1.tell_color(Color::Black);
        self.player2.tell_color(Color::White);
        loop {
            if self.board.check_placeable_somewhere(Color::Black) {
                self.player1.tell(self.board.visualize());
                let pos = self.player1.decide_position(&self.board);
                self.board.place(Color::Black, pos).unwrap();
            }
            if self.board.check_placeable_somewhere(Color::White) {
                self.player2.tell(self.board.visualize());
                let pos = self.player2.decide_position(&self.board);
                self.board.place(Color::White, pos).unwrap();
            }
            if !self.board.check_placeable_somewhere(Color::Black)
                && !self.board.check_placeable_somewhere(Color::White)
            {
                break;
            }
        }
        self.broadcast("Game End!".to_string());
    }

    fn broadcast(&mut self, content: String) {
        self.player1.tell(content.clone());
        self.player2.tell(content);
    }
}

pub struct RandomPlayer<const WIDTH: usize, const HEIGHT: usize> {
    color: Color,
}

impl<const WIDTH: usize, const HEIGHT: usize> Player<WIDTH, HEIGHT>
    for RandomPlayer<WIDTH, HEIGHT>
{
    fn decide_position(&mut self, board: &Board<WIDTH, HEIGHT>) -> Vector2<i8> {
        *board
            .placeable_positions(self.color)
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    fn tell_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> RandomPlayer<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        RandomPlayer {
            color: Color::Black,
        }
    }
}
