use itertools::Itertools;
use std::collections::HashMap;

fn within_bounds(pos: &(i32, i32), w: i32, h: i32) -> bool {
    (0 <= pos.0) && (pos.0 < w) && (0 <= pos.1) && (pos.1 < h)
}

fn get_antinodes((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> Vec<(i32, i32)> {
    let c = (ax + (ax - bx), ay + (ay - by));
    let d = (bx + (bx - ax), by + (by - ay));
    vec![c, d]
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let w = _input.lines().last().unwrap().chars().count() as i32;
    let h = _input.lines().count() as i32;
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    _input.lines().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, char)| {
            char.ne(&'.').then(|| {
                let y = r as i32;
                let x = c as i32;
                antennas
                    .entry(char)
                    .and_modify(|vec| vec.push((x, y)))
                    .or_insert(vec![(x, y)]);
            });
        });
    });

    antennas.into_iter().for_each(|(_antenna, antenna_pos)| {
        antenna_pos.clone().iter().combinations(2).for_each(|pair| {
            let antinodes = get_antinodes(*pair[0], *pair[1]);
            antinodes
                .iter()
                .filter(|antinode| within_bounds(antinode, w, h))
                .for_each(|antinode| {
                    map.entry(*antinode).or_insert('#');
                });
        });
    });

    let total_antinodes = map.into_iter().count();

    Ok(total_antinodes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
