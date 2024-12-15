use std::collections::HashMap;

use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list0,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(space1, complete::u64)(input)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, stones) = parse(_input).map_err(|err| miette!("error parsing {}", err))?;

    let mut lookup: HashMap<u64, u64> = HashMap::new();

    stones.into_iter().for_each(|stone| {
        lookup
            .entry(stone)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
    });

    let run_length = 75;

    // (0 : 1, 11: 1, 123: 1)
    // (1: 3, 248952: 1)
    // (2024: 3, 248: 1, 952: 1)
    // (20: 3 24: 3, ...)
    for _i in 0..run_length {
        let mut new_lookup: HashMap<u64, u64> = HashMap::new();
        lookup.into_iter().for_each(|(stone, amount)| match stone {
            0 => {
                new_lookup
                    .entry(1)
                    .and_modify(|curr_amount| *curr_amount += amount)
                    .or_insert(amount);
            }
            s if s.to_string().len() % 2 == 0 => {
                let stone_str = s.to_string();
                let (stone_a_str, stone_b_str) = stone_str.split_at(stone_str.len() / 2);
                let stone_a = stone_a_str.parse().unwrap();
                let stone_b = stone_b_str.parse().unwrap();
                new_lookup
                    .entry(stone_a)
                    .and_modify(|curr_amount| *curr_amount += amount)
                    .or_insert(amount);
                new_lookup
                    .entry(stone_b)
                    .and_modify(|curr_amount| *curr_amount += amount)
                    .or_insert(amount);
            }
            s => {
                new_lookup
                    .entry(s * 2024)
                    .and_modify(|curr_amount| *curr_amount += amount)
                    .or_insert(amount);
            }
        });
        lookup = new_lookup;
    }

    let res: u64 = lookup.values().sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 1 10 99 999";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
