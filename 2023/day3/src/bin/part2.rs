use std::{fs, string, net, char, usize, ops::Add};

#[derive(Debug, Clone)]
struct Group {
    num: Vec<Num>,
    symbol: char,
}

#[derive(Debug, Clone, Copy)]
struct Num {
    pos: Pos,
    n: usize,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    r: usize,
    c: usize
}

impl Pos {
    fn adjacent(self, r: usize, c: usize) -> bool{
        (self.r as isize - r as isize).abs() <= 1 
        && (self.c as isize - c as isize).abs() <= 1
    }
}

impl Group {
    fn set_symbol (&mut self, val: &char) {
        self.symbol = *val;
    }

    fn get_number (self) -> usize{
        self.num.iter().fold(String::new(), |acc, num| acc.add(&num.n.to_string())).parse::<usize>().unwrap()
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 

fn parse(file: &str) -> usize {
    let mut sum = 0;
    let mut groups: Vec<Group> = Vec::new();
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect()).collect();

    grid
        .iter()
        .enumerate()
        .for_each(|(r, row)| {
            let mut numbers: Vec<Num> = Vec::new();
            row
                .iter()
                .enumerate()
                .for_each(|(c, val)| {
                    match val {
                        val if val.is_ascii_digit() => {numbers.push(Num { n: val.to_digit(10).unwrap() as usize, pos: Pos { r: r, c: c }})},
                        _ if !numbers.is_empty() && !val.is_ascii_digit() => {groups.push(Group { num: numbers.clone(), symbol: '.' }); numbers.clear()},
                        _ => {}
                    }
                });
            if !numbers.is_empty() {groups.push(Group { num: numbers.clone(), symbol: '.' }); numbers.clear()}
        });

    for (r, row) in grid.iter().enumerate() {
        for (c, val) in row.iter().enumerate().filter(|(_, &val)| {val == '*'}) {
            let gears: Vec<&Group> = groups.iter().filter(|group| group.num.iter().any(|num| num.pos.adjacent(r, c))).collect();
            if gears.len() == 2 {
                sum += gears.iter().map(|&group| group.clone().get_number()).product::<usize>();
            }
        }
    }
    dbg!(sum)
}

fn solution(file: &str) -> usize {
    let groups = parse(file);
    return groups;
}