use std::{collections::HashMap, fs};

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-05.txt").expect("file from day 05 not found");

    let separator_idx = content.find("\n\n").expect("puzzle input not formatted");
    let section_a = &content[..separator_idx];
    let section_b = &content[separator_idx + 2..];

    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut middle_correct_updates = 0;

    section_a.lines().for_each(|line_a| {
        let (x_str, y_str) = line_a.split_once("|").expect("section a not formatted");

        let x: i32 = x_str.parse().expect("failed to parse y");
        let y: i32 = y_str.parse().expect("failed to parse y");

        if let Some(x_rules) = ordering_rules.get_mut(&x) {
            x_rules.push(y);
        } else {
            ordering_rules.insert(x, vec![y]);
        }
    });

    ordering_rules.iter_mut().for_each(|(_, rules)| {
        rules.sort();
    });

    section_b.lines().for_each(|line_b| {
        let mut current_update: Vec<i32> = Vec::new();
        let mut is_update_valid = true;
        let mut i: usize = 0;

        line_b.split(",").for_each(|page_str| {
            current_update.push(page_str.parse().expect("failed to parse page"));
        });


        while i < current_update.len() {
            let current_page = current_update[i];
            let mut j: usize = 0;

            while j < i  && is_update_valid {
                if let Some(rules) = ordering_rules.get(&current_page) {
                    let search_result = rules.binary_search(&current_update[j]);
                    is_update_valid = search_result.is_err();
                }

                j += 1;
            }

            i += 1;
        }

        if is_update_valid {
            middle_correct_updates += current_update[current_update.len() / 2];
        }
    });


    println!("total: {}", middle_correct_updates);
}

