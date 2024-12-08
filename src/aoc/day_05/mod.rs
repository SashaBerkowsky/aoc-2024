use std::{collections::HashMap, fs};

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-05.txt").expect("file from day 05 not found");

    println!("part one: {}", part_one(&content));
    println!("part two: {}", part_two(&content));
}

pub fn part_one(input: &str) -> u32 {
    let manual = Manual::from(input);
    let mut sum = 0;

    manual
        .updates
        .iter()
        .filter(|&update| manual.correct(update))
        .for_each(|update| {
            let mid = update.len() / 2;
            sum += update[mid]
        });

    sum
}


pub fn part_two(input: &str) -> u32 {
    let manual = Manual::from(input);
    let mut sum = 0;

    manual
        .updates
        .iter()
        .filter(|&update| !manual.correct(update)) 
        .for_each(|update| {
            let sorted_update = manual.fix_update(update.clone());

            let mid = sorted_update.len() / 2;
            sum += sorted_update[mid];
        });

    sum
}

#[derive(Clone)]
struct Manual {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Manual {
    fn correct(&self, update: &[u32]) -> bool {
        let map: HashMap<u32, usize> = update
            .iter()
            .enumerate()
            .map(|(idx, &page)| (page, idx))
            .collect();

        self.rules
            .iter()
            .all(|(x, y)| match (map.get(x), map.get(y)) {
                (Some(&x), Some(&y)) => x < y,
                _ => true,
            })
    }

    fn fix_update(&self, mut update: Vec<u32>) -> Vec<u32> {
        update.sort_by(|&x, &y| {
            if self.rules.contains(&(x, y)) {
                std::cmp::Ordering::Less
            } else if self.rules.contains(&(y, x)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        update
    }
}

impl From<&str> for Manual {
    fn from(input: &str) -> Self {
        let manual: Vec<&str> = input.split("\n\n").collect();
        let (rules, updates) = (manual[0], manual[1]);

        let rules: Vec<(u32, u32)> = rules
            .lines()
            .map(|rule| {
                let parts: Vec<u32> = rule.split('|').filter_map(|num| num.parse().ok()).collect();
                (parts[0], parts[1])
            })
            .collect();

        let updates: Vec<Vec<u32>> = updates
            .lines()
            .map(|update| {
                update
                    .split(',')
                    .filter_map(|num| num.parse().ok())
                    .collect()
            })
            .collect();

        Self { rules, updates }
    }
}
