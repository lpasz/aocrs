use crate::core::matrix;
use std::io;

pub fn part1(file_path: &str) -> io::Result<i32> {
    let mtx = matrix(file_path)?;

    let xmas = vec!['X', 'M', 'A', 'S'];
    let samx = vec!['S', 'A', 'M', 'X'];

    let mut matches = 0;

    for (y, line) in mtx.iter().enumerate() {
        for (x, _item) in line.iter().enumerate() {
            let arounds = positions((y, x));
            for around in arounds {
                let some = get_around(&mtx, &around);
                if let Some(value) = some {
                    if value == xmas || value == samx {
                        matches += 1;
                    }
                }
            }
        }
    }

    return Ok(matches);
}

fn get_around(mtx: &Vec<Vec<char>>, around: &Vec<(usize, usize)>) -> Option<Vec<char>> {
    let mut arounder = vec![];
    for (y, x) in around {
        let line = mtx.get(*y);
        if let Some(ln) = line {
            let item = ln.get(*x);
            if let Some(itm) = item {
                arounder.push(itm.clone());
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(arounder)
}

fn positions((y, x): (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let dirs = [
        [(0, 0), (1, 0), (2, 0), (3, 0)],    // ↓
        [(0, 0), (0, 1), (0, 2), (0, 3)],    // →
        [(0, 0), (1, 1), (2, 2), (3, 3)],    // ↘
        [(0, 0), (1, -1), (2, -2), (3, -3)], // ↙
    ];

    let x = x as isize;
    let y = y as isize;

    dirs.iter()
        .filter_map(|offsets| {
            let positions = offsets
                .iter()
                .map(|(dy, dx)| {
                    let ny = y + dy;
                    let nx = x + dx;
                    if ny >= 0 && nx >= 0 {
                        Some((ny as usize, nx as usize))
                    } else {
                        None
                    }
                })
                .collect();
            positions
        })
        .collect()
}

#[test]
fn test_part1() {
    let example = part1("assets/aoc24/day04/example.txt");
    let input = part1("assets/aoc24/day04/input.txt");
    assert_eq!(example.unwrap(), 18);
    assert_eq!(input.unwrap(), 2427);
}

pub fn part2(file_path: &str) -> io::Result<i32> {
    println!("Hey!");
    let mtx = matrix(file_path)?;
    println!("{:?}", mtx);

    let mas = vec!['M', 'A', 'S'];
    let sam = vec!['S', 'A', 'M'];

    let mut matches = 0;

    for (y, line) in mtx.iter().enumerate() {
        for (x, _item) in line.iter().enumerate() {
            println!("y: {}, x: {}", y, x);
            let arounds = x_mas((y, x));
            println!("{:?}", arounds);
            if arounds.len() != 2 {
                continue;
            }
            let around1 = arounds.get(0).unwrap();
            let around2 = arounds.get(1).unwrap();

            println!("{:?} - {:?}", around1, around2);

            let value1 = get_around(&mtx, around1);
            let value2 = get_around(&mtx, around2);

            println!("{:?} - {:?}", value1, value2);

            if value1 == None {
                continue;
            }
            if value2 == None {
                continue;
            }

            let value1 = value1.unwrap();
            let value2 = value2.unwrap();

            if (value1 == mas || value1 == sam) && (value2 == mas || value2 == sam) {
                matches += 1;
            }
        }
    }

    return Ok(matches);
}

fn x_mas((y, x): (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let dirs = [[(-1, -1), (0, 0), (1, 1)], [(-1, 1), (0, 0), (1, -1)]];

    let x = x as isize;
    let y = y as isize;

    dirs.iter()
        .filter_map(|offsets| {
            let positions = offsets
                .iter()
                .map(|(dy, dx)| {
                    let ny = y + dy;
                    let nx = x + dx;
                    if ny >= 0 && nx >= 0 {
                        Some((ny as usize, nx as usize))
                    } else {
                        None
                    }
                })
                .collect();
            positions
        })
        .collect()
}

#[test]
fn test_part2() {
    let example = part2("assets/aoc24/day04/example.txt");
    let input = part2("assets/aoc24/day04/input.txt");
    assert_eq!(example.unwrap(), 9);
    assert_eq!(input.unwrap(), 1900);
}
