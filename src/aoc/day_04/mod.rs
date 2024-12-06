use std::fs;

const SEARCH_WORD: &str = "XMAS";
const WORD_OFFSET: usize = SEARCH_WORD.len() - 1;

#[derive(Debug)]
enum ReadDirections {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

impl ReadDirections {
    pub fn read_word(&self, center_i: usize, center_j: usize, puzzle: &Vec<Vec<char>>) -> String {
        let mut word = String::new();
        let mut k: usize = 0;

        while k <= WORD_OFFSET {
            word.push(self.read_char(center_i, center_j, k, puzzle));

            k += 1;
        }

        word
    }

    fn read_char(&self, i: usize, j: usize, k: usize, puzzle: &Vec<Vec<char>>) -> char {
        match self {
            ReadDirections::West => puzzle[i][j - k],
            ReadDirections::East => puzzle[i][j + k],
            ReadDirections::South => puzzle[i + k][j],
            ReadDirections::North => puzzle[i - k][j],
            ReadDirections::NorthEast => puzzle[i - k][j + k],
            ReadDirections::SouthEast => puzzle[i + k][j + k],
            ReadDirections::NorthWest => puzzle[i - k][j - k],
            ReadDirections::SouthWest => puzzle[i + k][j - k],
        }
    }
}

pub fn solve() {
    let content = fs::read_to_string("src/txt/day-04.txt").expect("file from day 04 not found");
    let mut puzzle: Vec<Vec<char>> = Vec::new();

    content.lines().for_each(|line| {
        puzzle.push(line.chars().collect());
    });

    part_one(&puzzle);
    part_two(&puzzle);
}

fn part_one(puzzle: &Vec<Vec<char>>) {
    let mut word_total: i32 = 0;

    for i in 0.. puzzle.len() {
        for j in 0.. puzzle[i].len() {
            if puzzle[i][j] == 'X' {
                word_total += get_word_amount(i, j, &puzzle);
            }
        }
    }

    println!("total aparences of word {}: {}", SEARCH_WORD, word_total);
}

fn get_word_amount(i: usize, j: usize, puzzle: &Vec<Vec<char>>) -> i32 {
    let read_directions = get_read_directions(i, j, puzzle.len());
    let mut word_amount: i32 = 0;

    read_directions.iter().for_each(|direction| {
        if direction.read_word(i, j, puzzle) == SEARCH_WORD.to_string() {
            word_amount += 1;
        }
    });

    word_amount
}


fn get_read_directions(i: usize, j: usize, len: usize) -> Vec<ReadDirections> {
    let mut read_directions: Vec<ReadDirections> = Vec::new();
    
    if j + WORD_OFFSET < len {
        read_directions.push(ReadDirections::East);
        if i >= WORD_OFFSET {
            read_directions.push(ReadDirections::NorthEast);
        }

        if i + WORD_OFFSET < len {
            read_directions.push(ReadDirections::SouthEast);
        }
    }

    if j >= WORD_OFFSET {
        read_directions.push(ReadDirections::West);

        if i >= WORD_OFFSET {
            read_directions.push(ReadDirections::NorthWest);
        }

        if i + WORD_OFFSET < len {
            read_directions.push(ReadDirections::SouthWest);
        }
    }

    if i >= WORD_OFFSET {
        read_directions.push(ReadDirections::North);
    }

    if i + WORD_OFFSET < len {
        read_directions.push(ReadDirections::South);
    }

    read_directions
}

fn part_two(puzzle: &Vec<Vec<char>>) {
    let mut x_shaped_total: i32 = 0;

    for i in 1.. puzzle.len() - 1 {
        for j in 1.. puzzle[i].len() - 1 {
            if is_desired_pattern(i, j, puzzle) {
                x_shaped_total += 1;
            }
        }
    }

    println!("total x-shaped {}: {}", SEARCH_WORD, x_shaped_total);
}

fn is_desired_pattern(i: usize, j: usize, puzzle: &Vec<Vec<char>>) -> bool {
    let is_fst_diag_valid = (puzzle[i - 1][j - 1] == 'M' && puzzle[i + 1][j + 1] == 'S') || (puzzle[i - 1][j - 1] == 'S' && puzzle[i + 1][j + 1] == 'M');
    let is_snd_diag_valid = (puzzle[i + 1][j - 1] == 'M' && puzzle[i - 1][j + 1] == 'S') || (puzzle[i + 1][j - 1] == 'S' && puzzle[i - 1][j + 1] == 'M');

    puzzle[i][j] == 'A' && is_fst_diag_valid && is_snd_diag_valid
}
