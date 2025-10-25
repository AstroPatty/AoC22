mod check;
mod parse;
use ndarray::Array2;
use std::fs;

fn main() {
    let strdata = fs::read_to_string("data.txt").unwrap();
    let arr = parse::parse_lines(strdata);
    let visible = check::check_all(&arr);
    let total = visible.flatten().iter().fold(0, |acc, &elem| {
        if elem {
            return acc + 1;
        }
        acc
    });
    let scores = check::count_all(&arr);

    //println!("{:?}", total);
    println!("{:?}", scores.iter().max());
}
