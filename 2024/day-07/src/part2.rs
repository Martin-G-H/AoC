use itertools::min;
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

fn get_closest(goal: i64, a: i64, b: i64, c: i64) -> i64 {
    let acmp = (goal - a).abs();
    let bcmp = (goal - b).abs();
    let ccmp = (goal - c).abs();

    let vec = vec![acmp, bcmp, ccmp];

    match min(vec.iter()) {
        Some(res) if *res == acmp => a,
        Some(res) if *res == bcmp => b,
        Some(res) if *res == ccmp => c,
        None | Some(_) => panic!("wierd comparison"),
    }
}

fn concat(vec: &[i64]) -> i64 {
    vec.iter()
        .fold("".to_string(), |acc, elem| acc + (&elem.to_string()))
        .parse::<i64>()
        .unwrap()
}

fn calc_rec(goal: i64, first: i64, numbers: &[i64]) -> i64 {
    // dbg!(first, numbers);
    let second = numbers[0];

    let res_add = first + second;
    let res_mul = first * second;
    let vec = vec![first, second];
    // dbg!(first, second);
    let res_cat = concat(&vec);
    // dbg!(res_cat);

    if numbers.len() > 1 {
        let send = numbers.get(1..).unwrap();
        let ret_add = calc_rec(goal, res_add, send);
        let ret_mul = calc_rec(goal, res_mul, send);
        let ret_cat = calc_rec(goal, res_cat, send);
        return get_closest(goal, ret_add, ret_mul, ret_cat);
    }
    // dbg!(get_closest(goal, res_add, res_mul), res_add, res_mul);
    get_closest(goal, res_add, res_mul, res_cat)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, equations) = parse(_input).map_err(|e| miette!("parse failed {}", e))?;

    let res: i64 = equations
        .iter()
        .filter_map(|(goal, numbers)| {
            let closest = calc_rec(*goal, numbers[0], &numbers[1..]);
            // dbg!("-------------------", &closest, goal);
            closest.eq(goal).then_some(goal)
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
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
