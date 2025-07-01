use std::io;

mod day01;
mod day02;

fn main() -> io::Result<()> {
    day01::part1("../assets/aoc24/day01/input.txt")?;
    day01::part2("../assets/aoc24/day01/input.txt")?;

    Ok(())
}
