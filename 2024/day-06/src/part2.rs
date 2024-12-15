use core::panic;
use std::{
    char,
    collections::{HashMap, HashSet},
    i32,
};

const GUARDS: [char; 4] = ['^', 'v', '<', '>'];

fn identify_guard(char: &char) -> Option<(i32, i32)> {
    for guard in GUARDS {
        if guard == *char {
            match guard {
                '^' => return Some((0, -1)),
                'v' => return Some((0, 1)),
                '<' => return Some((-1, 0)),
                '>' => return Some((1, 0)),
                _ => panic!("Wierd Dir"),
            }
        }
    }
    return None;
}

fn pos_sum(lhs: &(i32, i32), rhs: &(i32, i32)) -> (i32, i32) {
    return (lhs.0 + rhs.0, lhs.1 + rhs.1);
}

fn rotate(dir: &(i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (1, 0) => (0, 1),
        _ => panic!("wierd direction"),
    }
}

fn check_obs(
    mut positions: HashMap<(i32, i32), char>,
    mut guard: (i32, i32),
    mut guard_direction: (i32, i32),
    mut moves: HashSet<((i32, i32), (i32, i32))>,
) -> bool {
    positions.insert(pos_sum(&guard_direction, &guard), 'O');

    let mut is_loop = true;
    while is_loop {
        let next_pos = pos_sum(&guard_direction, &guard);
        if moves.contains(&(guard, next_pos)) {
            return true;
        }
        match positions.get(&next_pos) {
            Some(&val) => match val {
                '#' | 'O' => guard_direction = rotate(&guard_direction),
                '.' => {
                    moves.insert((guard, next_pos));
                    positions.insert(next_pos, 'X');
                    guard = next_pos;
                }
                'X' => {
                    moves.insert((guard, next_pos));
                    guard = next_pos;
                }
                _ => panic!("Unknown Char"),
            },
            None => is_loop = false,
        }
    }
    return false;
}

pub fn process(_input: &str) -> miette::Result<String> {
    let mut guard: (i32, i32) = (0, 0);
    let mut obs: Vec<(i32, i32)> = Vec::new();
    let mut guard_direction: (i32, i32) = (0, -1);
    let mut positions: HashMap<(i32, i32), char> = HashMap::new();
    let mut moves: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let _grid = _input.lines().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, char)| {
            //convert into (X,Y) ~(C,R)
            let pos = (c.try_into().unwrap(), r.try_into().unwrap());
            positions.insert(pos, char);
            match identify_guard(&char) {
                Some(dir) => {
                    guard_direction = dir;
                    guard = pos;
                    positions.insert(pos, 'X');
                }
                None => (),
            }
        });
    });

    let mut within_bounds = true;

    while within_bounds {
        let next_pos = pos_sum(&guard_direction, &guard);
        match positions.get(&next_pos) {
            Some(&val) => match val {
                '#' => guard_direction = rotate(&guard_direction),
                '.' => {
                    if check_obs(
                        positions.clone(),
                        guard.clone(),
                        guard_direction.clone(),
                        moves.clone(),
                    ) {
                        obs.push(next_pos);
                    }

                    moves.insert((guard, next_pos));
                    positions.insert(next_pos, 'X');
                    guard = next_pos;
                }
                'X' => {
                    moves.insert((guard, next_pos));
                    guard = next_pos;
                }
                _ => panic!("Unknown Char"),
            },
            None => within_bounds = false,
        }
    }

    // DEBUG
    // let mut s: [[char; 10]; 10] = [['.'; 10]; 10];
    // for ((x, y), val) in positions {
    //     s[y as usize][x as usize] = val;
    // }
    //
    // for x in s {
    //     let mut d: String = String::new();
    //     for y in x {
    //         d.push(y);
    //     }
    //     dbg!(d);
    // }

    Ok(obs.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
