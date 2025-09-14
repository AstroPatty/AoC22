use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character;
use nom::combinator::{map, peek};
use nom::multi::{count, many_till, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

type Row = Vec<Option<char>>;

#[derive(Debug)]
pub struct Problem {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<u32>,
}

fn box_parser(data: &str) -> IResult<&str, Option<char>> {
    let box_parser = delimited(
        character::complete::char('['),
        character::complete::anychar,
        character::complete::char(']'),
    );
    let box_char_parser = map(box_parser, Some);
    let whitespace_parser = count(character::complete::char(' '), 3);
    let no_box_parser = map(whitespace_parser, |_| None);
    let mut slot_parser = terminated(
        alt((box_char_parser, no_box_parser)),
        character::complete::one_of(" \n"),
    );
    slot_parser.parse(data)
}

fn tags(data: &str) -> IResult<&str, &str> {
    alt((tag("move"), tag("from"), tag("to"))).parse(data)
}
fn parse_move_commands(data: &str) -> IResult<&str, Vec<u32>> {
    let delim = (
        character::complete::multispace0,
        tags,
        character::complete::multispace0,
    );
    let move_parser = separated_list1(delim, character::complete::u32);
    terminated(
        preceded(delim, move_parser),
        character::complete::multispace0,
    )
    .parse(data)
}

pub(crate) fn setup_parser(data: &str) -> IResult<&str, Problem> {
    let number_parser = delimited(
        character::complete::multispace0,
        character::complete::digit1,
        character::complete::multispace0,
    );
    let numlist_parser = many1(number_parser);
    let numline_parser = preceded(character::complete::multispace0, numlist_parser);
    let stack_parser = many_till(box_parser, numline_parser).map(|res| {
        let boxes = res.0;
        let n_stacks = res.1.len();
        let mut stacks = Vec::new();
        for _ in 0..n_stacks {
            stacks.push(Vec::new())
        }
        for (i, elem) in boxes.iter().rev().enumerate() {
            let stack_n = n_stacks - 1 - (i % n_stacks);
            if let Some(c) = elem {
                stacks[stack_n].push(*c)
            }
        }
        stacks
    });
    let setup_parser = (
        stack_parser,
        preceded(character::complete::multispace0, parse_move_commands),
    );
    setup_parser
        .map(|(stacks, moves)| Problem { stacks, moves })
        .parse(data)
}
