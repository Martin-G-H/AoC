use std::{
    char,
    collections::{HashMap, HashSet},
};

use glam::IVec2;

const DIRECTION: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
];

fn dfs_region(
    lookup: &HashMap<IVec2, char>,
    visited: &mut HashSet<IVec2>,
    pos: IVec2,
    letter: char,
    region: &mut i32,
    edges: &mut i32,
) {
    visited.insert(pos);
    *region += 1;
    for dir in DIRECTION {
        let new_pos = pos + dir;
        if visited.contains(&new_pos) {
            continue;
        }
        let maybe_letter = lookup.get(&new_pos);
        match maybe_letter {
            Some(pos_letter) if *pos_letter == letter => {
                dfs_region(lookup, visited, new_pos, letter, region, edges);
            }
            _ => *edges += 1,
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut lookup: HashMap<IVec2, char> = HashMap::new();
    let mut global_visited: HashSet<IVec2> = HashSet::new();
    let mut visited: HashSet<IVec2> = HashSet::new();

    for (r, line) in _input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            lookup.entry(IVec2::new(c as i32, r as i32)).or_insert(char);
        }
    }

    // dfs_region(&lookup, &mut visited, IVec2::new(0, 0), 'A', &mut 0, &mut 0);
    // return Ok(3.to_string());

    let mut total_sum = 0;

    for (r, line) in _input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            let pos = IVec2::new(c as i32, r as i32);
            if global_visited.contains(&pos) {
                continue;
            }
            let mut region = 0;
            let mut edges = 0;
            dfs_region(&lookup, &mut visited, pos, char, &mut region, &mut edges);
            visited.iter().for_each(|pos| {
                global_visited.insert(*pos);
            });
            visited.clear();
            total_sum += region * edges;
        }
    }

    Ok(total_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("140", process(input)?);
        Ok(())
    }
}
