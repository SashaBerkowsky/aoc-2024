use std::fs;

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-03.txt").expect("file from day 03 not found'");
    
    const MIN_OPERATION_LEN: usize = 8;
    const MAX_OPERATION_LEN: usize = 12;
    let mut mult_result: i32 = 0;
    let mut i: usize = 0;

    while i < content.len() {
        if content[i..].starts_with("mul(") {
            match content[i..].find(")") {
                Some(end_idx) => {
                    let operation_length = end_idx + 1;
                    if operation_length >= MIN_OPERATION_LEN && operation_length <= MAX_OPERATION_LEN {
                        let operands = &content[i + 4.. i + end_idx];
                        let unparsed_numbers: Vec<&str> = operands.split(',').collect();
                        println!("{:?}", operands);

                        if unparsed_numbers.len() == 2 {
                            if let (Ok(a), Ok(b)) = (unparsed_numbers[0].parse::<i32>(), unparsed_numbers[1].parse::<i32>()) {
                                mult_result += a * b;
                            }
                        }

                        i += end_idx;
                    } else {
                        i += 1
                    }
                },
                None => {},
            }
        } else {
            i += 1;
        }
    }

    println!("total: {}", mult_result)
}
