use std::fs;

use nom::{IResult, multi::separated_list1, character::complete::{line_ending, digit1, space1}, sequence::{preceded, separated_pair, delimited, tuple}, bytes::{streaming::tag}, number::complete};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn score(&self) -> usize{
        let power = self.numbers.iter().filter(|num| self.winning.contains(&num)).count() as u32;
        if power == 0 {
            0
        } else {
            2usize.pow(power - 1)
        }
    }
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_cards(input: &str) -> IResult<&str, Vec<Card>>{
    let (input, cards) = separated_list1(line_ending, parse_card)(input)?;
    Ok((input, cards))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(tuple((tag("Card"), space1)), digit1, tuple((tag(":"), space1)))(input)?;
    let (input, (w, n)) = separated_pair(parse_nums, tuple((tag(" |"), space1)), parse_nums)(input)?;
    Ok((input, Card{ id: id.parse::<usize>().unwrap(), winning: w, numbers: n}))
}

// 41 48 83 86 17
fn parse_nums(input: &str) -> IResult<&str, Vec<usize>>{
    let (input, num) = separated_list1(space1, digit1)(input)?;
    Ok((input, num.iter().map(|num| num.parse::<usize>().unwrap()).collect()))
}

fn solution(file: &str) -> usize {
    let (_, cards) = parse_cards(file).expect("Should Parse");
    cards.iter().map(|card| card.score()).sum()
}


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 