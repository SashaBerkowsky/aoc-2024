mod aoc;

fn main() {
    const AOC_DAY: u8 = 4;

    println!("Advent of code day {}:", {AOC_DAY});
    match AOC_DAY {
        1 => aoc::day_01::solve(),
        2 => aoc::day_02::solve(),
        3 => aoc::day_03::solve(),
        4 => aoc::day_04::solve(),
        _ => println!("Wrong day"),
    }
}
