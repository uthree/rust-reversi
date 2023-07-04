pub use crate::vector2::Vector2;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl From<Color> for CellState {
    fn from(color: Color) -> CellState {
        match color {
            Color::Black => CellState::Black,
            Color::White => CellState::White,
        }
    }
}

impl Color {
    fn opponent(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

pub type Position = Vector2<isize>;

#[derive(Clone)]
pub struct Board {
    data: Vec<Vec<CellState>>,
}

const VALID_DIRECTIONS: [Vector2<isize>; 8] = [
    Vector2 { x: 0, y: 1 },
    Vector2 { x: 1, y: 1 },
    Vector2 { x: 1, y: 0 },
    Vector2 { x: -1, y: -1 },
    Vector2 { x: -1, y: 1 },
    Vector2 { x: 1, y: -1 },
    Vector2 { x: 0, y: -1 },
    Vector2 { x: -1, y: 0 },
];

impl Board {
    pub fn new_with_size(size: Vector2<usize>) -> Self {
        let mut data = vec![];
        for _ in 0..size.y {
            let mut col = vec![];
            for _ in 0..size.x {
                col.push(CellState::Empty);
            }
            data.push(col);
        }
        let center_x = size.x / 2 - 1;
        let center_y = size.y / 2 - 1;
        data[center_y][center_x] = CellState::White;
        data[center_y + 1][center_x + 1] = CellState::White;
        data[center_y + 1][center_x] = CellState::Black;
        data[center_y][center_x + 1] = CellState::Black;

        Board { data }
    }

    pub fn new() -> Self {
        Self::new_with_size(Vector2::new(8, 8))
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn visualize(&self) -> String {
        let mut out = "".to_string();
        for line in self.data.iter() {
            for cell in line {
                match cell {
                    CellState::Empty => out += "\x1b\x5b0m\x1b\x5b42m\x1b\x5b37m \x1b\x5b0m",
                    CellState::White => out += "\x1b\x5b0m\x1b\x5b42m\x1b\x5b37m●\x1b\x5b0m",
                    CellState::Black => out += "\x1b\x5b0m\x1b\x5b42m\x1b\x5b30m●\x1b\x5b0m",
                }
            }
            out += "\n";
        }
        out
    }

    pub fn visualize_for_tui(&self, color: Color, cursor: Vector2<isize>) -> String {
        let mut out = "".to_string();
        for (y, line) in self.data.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let object = match cell {
                    CellState::Empty => {
                        if self.check_placeable(color, Position::new(x as isize, y as isize)) {
                            if color == Color::Black {
                                "\x1b\x5b30m."
                            } else {
                                "\x1b\x5b37m."
                            }
                        } else {
                            " "
                        }
                    }
                    CellState::Black => "\x1b\x5b30m●",
                    CellState::White => "\x1b\x5b37m●",
                };
                let bg = if x as isize == cursor.x && y as isize == cursor.y {
                    "\x1b\x5b44m"
                } else {
                    "\x1b\x5b42m"
                };
                let reset = "\x1b\x5b0m";
                out += &format!("{}{}{}", bg, object, reset);
            }
            out += "\n";
        }
        out
    }

    fn placeable_directions(&self, color: Color, position: Vector2<isize>) -> Vec<Vector2<isize>> {
        let mut result = vec![];
        if self.data[position.y as usize][position.x as usize] != CellState::Empty {
            return result;
        }
        for d in VALID_DIRECTIONS {
            let mut flag_opponent_color = false;
            for s in 1..std::cmp::max(self.width(), self.height()) as isize {
                let p = position + d * s;
                if !self.check_valid_position(p) {
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
                } else if c == CellState::Empty {
                    break;
                }
            }
        }
        result
    }

    pub fn place(&mut self, color: Color, position: Vector2<isize>) -> Result<(), &str> {
        if !self.check_placeable(color, position) {
            return Err("Can't place here!");
        }
        let dirs = self.placeable_directions(color, position);
        self.data[position.y as usize][position.x as usize] = color.into();
        for d in dirs {
            for s in 1..(std::cmp::max(self.width(), self.height()) as isize) {
                let p = position + d * s;
                if !self.check_valid_position(p) {
                    break;
                }
                if self.data[p.y as usize][p.x as usize] == color.into() {
                    break;
                }
                self.data[p.y as usize][p.x as usize] = color.into();
            }
        }
        Ok(())
    }

    pub fn placeable_positions(&self, color: Color) -> Vec<Vector2<isize>> {
        let mut result = vec![];
        for y in 0..self.width() {
            for x in 0..self.height() {
                if self.check_placeable(color, Position::new(x as isize, y as isize)) {
                    result.push(Position::new(x as isize, y as isize));
                }
            }
        }
        result
    }

    fn check_placeable(&self, color: Color, position: Vector2<isize>) -> bool {
        self.check_valid_position(position)
            && !self.placeable_directions(color, position).is_empty()
    }

    fn check_valid_position(&self, position: Vector2<isize>) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x < self.width() as isize
            && position.y < self.height() as isize
    }

    pub fn check_placeable_somewhere(&self, color: Color) -> bool {
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.check_placeable(color, Position::new(x as isize, y as isize)) {
                    return true;
                }
            }
        }
        false
    }
}

pub trait Player {
    fn decide_position(&self, color: Color, board: &Board) -> Vector2<isize>;
}

pub struct Game<'a> {
    board: Board,
    player_1: &'a dyn Player,
    player_2: &'a dyn Player,
}

impl<'a> Game<'a> {
    fn new(player_1: &'a dyn Player, player_2: &'a dyn Player) -> Self {
        Game {
            board: Board::new(),
            player_1,
            player_2,
        }
    }
    fn run(&mut self) {}
}
