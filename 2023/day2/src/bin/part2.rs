use core::num;
use std::{fs, collections::HashMap};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 

fn solution(file: &str) -> u64 {
    let result = file
        .lines()
        .filter_map(|line| {
            let mut it = line.split(": ");
            let _game = it.next();
            let mut product = u64::MAX;
            let mut prep: HashMap<&str, u64> = HashMap::from([
                ("red", 0),
                ("blue", 0),
                ("green", 0),
            ]);
            let _round = it
                .next()
                .unwrap()
                .split("; ")
                .for_each(|draw| {
                    let dice = draw.split(", ");
                    dice.for_each(|die| {
                        let mut d = die.split(" "); 
                        let num = d.next().unwrap().parse::<u64>().unwrap();
                        let col = d.next().unwrap();
                        if prep[&col] < num {
                            *prep.get_mut(&col).unwrap() = num;
                        }
                    });
                });
            let mut p = 1;
            prep.values().for_each(|val| p *= val);
            product = product.min(p);
            Some(product)
        })
        .sum();
    result
}
