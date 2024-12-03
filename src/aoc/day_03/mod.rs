use std::fs;

const MIN_OPERATION_LEN: usize = 8;
const MAX_OPERATION_LEN: usize = 12;

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-03.txt").expect("file from day 03 not found'");

    println!("part one total: {}", part_one(&content));
    println!("part two total: {}", part_two(&content));
}

fn part_one(content: &str) -> i32 {
    let mut mult_result: i32 = 0;
    let mut i: usize = 0;

    while i < content.len() {
        if content[i..].starts_with("mul(") {
            match content[i..].find(")") {
                Some(end_idx) => {
                    let operation_length = end_idx + 1;
                    if operation_length >= MIN_OPERATION_LEN && operation_length <= MAX_OPERATION_LEN {
                        mult_result += parse_multiplication(&content[i + 4.. i + end_idx]);

                        i += end_idx;
                    } else {
                        i += 1
                    }
                },
                None => break,
            }
        } else {
            i += 1;
        }
    }

    mult_result
}

fn part_two(content: &str) -> i32 {
    let mut is_operation_enabled: bool = true;
    let mut mult_result: i32 = 0;
    let mut i: usize = 0;

    while i < content.len() {
        let is_operation_mult = content[i..].starts_with("mul(");
        if is_operation_enabled && is_operation_mult {
            match content[i..].find(")") {
                Some(end_idx) => {
                    let operation_length = end_idx + 1;
                    if operation_length >= MIN_OPERATION_LEN && operation_length <= MAX_OPERATION_LEN {
                        mult_result += parse_multiplication(&content[i + 4.. i + end_idx]);

                        i += end_idx;
                    } else {
                        i += 1
                    }
                },
                None => break,
            }
        } else if !is_operation_enabled {
            match content[i..].find("do()") {
                Some(idx) => {
                    i += idx + 3;
                    is_operation_enabled = true;
                },
                None => i = content.len(),
            } 
        } else if !is_operation_mult {
            is_operation_enabled = !content[i..].starts_with("don't()");

            i += 1;
        } 
    }


    mult_result
}

fn parse_multiplication(operands: &str) -> i32 {
    let unparsed_numbers: Vec<&str> = operands.split(',').collect();
    let mut res: i32 = 0;

    if unparsed_numbers.len() == 2 {
        match (unparsed_numbers[0].parse::<i32>(), unparsed_numbers[1].parse::<i32>()) {
            (Ok(a), Ok(b)) => res = a * b,
            _ => {},
        };
    };

    res
}
