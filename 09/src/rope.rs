use crate::moves::Direction;
use crate::moves::Move;
use std::collections::HashSet;

pub fn get_unique_locations<'a>(moves: impl Iterator<Item = &'a Move>) -> HashSet<(isize, isize)> {
    let mut state = RopeState::new();
    let mut locs = HashSet::new();
    for mv in moves {
        let dlocs = state.do_move(&mv);
        for l in dlocs {
            locs.insert(l);
        }
    }
    return locs;
}

type RopeChain = Vec<RopeState>;

fn new_chain(size: usize) -> RopeChain {
    let mut state: RopeChain = Vec::new();
    for _ in 0..size {
        state.push(RopeState::new())
    }
    state
}

pub fn get_uniqe_chain_locations<'a>(
    moves: impl Iterator<Item = &'a Move>,
    n: usize,
) -> HashSet<(isize, isize)> {
    let mut locs = HashSet::new();
    let mut state = new_chain(n);
    for mv in moves {
        for _ in 0..mv.distance {
            state.get_mut(0).unwrap().move_head_1(&mv.direction);
            state.get_mut(0).unwrap().update_tail();
            for i in 0..state.len() - 1 {
                let slice = &mut state[i..i + 2];
                slice[1].head = slice[0].tail;
                slice[1].update_tail();
            }
            locs.insert(state.last().unwrap().tail.clone());
        }
    }
    locs
}

#[derive(Debug)]
pub struct RopeState {
    pub head: (isize, isize),
    pub tail: (isize, isize),
}

impl RopeState {
    pub fn new() -> Self {
        RopeState {
            head: (0, 0),
            tail: (0, 0),
        }
    }
    pub fn do_move(&mut self, mv: &Move) -> HashSet<(isize, isize)> {
        let mut locs = HashSet::new();
        locs.insert(self.tail.clone());
        for _ in 0..mv.distance {
            self.move_head_1(&mv.direction);
            self.update_tail();
            locs.insert(self.tail.clone());
        }
        locs
    }
    pub fn move_head_1(&mut self, direction: &Direction) {
        let (headx, heady) = self.head;
        match direction {
            Direction::UP => self.head = (headx, heady + 1),
            Direction::DOWN => self.head = (headx, heady - 1),
            Direction::LEFT => self.head = (headx - 1, heady),
            Direction::RIGHT => self.head = (headx + 1, heady),
        }
    }
    fn move_tail_1(&mut self, direction: Direction) {
        let (tailx, taily) = self.tail;
        match direction {
            Direction::UP => self.tail = (tailx, taily + 1),
            Direction::DOWN => self.tail = (tailx, taily - 1),
            Direction::LEFT => self.tail = (tailx - 1, taily),
            Direction::RIGHT => self.tail = (tailx + 1, taily),
        }
    }
    fn update_tail(&mut self) {
        let ds = (self.tail.0 - self.head.0, self.tail.1 - self.head.1);
        let ds_abs = (ds.0.abs(), ds.1.abs());
        if ds_abs.0 <= 1 && ds_abs.1 <= 1 {
            return;
        }
        match ds_abs {
            (2, 0) => {
                if ds.0 < 0 {
                    self.move_tail_1(Direction::RIGHT)
                } else {
                    self.move_tail_1(Direction::LEFT)
                }
            }
            (0, 2) => {
                if ds.1 < 0 {
                    self.move_tail_1(Direction::UP)
                } else {
                    self.move_tail_1(Direction::DOWN)
                }
            }
            _ => {
                if ds.0 < 0 {
                    self.move_tail_1(Direction::RIGHT)
                } else {
                    self.move_tail_1(Direction::LEFT)
                }
                if ds.1 < 0 {
                    self.move_tail_1(Direction::UP)
                } else {
                    self.move_tail_1(Direction::DOWN)
                }
            }
        }
    }
}
