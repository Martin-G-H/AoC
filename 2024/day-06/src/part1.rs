use core::panic;
use std::{char, collections::HashMap, i32};

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

pub fn process(_input: &str) -> miette::Result<String> {
    let mut guard: (i32, i32) = (0, 0);
    let mut guard_direction: (i32, i32) = (0, -1);
    let mut positions: HashMap<(i32, i32), char> = HashMap::new();
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

    let mut visited = 1;

    while within_bounds {
        let next_pos = pos_sum(&guard_direction, &guard);
        match positions.get(&next_pos) {
            Some(&val) => match val {
                '#' => guard_direction = rotate(&guard_direction),
                '.' => {
                    positions.insert(next_pos, 'X');
                    guard = next_pos;
                    visited += 1;
                }
                'X' => {
                    guard = next_pos;
                    // dbg!("Old:", next_pos);
                }
                _ => panic!("Unknown Char"),
            },
            None => within_bounds = false,
        }
    }

    let vis = positions.values().filter(|val| **val == 'X').count();

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

    Ok(visited.to_string())
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
