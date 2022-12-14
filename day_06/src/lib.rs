use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let input = read_input()?;

    if let Some(result) = find_start_of_packet_pos(&input, 4) {
        println!("4 distinct characters - {result}");
    }

    if let Some(result) = find_start_of_packet_pos(&input, 14) {
        println!("14 distinct characters - {result}");
    }

    Ok(())
}

fn read_input() -> MyResult<String> {
    Ok(fs::read_to_string("./input.txt")?)
}

fn find_start_of_packet_pos(input: &str, len: usize) -> Option<u32> {
    let mut chars: Vec<char> = vec![];

    for (i, c) in input.chars().enumerate() {
        let is_in = chars.contains(&c);

        if !is_in && chars.len() == (len - 1) {
            return Some(i as u32 + 1);
        }

        if let Some(repeated_pos) = chars.iter().position(|e| e == &c) {
            chars.drain(..=repeated_pos);
        }

        chars.push(c);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_packet_pos() {
        let result = find_start_of_packet_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(result, Some(5));

        let result = find_start_of_packet_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        assert_eq!(result, Some(10));

        let result = find_start_of_packet_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(result, Some(11));

        let result = find_start_of_packet_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        assert_eq!(result, Some(19));

        let result = find_start_of_packet_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        assert_eq!(result, Some(23));

        let result = find_start_of_packet_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        assert_eq!(result, Some(29));
    }
}
