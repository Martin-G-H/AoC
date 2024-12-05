use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse_ordering(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list0(
        tag("\n"),
        separated_pair(complete::i32, tag("|"), complete::i32),
    )(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list0(tag("\n"), separated_list0(tag(","), complete::i32))(input)
}

fn check_order(num: i32, orders: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
    let res = orders.iter().find(|(_, second)| *second == num);
    match res {
        Some(pair) => {
            update.iter()
        }
        None => {
            return true;
        }
    }
    true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (orders, updates)) = separated_pair(parse_ordering, tag("\n\n"), parse_update)(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    let sum: i32 = updates
        .iter()
        .filter(|&update| update.iter().all(|num| check_order(*num, , &orders, &update)))
        .map(|update| {
            let length = update.len();
            update[length / 2]
        })
        .sum();
    Ok("3".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
