use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let input = read_input()?;

    println!("total contained range: {}", count_contained_range(&input));
    println!("total overlapped range: {}", count_overlapped_range(&input));
    Ok(())
}

fn read_input() -> MyResult<Vec<String>> {
    let input = fs::read_to_string("./input.txt")?;
    let input: Vec<String> = input.trim().split("\n").map(|el| el.to_string()).collect();

    Ok(input)
}

fn count<F>(input: &Vec<String>, func: F) -> u32
where
    F: Fn((u32, u32), (u32, u32)) -> bool,
{
    let mut counter = 0;

    for el in input.iter() {
        if let Some(sections) = el.split_once(',') {
            let (elf1_sections, elf2_sections) = sections;

            let elf1_section_ids: Vec<u32> = elf1_sections
                .split('-')
                .filter_map(|el| el.parse().ok())
                .collect();
            let elf2_section_ids: Vec<u32> = elf2_sections
                .split('-')
                .filter_map(|el| el.parse().ok())
                .collect();

            if func(
                (elf1_section_ids[0], elf1_section_ids[1]),
                (elf2_section_ids[0], elf2_section_ids[1]),
            ) {
                counter += 1;
            }
        }
    }

    counter
}

fn count_contained_range(input: &Vec<String>) -> u32 {
    count(input, |elf1, elf2| {
        let elf1_range = elf1.0..=elf1.1;
        let elf_2range = elf2.0..=elf2.1;

        (elf1_range.contains(&elf2.0) && elf1_range.contains(&elf2.1))
            || (elf_2range.contains(&elf1.0) && elf_2range.contains(&elf1.1))
    })
}

fn count_overlapped_range(input: &Vec<String>) -> u32 {
    count(input, |elf1, elf2| {
        let elf1_range = elf1.0..=elf1.1;
        let elf2_range = elf2.0..=elf2.1;

        elf1_range.contains(&elf2.0)
            || elf1_range.contains(&elf2.1)
            || elf2_range.contains(&elf1.0)
            || elf2_range.contains(&elf1.1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_contained_range() {
        let input: Vec<String> = vec!["2-4,6-8", "5-7,7-9", "2-8,3-7", "6-6,4-6"]
            .iter()
            .map(|el| el.to_string())
            .collect();
        assert_eq!(count_contained_range(&input), 2);
    }

    #[test]
    fn test_count_overlapped_range() {
        let input: Vec<String> = vec!["2-4,6-8", "5-7,7-9", "2-8,3-7", "6-6,4-6"]
            .iter()
            .map(|el| el.to_string())
            .collect();
        assert_eq!(count_overlapped_range(&input), 3);
    }
}
