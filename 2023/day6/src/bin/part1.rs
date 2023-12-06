use std::fs;

use nom::{sequence::{separated_pair, preceded, tuple}, character::complete::{line_ending, space1, digit1}, bytes::complete::tag, multi::separated_list1, IResult};

#[derive(Debug)]
struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn solve(&self) -> usize {
        let acc: usize = (0..=self.time).fold(0, |mut acc, i| {
            let speed = i;
            let rest = self.time - i;
            let dist = speed * rest;
            if dist > self.dist {acc += 1;}
            acc
        });
        acc
    }
}

fn to_usize(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

fn parse_input(input:  &str) -> IResult<&str,Race>{
    let (input, (time, dist)) = separated_pair(
        preceded(tuple((tag("Time:"), space1)), separated_list1(space1, digit1)),
        line_ending,
        preceded(tuple((tag("Distance:"), space1)), separated_list1(space1, digit1))
        )(input)?;
    let new_time = time.iter().fold(String::new(), |mut acc, chunk| {acc += chunk; acc});
    let new_dist = dist.iter().fold(String::new(), |mut acc, chunk| {acc += chunk; acc});
    let race = Race { time: to_usize(&new_time), dist: to_usize(&new_dist) };
    dbg!(&race);
    return Ok((input, race));
}

fn solution(input:  &str) -> usize {
    let (_, race) = parse_input(input).expect("Should parse");
    let sol = race.solve();
    dbg!(sol)
}


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", solution(&file));
}
