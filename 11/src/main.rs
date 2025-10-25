use std::cell::RefCell;
use std::cmp::Reverse;
use std::fs;

mod worry;
fn p1_stress(val: &usize) -> usize {
    val / 3
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut monkeys_1 = worry::parse_monkeys(&data).unwrap().1;
    let mut monkeys_2 = monkeys_1.clone();

    let mut monkeys_p1 = monkeys_1.drain(0..).map(RefCell::new).collect();
    let gcd = monkeys_2.iter().fold(1, |acc, m| acc * m.test.0);
    let mut monkeys_p2 = monkeys_2.drain(0..).map(RefCell::new).collect();

    let p2_stress = |val: &usize| val % gcd;

    monkeys_p1 = worry::do_turns(monkeys_p1, 20, p1_stress);
    monkeys_p2 = worry::do_turns(monkeys_p2, 10000, p2_stress);

    let mut inspections_p1: Vec<usize> = monkeys_p1.iter().map(|m| m.borrow().n).collect();
    let mut inspections_p2: Vec<usize> = monkeys_p2.iter().map(|m| m.borrow().n).collect();

    inspections_p1.sort_by_key(|&val| Reverse(val));
    inspections_p2.sort_by_key(|&val| Reverse(val));

    let result_p1 = inspections_p1.get(0).unwrap() * inspections_p1.get(1).unwrap();
    let result_p2 = inspections_p2.get(0).unwrap() * inspections_p2.get(1).unwrap();
    println!("{:?}", result_p1);
    println!("{:?}", result_p2);
}
