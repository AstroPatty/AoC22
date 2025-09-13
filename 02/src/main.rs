use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Throw {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl TryFrom<usize> for Throw {
    type Error = String;
    fn try_from(val: usize) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Throw::Rock),
            1 => Ok(Throw::Paper),
            2 => Ok(Throw::Scissors),
            _ => Err(format!("Bad Data: {}", val)),
        }
    }
}

enum GameResult {
    Loss,
    Win,
    Draw,
}

fn get_game(other: Throw, result: GameResult) -> Game {
    let other_val = other.clone() as i32;

    let my_throw_val: usize = match result {
        GameResult::Loss => ((other_val - 1 + 3) % 3) as usize,
        GameResult::Win => (other.clone() as usize + 1) % 3,
        GameResult::Draw => other.clone() as usize,
    };

    let my_throw = Throw::try_from(my_throw_val).unwrap();
    return Game {
        other: other,
        yours: my_throw,
    };
}

#[derive(Debug)]
struct Game {
    other: Throw,
    yours: Throw,
}

fn parse_games(s: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game).parse(s)
}

fn parse_game_results(s: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game_result).parse(s)
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let result = separated_pair(
        nom::character::complete::anychar,
        nom::character::complete::space0,
        nom::character::complete::anychar,
    )
    .parse(s)?;
    let remainder = result.0;
    let chars = result.1;
    let other = throw_from_char(chars.0);
    let yours = throw_from_char(chars.1);

    if other.is_none() || yours.is_none() {
        return Err(nom::Err::Failure(nom::error::make_error(
            remainder,
            nom::error::ErrorKind::Fail,
        )));
    };
    return Ok((
        remainder,
        Game {
            other: other.unwrap(),
            yours: yours.unwrap(),
        },
    ));
}

fn parse_game_result(s: &str) -> IResult<&str, Game> {
    let result = separated_pair(
        nom::character::complete::anychar,
        nom::character::complete::space0,
        nom::character::complete::anychar,
    )
    .parse(s)?;
    let remainder = result.0;
    let chars = result.1;
    let other = throw_from_char(chars.0);
    let game_result = game_from_char(chars.1);
    if other.is_none() || game_result.is_none() {
        return Err(nom::Err::Failure(nom::error::make_error(
            remainder,
            nom::error::ErrorKind::Fail,
        )));
    };
    let game = get_game(other.unwrap(), game_result.unwrap());
    return Ok((remainder, game));
}

fn game_from_char(item: char) -> Option<GameResult> {
    match item.to_uppercase().next().unwrap() {
        'X' => Some(GameResult::Loss),
        'Y' => Some(GameResult::Draw),
        'Z' => Some(GameResult::Win),
        _ => None,
    }
}

fn throw_from_char(item: char) -> Option<Throw> {
    match item.to_uppercase().next().unwrap() {
        'A' | 'X' => Some(Throw::Rock),
        'B' | 'Y' => Some(Throw::Paper),
        'C' | 'Z' => Some(Throw::Scissors),
        _ => None,
    }
}

fn get_game_total(game: &Game) -> usize {
    let other_value = game.other as usize;
    let your_value = game.yours as usize;

    if other_value == your_value {
        return 3 + your_value + 1;
    } else if your_value == (other_value + 1) % 3 {
        return 6 + your_value + 1;
    }

    return your_value + 1;
}

fn get_total(games: Vec<Game>) -> usize {
    games.iter().fold(0, |acc, game| acc + get_game_total(game))
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let games = parse_game_results(&data).unwrap();
    let result = get_total(games.1);
    println!("{:?}", result)
}
