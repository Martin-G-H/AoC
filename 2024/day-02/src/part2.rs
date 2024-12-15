use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let res = _input.lines().filter(|line| {
        let it = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        it.clone().enumerate().any(|(index, _num)| {
            let mut new = it.clone().collect_vec();
            new.remove(index);
            let limits = new
                .clone()
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| (a - b).abs() <= 3);
            let inc = new
                .clone()
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| (a - b) < 0);
            let dec = new
                .clone()
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| (a - b) > 0);
            limits & (inc | dec)
        })
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
