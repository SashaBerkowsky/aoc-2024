use std::fs;

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-02.txt").expect("file from day 02 not found");
    let lines = content.lines();

    // part one:
    let mut safe_reports: u32 = 0;
    // part two:
    let mut dampened_reports: u32 = 0;

    lines.for_each(|line| {
        let report_str = line.split_ascii_whitespace();
        let mut report: Vec<i32> = Vec::new();

        report_str.for_each(|r| {
            report.push(r.parse().expect("parsing error"));

        });

        if is_safe_report(&report) {
            safe_reports += 1;
        } else if is_dampened_safe_report(&report) {
            dampened_reports += 1;
        }
    });

    println!("safe reports: {}", safe_reports);
    println!("safe dampened reports: {}", dampened_reports + safe_reports);
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    const SAFE_DIFF_MIN: i32 = 1;
    const SAFE_DIFF_MAX: i32 = 3;

    let mut is_strictly_inc = true;
    let mut is_strictly_dec = true;
    let mut is_safe_diff = true;
    let mut i: usize = 1;

    while i < report.len() && (is_strictly_dec || is_strictly_inc) && is_safe_diff {
        let distance = (report[i] - report[i - 1]).abs();

        is_strictly_dec &= report[i - 1] > report[i];
        is_strictly_inc &= report[i - 1] < report[i];
        is_safe_diff &= distance >= SAFE_DIFF_MIN && distance <= SAFE_DIFF_MAX;

        i += 1;
    }
       
    (is_strictly_dec || is_strictly_inc) && is_safe_diff
}

fn is_dampened_safe_report(report :&Vec<i32>) -> bool {
    let mut is_dampened = false;
    let mut i: usize = 0;

    while i < report.len() && !is_dampened {
        let mut report_clone = report.clone();
        report_clone.remove(i);
        is_dampened = is_safe_report(&report_clone);

        i += 1;
    }

    is_dampened
}
