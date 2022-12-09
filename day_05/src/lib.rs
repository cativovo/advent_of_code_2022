use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;
type Stacks = Vec<Vec<String>>;
type Moves = Vec<String>;

const SPACING: &str = "   ";

pub fn run() -> MyResult<()> {
    let input = read_input()?;
    let (mut stacks, moves) = get_stacks_and_moves(input);

    // clear screen
    // print!("\x1B[2J\x1B[1;1H");
    println!("start: ");
    print_stacks(&stacks);
    println!();

    for mov in moves.into_iter() {
        let pos = get_position(&mov);
        let mut crate_count = pos[0];
        let stack_from_pos = (pos[1] - 1) as usize;
        let stack_to_pos = (pos[2] - 1) as usize;

        loop {
            if crate_count == 0 {
                break;
            }

            let mut crat_to_move = String::new();

            if let Some(stack_from) = stacks.get_mut(stack_from_pos) {
                for (i, crat) in stack_from.iter().enumerate() {
                    if crat != SPACING {
                        crat_to_move.clear();
                        crat_to_move.push_str(crat);
                        stack_from.remove(i);
                        break;
                    }
                }
            }

            if let Some(stack_to) = stacks.get_mut(stack_to_pos) {
                for (i, crat) in stack_to.iter().enumerate() {
                    if crat != SPACING {
                        stack_to.insert(i, crat_to_move);
                        break;
                    }
                }
            }

            crate_count -= 1;
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("{}", mov);
        println!();
        print_stacks(&stacks);
    }

    Ok(())
}

fn read_input() -> MyResult<Vec<String>> {
    let content = fs::read_to_string("./input.txt")?;
    let result: Vec<String> = content
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    Ok(result)
}

fn get_stacks_and_moves(input: Vec<String>) -> (Stacks, Moves) {
    let stacks_count = 9;
    let stacks_end = 8;
    let moves_start = 10;

    let mut stacks: Vec<Vec<String>> = vec![];
    let moves = input[moves_start..].to_vec();
    // let moves = input[moves_start..moves_start + 10].to_vec();

    for row in input[..=stacks_end].into_iter() {
        let mut cur_index = 0;
        let crate_len = 3;

        for i in 0..stacks_count {
            let crat = row[cur_index..crate_len + cur_index].to_string();

            if let Some(stack) = stacks.get_mut(i) {
                stack.push(crat);
            } else {
                stacks.push(vec![crat]);
            }
            cur_index += crate_len + 1;
        }
    }

    (stacks, moves)
}

fn get_position(mov: &String) -> Vec<u32> {
    mov.split(" ")
        .filter_map(|el| el.parse::<u32>().ok())
        .collect()
}

fn print_stacks(stacks: &Stacks) {
    let mut result: Vec<Vec<String>> = vec![];
    let largest = stacks.iter().fold(0, |acc, crat| {
        let filltered: Vec<&String> = crat.iter().filter(|x| x.as_str() != SPACING).collect();

        if filltered.len() > acc {
            filltered.len()
        } else {
            acc
        }
    });

    for stack in stacks.into_iter() {
        let mut filtered: Vec<String> = stack
            .into_iter()
            .filter(|crat| crat.as_str() != SPACING)
            .map(|crat| crat.to_string())
            .collect();

        while largest > filtered.len() {
            filtered.insert(0, SPACING.to_string());
        }

        for (i, crat) in filtered.iter().enumerate() {
            let crat = crat.to_string();
            if let Some(row) = result.get_mut(i) {
                row.push(crat);
            } else {
                result.push(vec![crat])
            }
        }
    }

    let result = result
        .iter()
        .map(|el| el.join(" "))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{result}");
}
