#[macro_use] extern crate enum_primitive;

extern crate num;

use num::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum Direction {
    North = 0,
    East  = 1,
    South = 2,
    West  = 3,
}
}

pub struct Robot {
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {

    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let d = (self.d as u8 + 1) % 4;
        Robot { x: self.x, y: self.y, d: Direction::from_u8(d).unwrap() }
    }

    pub fn turn_left(self) -> Self {
        let d = if self.d == Direction::North { 3 } else { self.d as u8 - 1 };
        Robot { x: self.x, y: self.y, d: Direction::from_u8(d).unwrap() }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };

        Robot { x, y, d: self.d }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
             _  => panic!(),
        })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
