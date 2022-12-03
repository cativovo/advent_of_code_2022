use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn calculate_points() -> MyResult<()> {
    let input = to_vector(read_input()?);

    let mut result = 0;

    for row in input.iter() {
        let row: Vec<char> = row.chars().collect();

        if row.len() == 3 {
            result += calculate_row_points((row[0], row[2]));
        }
    }

    println!("result: {result}");

    Ok(())
}

fn read_input() -> MyResult<String> {
    Ok(fs::read_to_string("./input.txt")?.to_string())
}

fn to_vector(input: String) -> Vec<String> {
    input.split("\n").map(|el| el.to_string()).collect()
}

fn get_outcome_points(outcome: char) -> u8 {
    // X - Lose, Y - Draw, Z - Win
    match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn get_move_points(mov: char) -> u8 {
    // A - Rock, B - Paper, C - Scissors
    match mov {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn generate_losing_move(elf_move: char) -> Option<char> {
    // A - Rock, B - Paper, C - Scissors
    match elf_move {
        'A' => Some('C'),
        'B' => Some('A'),
        'C' => Some('B'),
        _ => None,
    }
}

fn generate_winning_move(elf_move: char) -> Option<char> {
    // A - Rock, B - Paper, C - Scissors
    match elf_move {
        'A' => Some('B'),
        'B' => Some('C'),
        'C' => Some('A'),
        _ => None,
    }
}

fn generate_move((elf_move, outcome): (char, char)) -> Option<char> {
    // X - Lose, Y - Draw, Z - Win
    match outcome {
        'X' => generate_losing_move(elf_move),
        'Y' => Some(elf_move.into()),
        'Z' => generate_winning_move(elf_move),
        _ => None,
    }
}

fn calculate_row_points(row: (char, char)) -> u32 {
    match generate_move(row) {
        Some(my_move) => (get_outcome_points(row.1) + get_move_points(my_move)).into(),
        None => 0,
    }
}
