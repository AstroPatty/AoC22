use nom::{IResult, Parser};
use std::fs;
use std::u64;

use nom::character::complete::multispace1;
use nom::character::complete::u64;
use nom::multi::separated_list1;
use std::io::Result;

fn get_numbers(s: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(nom::character::complete::newline, u64).parse(s)
}

fn get_all_numbers(s: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(multispace1, get_numbers).parse(s)
}

fn get_max_n(val: &str, n: usize) -> Vec<u64> {
    let mut totals = get_all_totals(val);
    totals.truncate(n);
    totals
}

fn get_all_totals(val: &str) -> Vec<u64> {
    let output = get_all_numbers(val).unwrap();
    let values = output.1;
    let mut totals: Vec<u64> = values
        .iter()
        .map(|list| list.iter().fold(0, |acc, &num| acc + num))
        .collect();

    totals.sort();
    totals.reverse();

    totals
}

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let s = get_max_n(&f, 3);
    let res = s.iter().fold(0, |acc, &num| acc + num);

    println!("{:?}", res);
}
