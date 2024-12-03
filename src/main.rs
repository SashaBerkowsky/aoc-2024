mod aoc;

fn main() {
    const AOC_DAY: u8 = 2;

    match AOC_DAY {
        1 => aoc::day_01::solve(),
        2 => aoc::day_02::solve(),
        _ => println!("Wrong day"),
    }
}
