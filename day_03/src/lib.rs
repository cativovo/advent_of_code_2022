use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn run() -> MyResult<()> {
    let reader = read_input()?;
    let priorities = get_priorities(reader)?;
    let sum: u32 = calculate_priorities_sum(priorities);

    println!("Priority total: {sum}");

    Ok(())
}

fn read_input() -> MyResult<BufReader<File>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn in_both_compartments(items: &str, to_find: char) -> bool {
    let half = items.len() / 2;
    let first_compartment = &items[..half];
    let second_compartment = &items[half..];

    first_compartment.contains(to_find) && second_compartment.contains(to_find)
}

fn get_priority(item: char) -> u8 {
    let items = ALPHABET.to_string() + &ALPHABET.to_uppercase();

    match items.find(item) {
        Some(index) => (index as u8) + 1,
        None => 0,
    }
}

fn get_priorities(reader: BufReader<File>) -> MyResult<Vec<u32>> {
    let mut priorities: Vec<u32> = vec![];

    for line in reader.lines() {
        let line = line?;

        for c in line.chars() {
            if in_both_compartments(&line, c) {
                priorities.push(get_priority(c) as u32);
                break;
            }
        }
    }

    Ok(priorities)
}

fn calculate_priorities_sum(priorities: Vec<u32>) -> u32 {
    priorities.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_both_compartments() {
        assert!(in_both_compartments("asdfjkls", 's'));
        assert!(!in_both_compartments("abcdefgj", 'a'));
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
    }

    #[test]
    fn test_calculate_priorities_sum() {
        assert_eq!(calculate_priorities_sum(vec![1, 2, 3]), 6);
    }
}
