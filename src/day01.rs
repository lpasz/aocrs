use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

fn to_vecs(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut first_line: Vec<i32> = vec![];
    let mut second_line: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let num1 = nums[0];
        let num2 = nums[1];

        first_line.push(num1);
        second_line.push(num2);
    }

    Ok((first_line, second_line))
}

pub fn part1(file_path: &str) -> io::Result<i32> {
    let result = to_vecs(file_path)?;
    let (mut first_line, mut second_line) = result;

    first_line.sort();
    second_line.sort();

    let mut part1 = 0;

    for (x, y) in first_line.iter().zip(second_line.iter()) {
        part1 += i32::abs(x - y);
    }

    Ok(part1)
}

#[test]
fn test_part1() {
    let example = part1("assets/aoc24/day01/example.txt");
    let input = part1("assets/aoc24/day01/input.txt");
    assert_eq!(example.unwrap(), 11);
    assert_eq!(input.unwrap(), 2367773);
}

pub fn part2(file_path: &str) -> io::Result<i32> {
    let result = to_vecs(file_path)?;
    let (first_line, second_line) = result;

    let mut freq: HashMap<i32, i32> = HashMap::new();

    for val in second_line {
        *freq.entry(val).or_insert(0) += 1;
    }

    let mut part2 = 0;

    for val in first_line {
        let cnt = *freq.get(&val).unwrap_or(&0);
        part2 += val * cnt;
    }

    Ok(part2)
}

#[test]
fn test_part2() {
    let example = part2("assets/aoc24/day01/example.txt");
    let input = part2("assets/aoc24/day01/input.txt");
    assert_eq!(example.unwrap(), 31);
    assert_eq!(input.unwrap(), 21271939);
}
