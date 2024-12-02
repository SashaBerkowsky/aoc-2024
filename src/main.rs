mod aoc;

fn main() {
    const AOC_DAY: u8 = 1;

    match AOC_DAY {
        1 => aoc::day_01::solve(),
        _ => println!("Wrong day"),
    }
}
