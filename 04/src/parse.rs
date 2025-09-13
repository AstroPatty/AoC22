use nom::character;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

pub(crate) fn parse_sections(data: &str) -> IResult<&str, (usize, usize)> {
    let parse = separated_pair(
        character::complete::digit1,
        character::complete::char('-'),
        character::complete::digit1,
    );

    let mut result = map_res(parse, |s: (&str, &str)| {
        Ok::<(usize, usize), String>((s.0.parse::<usize>().unwrap(), s.1.parse::<usize>().unwrap()))
    });
    result.parse(data)
}

pub(crate) fn parse_pair(data: &str) -> IResult<&str, ((usize, usize), (usize, usize))> {
    separated_pair(
        parse_sections,
        character::complete::char(','),
        parse_sections,
    )
    .parse(data)
}
