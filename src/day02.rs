use std::fs;
use std::io;
use std::io::BufRead;

pub fn part1(file_path: &str) -> io::Result<i32> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe = 0;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut inc: Option<bool> = None;
        let mut valid = true;

        for pairs in nums.windows(2) {
            let p1 = pairs[0];
            let p2 = pairs[1];
            let is_inc = p1 < p2;

            if inc == None {
                inc = Some(is_inc);
            }

            let diff = i32::abs(p1 - p2);

            valid = valid && diff >= 1 && diff <= 3 && inc == Some(is_inc)
        }

        if valid {
            safe += 1;
        }
    }

    Ok(safe)
}

#[test]
fn test_part1() {
    let example = part1("assets/aoc24/day02/example.txt");
    let input = part1("assets/aoc24/day02/input.txt");
    assert_eq!(example.unwrap(), 2);
    assert_eq!(input.unwrap(), 670);
}

pub fn part2(file_path: &str) -> io::Result<i32> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safev2 = 0;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut minus = nums.clone();
        let mut any_valid = false;
        for i in 0..nums.len() {
            let item = minus[i];
            minus.remove(i);

            let mut inner_valid = true;

            let mut inner_inc: Option<bool> = None;
            for pairs in minus.windows(2) {
                let p1 = pairs[0];
                let p2 = pairs[1];
                let is_inc = p1 < p2;

                if inner_inc == None {
                    inner_inc = Some(is_inc);
                }

                let diff = i32::abs(p1 - p2);

                inner_valid = inner_valid && diff >= 1 && diff <= 3 && inner_inc == Some(is_inc)
            }

            if inner_valid {
                any_valid = true;
            }

            minus.insert(i, item);
        }

        if any_valid {
            safev2 += 1
        }
    }

    Ok(safev2)
}

#[test]
fn test_part2() {
    let example = part2("assets/aoc24/day02/example.txt");
    let input = part2("assets/aoc24/day02/input.txt");
    assert_eq!(example.unwrap(), 4);
    assert_eq!(input.unwrap(), 700);
}
