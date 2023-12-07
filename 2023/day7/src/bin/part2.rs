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
    CJ = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    CT = 10,
    CQ = 11,
    CK = 12,
    CA = 13
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

        let modified = self.cards.iter().filter(|&card| *card != Card::CJ).collect::<Vec<_>>();
        let jokers = self.cards.len() - modified.len();

        let unique = get_unique(&modified);
        match unique.len() {
            1 => HandType::Five,
            2 => {
                match unique.values().max().unwrap() + jokers{
                    4 => HandType::Four,
                    3 => HandType::Full,
                    _ => panic!()
                }
            },
            3 => {
                let m = unique.values().max().unwrap();
                match m + jokers {
                    3 => HandType::Three, 
                    2 => HandType::Two,
                    _ => panic!()
                }
            },
            4 => HandType::One,
            5 => HandType::High,
            0 => HandType::Five,
            _ => panic!("unique len")
        }
    }
    
    fn get_unique(&self) -> HashMap<Card, usize> {
        let mut map: HashMap<Card, usize> = HashMap::new();
        self.cards.iter().for_each(|card| {
            map.entry(*card).and_modify(|a| *a += 1).or_insert(1);
        });
        map
    }
    
    fn get_best(&self) -> (Card, usize) {
        self.get_unique().iter().fold((Card::CJ, usize::MIN), |mut acc, (key, val)| {
            if val > &acc.1 && *key != Card::CJ { acc = (*key, *val)}
            acc
        })
    }
}

fn get_unique(cards: &Vec<&Card>) -> HashMap<Card, usize> {
    let mut map: HashMap<Card, usize> = HashMap::new();
    cards.iter().for_each(|&card| {
        map.entry(*card).and_modify(|a| *a += 1).or_insert(1);
    });
    map
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
    sorted.enumerate().map(|(id, a)| ((id + 1) * a.bid)).sum()
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", solution(&file));
}