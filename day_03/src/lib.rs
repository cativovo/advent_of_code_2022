use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn run() -> MyResult<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut duplicate_items: Vec<char> = vec![];

    let mut badges: Vec<char> = vec![];
    let mut items_in_group = String::new();

    for (line_number, line) in reader.lines().enumerate() {
        let item = line?;

        if let Some(badge) = get_badge(&mut items_in_group, line_number, &item) {
            badges.push(badge);
        };

        if let Some(item) = get_duplicate_item(&item)? {
            duplicate_items.push(item);
        };
    }

    let duplicate_items_sum = calculate_priorities_sum(duplicate_items);
    let badge_sum = calculate_priorities_sum(badges);

    println!("Duplicate items priority total: {duplicate_items_sum}");
    println!("Badge total: {badge_sum}");

    Ok(())
}

fn in_both_compartments(items: &str, to_find: char) -> bool {
    let half = items.len() / 2;
    let first_compartment = &items[..half];
    let second_compartment = &items[half..];

    first_compartment.contains(to_find) && second_compartment.contains(to_find)
}

fn get_duplicate_item(item: &str) -> MyResult<Option<char>> {
    let mut result = None;

    for c in item.chars() {
        if in_both_compartments(&item, c) {
            result = Some(c);
            break;
        }
    }

    Ok(result)
}

fn get_badge(items_in_group: &mut String, line_number: usize, item: &str) -> Option<char> {
    let mut result = None;

    items_in_group.push_str(&remove_duplicate(&item));

    if (line_number + 1) % 3 == 0 {
        for c in items_in_group.chars() {
            if items_in_group.matches(c).count() == 3 {
                result = Some(c);
                break;
            }
        }

        items_in_group.clear();
    }

    result
}

fn get_priority(item: char) -> u8 {
    let items = ALPHABET.to_string() + &ALPHABET.to_uppercase();

    match items.find(item) {
        Some(index) => (index as u8) + 1,
        None => 0,
    }
}

fn remove_duplicate(items: &str) -> String {
    let mut result = String::new();

    for c in items.chars() {
        if !result.contains(c) {
            result.push(c);
        }
    }

    result
}

fn calculate_priorities_sum(items: Vec<char>) -> u32 {
    let mut sum = 0;

    for item in items.into_iter() {
        sum += get_priority(item) as u32;
    }

    sum
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
        assert_eq!(calculate_priorities_sum(vec!['a', 'A', 'b']), 30);
    }

    #[test]
    fn test_get_duplicate_item() -> MyResult<()> {
        let result = get_duplicate_item("abcb")?;
        assert_eq!(result, Some('b'));

        Ok(())
    }

    #[test]
    fn test_get_badge() {
        let mut items_in_group = String::new();

        let result = get_badge(&mut items_in_group, 0, "abcdY");
        assert_eq!(items_in_group, "abcdY".to_string());
        assert_eq!(result, None);

        let result = get_badge(&mut items_in_group, 1, "Yefg");
        assert_eq!(items_in_group, "abcdYYefg".to_string());
        assert_eq!(result, None);

        let result = get_badge(&mut items_in_group, 2, "hYij");
        assert_eq!(items_in_group, "".to_string());
        assert_eq!(result, Some('Y'));
    }

    #[test]
    fn test_remove_duplicate() {
        assert_eq!(remove_duplicate("aabbcc"), "abc".to_string());
    }
}
