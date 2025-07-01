use regex::Regex;
use std::fs;
use std::io;

pub fn part1(file_path: &str) -> io::Result<i32> {
    let content = fs::read_to_string(file_path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;

    for (_, [n1, n2]) in re.captures_iter(&content).map(|c| c.extract()) {
        let v1 = n1.parse::<i32>().unwrap();
        let v2 = n2.parse::<i32>().unwrap();

        result += v1 * v2;
    }

    Ok(result)
}

#[test]
fn test_part1() {
    let example = part1("assets/aoc24/day03/example.txt");
    let input = part1("assets/aoc24/day03/input.txt");
    assert_eq!(example.unwrap(), 161);
    assert_eq!(input.unwrap(), 179571322);
}

pub fn part2(file_path: &str) -> io::Result<i32> {
    let content = fs::read_to_string(file_path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do(\(\)))|(don't(\(\)))").unwrap();

    let mut enabled = true;
    let mut result = 0;

    for (_, [n1, n2]) in re.captures_iter(&content).map(|c| c.extract()) {
        match (n1, n2) {
            ("do()", _n) => enabled = true,
            ("don't()", _n) => enabled = false,
            (n1, n2) if enabled => {
                let v1 = n1.parse::<i32>().unwrap();
                let v2 = n2.parse::<i32>().unwrap();

                result += v1 * v2;
            }
            _ => {}
        }
    }

    Ok(result)
}

#[test]
fn test_part2() {
    let example = part2("assets/aoc24/day03/example2.txt");
    let input = part2("assets/aoc24/day03/input.txt");
    assert_eq!(example.unwrap(), 48);
    assert_eq!(input.unwrap(), 103811193);
}
