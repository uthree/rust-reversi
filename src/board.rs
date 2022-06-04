use std::ops;
use std::vec::Vec;

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    White,
    Black,
}

pub struct Board {
    height: usize,
    width: usize,
    data: Vec<Cell>,
}

#[derive(Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        let p = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
        return p;
    }
}

#[derive(Clone, Copy, Debug)]
struct OutOfBoard;

impl Board {
    pub fn new(height: usize, width: usize) -> Board {
        let mut data: Vec<Cell> = Vec::new();
        for _y in 0..height {
            for _x in 0..width {
                data.push(Cell::Empty)
            }
        }
        let center_x = width / 2;
        let center_y = height / 2;
        let mut board = Board {
            height,
            width,
            data,
        };
        board
            .set_cell(
                Point {
                    x: center_x - 1,
                    y: center_y - 1,
                },
                Cell::White,
            )
            .unwrap();
        board
            .set_cell(
                Point {
                    x: center_x,
                    y: center_y,
                },
                Cell::White,
            )
            .unwrap();
        board
            .set_cell(
                Point {
                    x: center_x,
                    y: center_y - 1,
                },
                Cell::Black,
            )
            .unwrap();
        board
            .set_cell(
                Point {
                    x: center_x - 1,
                    y: center_y,
                },
                Cell::Black,
            )
            .unwrap();
        return board;
    }

    fn get_cell(&self, point: Point) -> Result<Cell, OutOfBoard> {
        let idx = point.y * self.height + point.x;
        if point.y > self.height - 1 {
            return Err(OutOfBoard);
        }
        if point.x > self.width - 1 {
            return Err(OutOfBoard);
        }

        Ok(self.data[idx])
    }

    fn set_cell(&mut self, point: Point, cell: Cell) -> Result<(), OutOfBoard> {
        if point.y > self.height - 1 {
            return Err(OutOfBoard);
        }
        if point.x > self.width - 1 {
            return Err(OutOfBoard);
        }
        let idx = point.y * self.height + point.x;
        self.data[idx] = cell;

        Ok(())
    }

    fn check_can_placable(&self, point: Point, cell: Cell) -> Result<(), OutOfBoard> {
        Ok(())
    }
}

use std::string::ToString;
impl ToString for Board {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                let piece_s = match self.get_cell(Point { x, y }).unwrap() {
                    Cell::Empty => ".",
                    Cell::White => "w",
                    Cell::Black => "b",
                }
                .to_string();
                s = [s, piece_s].join("");
            }
            s = [s, "\n".to_string()].join("");
        }
        s
    }
}
