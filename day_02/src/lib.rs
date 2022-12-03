use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn calculate_points() -> MyResult<()> {
    let input = to_vector(read_input()?);

    let mut result = 0;

    for row in input.iter() {
        let row = row.split_once(" ").unwrap_or_default();
        result += calculate_row_points(row);
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

fn get_outcome_points((elf, me): (&str, &str)) -> u8 {
    // Elf's -> A - Rock, B - Paper, C - Scissors
    // Me -> X - Rock, Y - Paper, Z - Scissors
    match (elf, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,

        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,

        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,

        _ => 0,
    }
}

fn get_shape_points(shape: &str) -> u8 {
    // X - Rock, Y - Paper, Z - Scissors
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn calculate_row_points(row: (&str, &str)) -> u32 {
    (get_outcome_points(row) + get_shape_points(row.1)).into()
}
