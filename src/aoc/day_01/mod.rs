use std::{collections::HashMap, fs};

pub fn solve() {
    let content = fs::read_to_string("../../txt/day-01.txt").expect("file from day 01 not found");
    let lines = content.lines();

    // part one:
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut distance: i32 = 0;

    // part two:
    let mut similarity_score: i32 = 0;
    let mut right_ocurrences: HashMap<i32, i32> = HashMap::new();

    lines.for_each(|line| {
        let mut number_iter = line.split_ascii_whitespace();
        let left_str = number_iter.next().expect("no more numbers left");
        let right_str = number_iter.next().expect("no more numbers right");

        left_list.push(left_str.parse().unwrap());
        right_list.push(right_str.parse().unwrap());
    });

    left_list.sort();
    right_list.sort();

    for n in 0.. left_list.len() {
        let (left, right) = (left_list[n], right_list[n]);
        distance += (left - right).abs();

        let right_counter = right_ocurrences.entry(right).or_insert(0);
        *right_counter += 1;

        if !right_ocurrences.contains_key(&left) {
            right_ocurrences.insert(left, 0);
        }
    }

    for n in 0.. left_list.len() {
        let left = left_list[n];
        similarity_score += left * right_ocurrences[&left];
    }

    println!("distance: {}", distance);
    println!("similarity score: {}", similarity_score);
}
