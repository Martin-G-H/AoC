use std::fs;

use nom::{IResult, multi::separated_list1, character::complete::{line_ending, digit1}, sequence::{preceded, separated_pair, delimited}, bytes::{streaming::tag, complete}, combinator::map_res};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_cards(input: &str) -> IResult<&str, Vec<Card>>{
    let cards: Vec<Card> = input.lines().map(|line| {
        let (input , card) = parse_card(line).unwrap();
        card
    }).collect();
    Ok((input, cards))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(tag("Card "), digit1, tag(": "))(input)?;
    dbg!(input, id);
    let (input, (w, n)) = separated_pair(parse_nums, tag(" | "), parse_nums)(input)?;
    dbg!(input, w, n);
    //Ok((input, Card{ id: id.parse::<usize>().unwrap(), winning: winning, numbers:numbers}))
    todo!()
}

// 41 48 83 86 17
fn parse_nums(input: &str) -> IResult<&str, Vec<usize>>{
    dbg!(input);
    let (input, num) = separated_list1(tag(" "), parse_num)(input)?;
    Ok((input, num))
}

// 41
fn parse_num(input: &str) -> IResult<&str, usize> {
    dbg!(input);
    Ok((input, input.parse::<usize>().unwrap()))
}

fn solution(file: &str) -> usize {
    let cards = parse_cards(file).expect("Should Parse");
    dbg!(cards);
    todo!()
}


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 