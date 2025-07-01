use std::io;

mod day01;
mod day02;
mod day03;

fn main() -> io::Result<()> {
    day01::part1("../assets/aoc24/day01/input.txt")?;
    day01::part2("../assets/aoc24/day01/input.txt")?;

    day02::part1("../assets/aoc24/day02/input.txt")?;
    day02::part2("../assets/aoc24/day02/input.txt")?;

    day03::part1("../assets/aoc24/day03/input.txt")?;
    day03::part2("../assets/aoc24/day03/input.txt")?;
    Ok(())
}
