use std::fs;
mod parse;
use std::collections::VecDeque;

impl parse::Problem {
    fn do_moves(&mut self) {
        for m in self.moves.chunks(3) {
            let n = m[0];
            let from_index = (m[1] - 1) as usize;
            let to_index = (m[2] - 1) as usize;
            let mut temp: Vec<char> = Vec::new();
            for _ in 0..n {
                let val = self.stacks[from_index].pop();
                temp.push(val.unwrap())
            }
            for &val in temp.iter().rev() {
                self.stacks[to_index].push(val)
            }
        }
    }
    fn get_top(self) -> String {
        let values: String = self.stacks.iter().map(|v| *v.last().unwrap()).collect();
        values
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut res = parse::setup_parser(&data).unwrap().1;
    res.do_moves();
    println!("{:?}", res.get_top());
}
