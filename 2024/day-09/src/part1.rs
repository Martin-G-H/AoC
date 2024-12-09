use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let input = _input.trim_end();

    let disc_length: usize = input
        .trim_end()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .sum();

    let mut empty_block: bool = false;
    let mut index: i32 = 0;
    let disc: Vec<i32> = input
        .chars()
        .into_iter()
        .flat_map(|char| {
            let num = char.to_digit(10).unwrap() as usize;
            let ret: Vec<_>;
            match empty_block {
                true => ret = vec![-1; num],
                false => {
                    ret = vec![index as i32; num];
                    index += 1;
                }
            };
            empty_block = !empty_block;
            ret
        })
        .collect();

    let empty_slots = disc.iter().filter(|num| **num == -1).count();

    let empty = disc
        .iter()
        .rev()
        .filter(|num| **num != -1)
        .take(empty_slots);

    // let checksum = disc.into_iter().enumerate().map(|(idx, num)| match num {
    //     -1 => {
    //         let (index, num) = rest.next().unwrap();
    //         if index == idx {
    //             todo!();
    //         }
    //         num
    //     }
    //     _ => num,
    // });

    dbg!(disc);
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
