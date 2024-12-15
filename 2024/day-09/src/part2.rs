use core::panic;

use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let input = _input.trim_end();

    let empty_chunks = input
        .chars()
        .enumerate()
        .filter(|(id, _char)| id % 2 == 1)
        .map(|(_, char)| char.to_digit(10).unwrap());

    let files = input
        .chars()
        .enumerate()
        .filter(|(id, _char)| id % 2 == 0)
        .map(|(_, char)| char.to_digit(10).unwrap());

    let mut checksum: u64 = 0;

    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}

