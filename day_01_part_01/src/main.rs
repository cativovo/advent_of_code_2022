use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Ok(highest_calories) = get_highest_calories() {
        println!("{highest_calories}");
    }
}

fn read_input() -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut result: Vec<Vec<u32>> = vec![];
    let mut nums: Vec<u32> = vec![];

    let input_path = "./input.txt";
    let file = File::open(input_path)?;
    let buf = BufReader::new(file);

    for line in buf.lines() {
        let line = line?;

        if !line.is_empty() {
            nums.push(line.parse()?);
        } else {
            result.push(nums.to_vec());
            nums.clear();
        }
    }

    Ok(result)
}

fn get_highest_calories() -> Result<u32, Box<dyn Error>> {
    let input = read_input()?;

    let mut result: Vec<u32> = input
        .into_iter()
        .map(|vec| vec.into_iter().reduce(|acc, el| acc + el).unwrap_or(0))
        .collect();

    result.sort_by(|a, b| b.cmp(a));

    Ok(result[0])
}
