use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let input = read_input()?;

    println!("total contained range: {}", count_contained_range(&input));
    Ok(())
}

fn read_input() -> MyResult<Vec<String>> {
    let input = fs::read_to_string("./input.txt")?;
    let input: Vec<String> = input.trim().split("\n").map(|el| el.to_string()).collect();

    Ok(input)
}

fn count_contained_range(input: &Vec<String>) -> u32 {
    let mut count = 0;

    for el in input.iter() {
        if let Some(sections) = el.split_once(",") {
            let (elf1_sections, elf2_sections) = sections;

            let elf1_section_ids: Vec<u32> = elf1_sections
                .split('-')
                .filter_map(|el| el.parse().ok())
                .collect();
            let elf2_section_ids: Vec<u32> = elf2_sections
                .split('-')
                .filter_map(|el| el.parse().ok())
                .collect();

            let elf1_first_section = elf1_section_ids[0];
            let elf1_last_section = elf1_section_ids[1];

            let elf2_first_section = elf2_section_ids[0];
            let elf2_last_section = elf2_section_ids[1];

            let elf1_section_range = elf1_first_section..=elf1_last_section;
            let elf2_section_range = elf2_first_section..=elf2_last_section;

            if (elf1_section_range.contains(&elf2_first_section)
                && elf1_section_range.contains(&elf2_last_section))
                || (elf2_section_range.contains(&elf1_first_section)
                    && elf2_section_range.contains(&elf1_last_section))
            {
                count += 1;
            }
        }
    }

    count
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
}
