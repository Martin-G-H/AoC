use core::panic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let input = _input.trim_end();

    let mut empty_block: bool = false;
    let mut index: i32 = 0;
    let disc: Vec<i32> = input
        .chars()
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

    let mut fillers = disc
        .clone()
        .into_iter()
        .rev()
        .take(empty_slots)
        .filter(|num| *num != -1);

    let mut checksum: u64 = 0;
    for i in 0..(disc.len() - empty_slots) {
        match disc.get(i) {
            Some(val) => match val {
                -1 => {
                    let new = fillers.next().unwrap() as u64;
                    checksum += i as u64 * new;
                }
                _ => checksum += i as u64 * *val as u64,
            },
            None => panic!("sdadas"),
        }
    }
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
