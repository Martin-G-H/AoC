use std::{fs, string, net};

#[derive(Debug, Clone, Copy)]
struct Group {
    n: u32,
    r: u32,
    c: u32,
    len: u32,
    symbol: char
}

impl Group {
    fn reset(mut self) {
        self.n = 0;
        self.r = 0;
        self.c = 0;
        self.len = 0;
        self.symbol = '.';
    }
}


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 

fn is_symbol(val: &char) -> bool {
    *val != '.' && !val.is_digit(10)
}

fn parse(file: &str) -> Option<Vec<Group>> {
    let mut groups: Vec<Group> = Vec::new();
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect()).collect();

    grid
        .iter()
        .enumerate()
        .for_each(|(r, row)| {
            let mut group: Group = Group {n: 0, r: 0, c: 0, len: 0, symbol: '.'};
            let mut curr = String::new();
            row
                .iter()
                .enumerate()
                .for_each(|(c, val)| {
                    match val {
                        _ if is_symbol(val) => group.symbol.clone_from(val),
                        _ if val.is_digit(10) => curr += &val.to_string(),
                        _ => if !curr.is_empty() {
                            group.n = curr.parse::<u32>().unwrap();
                            group.len = curr.len() as u32;
                            group.r = r as u32;
                            group.c = c as u32;
                            groups.push(group.clone());
                            group.reset();
                        } ,
                    }
                });
            dbg!(group);
            if group.n != 0 {
                groups.push(group.clone());
            }
        });
    Some(groups)
}

fn solution(file: &str) -> u32 {
    let mut groups = parse(file);
    dbg!(groups);
    todo!();
}