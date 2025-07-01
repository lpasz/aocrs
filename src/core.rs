use std::io::Result;

use std::fs;
use std::io;
use std::io::BufRead;

pub fn matrix(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut mtx = vec![];
    for line in reader.lines() {
        let line = line?;

        let v: Vec<char> = line.chars().collect();
        mtx.push(v);
    }

    Ok(mtx)
}

#[test]
fn test_matrix() {
    let mtx = matrix("assets/aoc24/day04/example.txt");
    println!("{:?}", mtx);
    let mtx = mtx.unwrap();
    let eq: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];
    assert_eq!(mtx, eq);
}
