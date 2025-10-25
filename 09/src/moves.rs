use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Move {
    pub direction: Direction,
    pub distance: usize,
}

impl Move {
    fn new(direction: Direction, distance: usize) -> Self {
        Move {
            direction,
            distance,
        }
    }
}

impl TryFrom<(char, usize)> for Move {
    type Error = ();
    fn try_from(value: (char, usize)) -> Result<Self, Self::Error> {
        match value.0.to_uppercase().next() {
            Some('U') => Ok(Move::new(Direction::UP, value.1)),
            Some('D') => Ok(Move::new(Direction::DOWN, value.1)),
            Some('L') => Ok(Move::new(Direction::LEFT, value.1)),
            Some('R') => Ok(Move::new(Direction::RIGHT, value.1)),
            _ => Err(()),
        }
    }
}

pub fn read_from_lines(lines: Lines<BufReader<File>>) -> impl Iterator<Item = Move> {
    lines.map_while(Result::ok).map(|elems| {
        let mut vals = elems.split_whitespace();
        let dir = vals.next().unwrap().chars().nth(0).unwrap();
        let size = vals.next().unwrap().parse::<usize>().unwrap();
        Move::try_from((dir, size)).unwrap()
    })
}
