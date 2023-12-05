use std::{fs, iter::FilterMap};

const STR_DIGITS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part2(&file));
} 

fn part1(file: &str) -> u32 {
    let result = file
        .lines()
        .map(|item| item
            .chars()
            .filter(|n| ('0'..='9').contains(n))
            .collect::<Vec<char>>()
        )
        .map(|item| {
            let a = item.first().unwrap();
            let b = item.last().unwrap_or(a);
            a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()
        })
        .sum::<u32>();
    return result;
}

fn part2(file: &str) -> u32 {
    let result = file
        .lines()
        .map(|item| {
            let c = item.chars().collect::<Vec<_>>();
            return c.iter().enumerate().filter_map(|(idx, chars)| match chars {
                '0'..='9' => Some(chars.to_digit(10).unwrap()),
                _ => STR_DIGITS.iter()
                    .enumerate()
                    .find_map(|(idz, s)| item[idx..].starts_with(s).then_some((idz+1) as u32))
            } 
            ).collect::<Vec<_>>();
        })
        .map(|item| {
                let a = item.first().unwrap();
                let b = item.last().unwrap_or(a);
                a * 10 + b
            }
        )
        .sum::<u32>();
    return result;
}
