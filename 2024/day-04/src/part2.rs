use itertools::Itertools;

fn g_get(grid: &Vec<&str>, r: usize, c: usize) -> char {
    grid[r].chars().nth(c).unwrap()
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let grid = _input.lines().collect_vec();
    let mut sum = 0;

    let width = grid.len();
    let height = grid[0].len();
    for r in 1..width - 1 {
        for c in 1..height - 1 {
            if g_get(&grid, r, c) == 'A' {
                let mut back = String::from("");
                for n in -1..=1 {
                    back.push(g_get(
                        &grid,
                        (r as i32 + n) as usize,
                        (c as i32 - n) as usize,
                    ));
                }
                let mut forw = String::from("");
                for n in -1..=1 {
                    forw.push(g_get(
                        &grid,
                        (r as i32 + n) as usize,
                        (c as i32 + n) as usize,
                    ));
                }

                if (forw == "MAS" || forw == "SAM") && (back == "MAS" || back == "SAM") {
                    sum += 1;
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
