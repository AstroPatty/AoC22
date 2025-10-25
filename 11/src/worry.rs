use nom::bytes::complete::tag;
use nom::bytes::complete::{take_till, take_until};
use nom::character;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub inspection_op: Rc<dyn Fn(usize) -> usize>,
    pub test: (usize, usize, usize),
    pub n: usize,
}

impl Monkey {
    pub fn take_turn(&mut self, stress_manager: &impl Fn(&usize) -> usize) -> Vec<(usize, usize)> {
        self.n += self.items.len();
        let output = self
            .items
            .iter()
            .map(|&item| (self.inspection_op)(item))
            .map(|item| stress_manager(&item))
            .map(|val| {
                if val % self.test.0 == 0 {
                    (self.test.1, val)
                } else {
                    (self.test.2, val)
                }
            })
            .collect();
        self.items = Vec::new();
        output
    }
    fn push_item(&mut self, item: usize) {
        self.items.push(item)
    }
}

pub fn do_turns(
    monkeys: Vec<RefCell<Monkey>>,
    n: usize,
    stress_manager: impl Fn(&usize) -> usize,
) -> Vec<RefCell<Monkey>> {
    for _ in 0..n {
        for monkey in monkeys.iter() {
            let output = monkey.borrow_mut().take_turn(&stress_manager);
            for (m, v) in output {
                monkeys.get(m).unwrap().borrow_mut().push_item(v);
            }
        }
    }
    monkeys
}
pub fn parse_monkeys(data: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(character::complete::multispace0, parse_monkey).parse(data)
}

pub fn parse_monkey(data: &str) -> IResult<&str, Monkey> {
    let start_ = preceded(character::complete::multispace0, parse_starting_items);
    let start = preceded(take_until("\n"), start_);

    let op = preceded(character::complete::multispace0, parse_operation);
    let test = preceded(character::complete::multispace0, parse_test);
    (start, op, test).parse(data).map(|(rem, vals)| {
        let m = Monkey {
            items: vals.0,
            inspection_op: vals.1,
            test: vals.2,
            n: 0,
        };
        (rem, m)
    })
}

fn parse_starting_items(data: &str) -> IResult<&str, Vec<usize>> {
    let list_parser = separated_list1(tag(", "), character::complete::usize);
    preceded(take_till(|c: char| c.is_digit(10)), list_parser).parse(data)
}

fn parse_operation(data: &str) -> IResult<&str, Rc<dyn Fn(usize) -> usize>> {
    let prefix = take_until("old");
    let lhs = preceded(
        character::complete::multispace0,
        character::complete::alpha1,
    );
    let op = preceded(
        character::complete::multispace0,
        character::complete::one_of("+-*/"),
    );
    let rhs = preceded(
        character::complete::multispace0,
        character::complete::alphanumeric1,
    );
    preceded(prefix, (lhs, op, rhs))
        .parse(data)
        .map(|(rem, vals)| {
            let right = vals.2.parse::<usize>();
            let partial = match vals.1 {
                '+' => |v: usize, r: usize| v + r,
                '-' => |v: usize, r: usize| v - r,
                '*' => |v: usize, r: usize| v * r,
                '/' => |v: usize, r: usize| v / r,
                _ => panic!(),
            };
            if let Ok(r) = right {
                (
                    rem,
                    Rc::new(move |v| partial(v, r)) as Rc<dyn Fn(usize) -> usize>,
                )
            } else {
                (
                    rem,
                    Rc::new(move |v| partial(v, v)) as Rc<dyn Fn(usize) -> usize>,
                )
            }
        })
}

fn parse_test(data: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        parse_single_number,
        parse_single_number,
        parse_single_number,
    )
        .parse(data)
}

fn parse_single_number(data: &str) -> IResult<&str, usize> {
    delimited(
        take_till(|c: char| c.is_digit(10)),
        character::complete::usize,
        character::complete::newline,
    )
    .parse(data)
}
