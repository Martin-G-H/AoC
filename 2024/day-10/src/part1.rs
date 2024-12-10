use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

const DIRS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
];

fn find_peaks_rec(pos_map: &HashMap<IVec2, i32>, pos: IVec2, curr_heigh: i32) -> Vec<IVec2> {
    if curr_heigh == 9 {
        return vec![pos];
    }
    let mut peaks: Vec<IVec2> = Vec::new();
    for dir in DIRS {
        let next_pos = pos + dir;
        if pos_map.contains_key(&next_pos) {
            let height = pos_map.get(&next_pos).unwrap();
            if height - curr_heigh == 1 {
                peaks.append(&mut find_peaks_rec(pos_map, next_pos, *height));
            }
        }
    }
    peaks
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut position_map: HashMap<IVec2, i32> = HashMap::new();
    _input.lines().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, char)| {
            let key = IVec2::new(c as i32, r as i32);
            let value = char.to_digit(10).unwrap() as i32;
            position_map.entry(key).or_insert(value);
        })
    });

    dbg!(position_map.len());

    let res: i32 = position_map
        .clone()
        .into_iter()
        .filter(|(_vec, height)| *height == 0)
        .map(|(pos, height)| {
            find_peaks_rec(&position_map, pos, height)
                .iter()
                .unique()
                .count() as i32
        })
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
