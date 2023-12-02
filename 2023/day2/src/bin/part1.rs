use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solution(&file));
} 

fn solution(file: &str) -> i32 {
    let cap: HashMap<&str,i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let result = file
        .lines()
        .filter_map(|line| {
            let mut it = line.split(": ");
            let mut game = it.next().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            let _round = it
                .next()
                .unwrap()
                .split("; ")
                .for_each(|draw| {
                    let mut prep = cap.clone();
                    let dice = draw.split(", ");
                    dice.for_each(|die| {
                        let mut d = die.split(" "); 
                        let num = d.next().unwrap();
                        *prep.get_mut(&d.next().unwrap()).unwrap() -= num.parse::<i32>().unwrap();
                    });
                    prep.iter().for_each(|(_, n)| {
                        if *n < 0 {
                            game = 0;
                        }
                    });
                });
            Some(game)
        })
        .sum();
    result
}
