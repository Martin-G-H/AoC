use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use miette::miette;

fn parse_equation(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(
        complete::i64,
        tag(": "),
        separated_list1(tag(" "), complete::i64),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>> {
    separated_list1(line_ending, parse_equation)(input)
}

fn get_closest(goal: i64, lhs: i64, rhs: i64) -> i64 {
    let lhcmp = (goal - lhs).abs();
    let rhcmp = (goal - rhs).abs();

    match goal {
        _ if lhcmp < rhcmp || (lhcmp == rhcmp) => lhs,
        _ => rhs,
    }
}

fn calc_rec(goal: i64, first: i64, numbers: &[i64]) -> i64 {
    // dbg!(first, numbers);
    let second = numbers[0];

    let res_add = first + second;
    let res_mul = first * second;

    if numbers.len() > 1 {
        let send = numbers.get(1..).unwrap();
        let ret_add = calc_rec(goal, res_add, send);
        let ret_mul = calc_rec(goal, res_mul, send);
        return get_closest(goal, ret_add, ret_mul);
    }
    // dbg!(get_closest(goal, res_add, res_mul), res_add, res_mul);
    get_closest(goal, res_add, res_mul)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, equations) = parse(_input).map_err(|e| miette!("parse failed {}", e))?;

    let res: i64 = equations
        .iter()
        .filter_map(|(goal, numbers)| {
            let closest = calc_rec(*goal, numbers[0], &numbers[1..]);
            // dbg!("-------------------", &closest, goal);
            return closest.eq(goal).then_some(goal);
        })
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
