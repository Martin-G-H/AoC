use itertools::Itertools;

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn cmp_word(word: &str) -> bool {
    (word == "XMAS") || (word == "SAMX")
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let grid = _input.lines().collect_vec();
    let mut sum = 0;

    for line in &grid {
        let windows = char_windows(&line, 4);
        sum += windows.filter(|&word| cmp_word(word)).count()
    }

    grid.windows(4).for_each(|window| {
        let len = window[0].len();
        for j in 0..len {
            cmp_word(
                (0..4)
                    .map(|i| window[i].chars().nth(j).unwrap())
                    .collect::<String>()
                    .as_str(),
            )
            .then(|| sum += 1);
        }

        for j in 0..len - 3 {
            cmp_word(
                (0..4)
                    .map(|i| window[i].chars().nth(j + i).unwrap())
                    .collect::<String>()
                    .as_str(),
            )
            .then(|| sum += 1);
        }

        for j in 3..len {
            cmp_word(
                (0..4)
                    .map(|i| window[i].chars().nth(j - i).unwrap())
                    .collect::<String>()
                    .as_str(),
            )
            .then(|| sum += 1);
        }
    });
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
