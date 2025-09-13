use std::fs;
mod parse;

fn contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
    ((r1.0 <= r2.0) && (r1.1 >= r2.1)) || ((r2.0 <= r1.0) && (r2.1 >= r1.1))
}
fn overlaps(r1: (usize, usize), r2: (usize, usize)) -> bool {
    return !(r1.0 > r2.1 || r1.1 < r2.0);
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let result_1: (usize, usize) = data
        .lines()
        .filter_map(|l| parse::parse_pair(l).ok())
        .map(|s| {
            (
                contains(s.1.0, s.1.1) as usize,
                overlaps(s.1.0, s.1.1) as usize,
            )
        })
        .fold((0, 0), |acc, elem| (acc.0 + elem.0, acc.1 + elem.1));

    println!("{:?}", result_1);
}
