mod moves;
mod rope;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let mvs: Vec<moves::Move> = moves::read_from_lines(reader).collect();
    let locs_1 = rope::get_unique_locations(mvs.iter());
    let locs_2 = rope::get_uniqe_chain_locations(mvs.iter(), 9);
    println!("{:?}", locs_2.len())
}
