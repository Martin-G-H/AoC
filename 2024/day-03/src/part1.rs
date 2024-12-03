use itertools::Itertools;
use regex::Regex;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let re = Regex::new(r"(mul\(\d*,\d*\))").unwrap();
    let mut res = vec![];
    for (_, [string]) in re.captures_iter(_input).map(|c| c.extract()) {
        res.push(string);
    }
    dbg!(&res);
    let sum = res.iter().fold(0, |acc, &x| {
        let val = x.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
        let mul = val
            .split(",")
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec();
        acc + mul[0] * mul[1]
    });
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
