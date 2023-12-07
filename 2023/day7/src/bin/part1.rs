use std::{fs, collections::HashMap, cmp::Ordering};

use nom::{sequence::{separated_pair, preceded, tuple}, character::complete::{line_ending, space1, digit1, alphanumeric1}, bytes::complete::tag, multi::separated_list1, IResult};

use itertools::{self, Itertools};

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    CT = 10,
    CJ = 11,
    CQ = 12,
    CK = 13,
    CA = 14
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            _ if (self.eval() as usize) > (other.eval() as usize) => return Ordering::Greater,
            _ if (self.eval() as usize) < (other.eval() as usize) => return Ordering::Less,
            _ => {
                for (id, card) in self.cards.iter().enumerate() {
                    let m = *card as usize;
                    match m {
                        m if m > other.cards[id] as usize => return Ordering::Greater,
                        m if m < other.cards[id] as usize => return Ordering::Less,
                        _ => continue
                    };
                };
                return Ordering::Equal;
            }
        }
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Hand {
    fn eval(&self) -> HandType{
        let mut unique = self.cards.iter().unique().copied().collect::<Vec<Card>>();
        match unique.len() {
            1 => HandType::Five,
            2 => {
                match self.cards.iter().filter(|&card| *card == *unique.iter().next().unwrap()).count() {
                    1 | 4 => HandType::Four,
                    2 | 3 => HandType::Full,
                    _ => panic!()
                }
            },
            3 => {
                let m = unique.into_iter().map(|card| self.cards.iter().filter(|&other| *other == card).count()).max().unwrap();
                match m {
                    3 => HandType::Three, 
                    2 => HandType::Two,
                    _ => panic!()
                }
            },
            4 => HandType::One,
            5 => HandType::High,
            _ => panic!()
        }
    }
}


fn get_card(input: char) -> Card {
    match input {
        '2' => Card::C2,
        '3' => Card::C3,
        '4' => Card::C4,
        '5' => Card::C5,
        '6' => Card::C6,
        '7' => Card::C7,
        '8' => Card::C8,
        '9' => Card::C9,
        'T' => Card::CT,
        'J' => Card::CJ,
        'Q' => Card::CQ,
        'K' => Card::CK,
        'A' => Card::CA,
        _ => panic!("Shouldnt get any other input")
    }
}

fn to_usize(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

// 32T3K 765
fn parse_hand(input:  &str) -> IResult<&str, Hand> {
    let (input, (labels, bid)) = separated_pair(alphanumeric1, space1, digit1)(input)?;
    let mut hand: Vec<Card> = Vec::new();
    for card in labels.chars() {
        hand.push(get_card(card))
    };
    Ok((input, Hand {cards: hand, bid: to_usize(bid)}))
}

fn parse_input(input:  &str) -> IResult<&str,Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn solution(input:  &str) -> usize {
    let (_, hands) = parse_input(input).expect("Parse");
    let sorted = hands.iter().sorted();
    let sum: usize = sorted.enumerate().map(|(id, a)| (id + 1) * a.bid).sum();
    dbg!(sum)
}


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", solution(&file));
}