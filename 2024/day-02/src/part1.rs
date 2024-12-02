use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let res = _input.lines().filter(|line| {
        let it = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        let limits = it.clone().tuple_windows().all(|(a, b)| (a - b).abs() <= 3);
        let inc = it.clone().tuple_windows().all(|(a, b)| (a - b) < 0);
        let dec = it.clone().tuple_windows().all(|(a, b)| (a - b) > 0);
        dbg!(limits, inc, dec);
        limits & (inc | dec)
    });

    Ok(res.count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
