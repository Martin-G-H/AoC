use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list0,
    IResult,
};

fn calc(number: u64, rounds: i32) -> Vec<u64> {
    let mut stones: Vec<u64> = vec![number];
    let mut count: Vec<u64> = Vec::new();
    for _i in 0..rounds {
        let mut new_stones: Vec<u64> = Vec::new();
        stones.into_iter().for_each(|stone| match stone {
            0 => new_stones.push(1),
            _ if stone.to_string().len() % 2 == 0 => {
                let stone_str = stone.to_string();
                let (stone_a, stone_b) = stone_str.split_at(stone_str.len() / 2);
                new_stones.push(stone_a.parse().unwrap());
                new_stones.push(stone_b.parse().unwrap());
            }
            _ => new_stones.push(stone * 2024),
        });
        count.push(new_stones.len() as u64);
        stones = new_stones;
    }

    return count;
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(space1, complete::u64)(input)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, mut stones) = parse(_input).map_err(|err| miette!("error parsing {}", err))?;

    let calc0 = calc(0, 25);

    let run_length = 25;

    let mut total_sum = 0;

    for i in 0..run_length {
        let mut new_stones: Vec<u64> = Vec::new();
        stones.into_iter().for_each(|stone| match stone {
            0 => total_sum += calc0[run_length - 1 - i],
            _ if stone.to_string().len() % 2 == 0 => {
                let stone_str = stone.to_string();
                let (stone_a, stone_b) = stone_str.split_at(stone_str.len() / 2);
                new_stones.push(stone_a.parse().unwrap());
                new_stones.push(stone_b.parse().unwrap());
            }
            _ => new_stones.push(stone * 2024),
        });
        stones = new_stones;
    }

    total_sum += stones.len() as u64;
    dbg!(total_sum);

    Ok(2.to_string())
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
