use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 3
///
fn main() {
    println!("advent-of-code: three");

    let binding = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let alphabet = binding.chars();

    // First Part
    let Ok(lines) = read_lines("./input") else { return };
    let sum = lines
        .flatten()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            // println!("{} | {}", first, second);
            let common = first
                .chars()
                .filter(|&c1| second.contains(c1))
                .nth(0)
                .unwrap_or_default();

            (alphabet
                .clone()
                .position(|c| c == common)
                .unwrap_or_default()
                + 1) as i32
        })
        .sum::<i32>();
    println!("{}", sum);

    // Second Part
    let Ok(lines) = read_lines("./input") else { return };
    let sum = lines
        .flatten()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| {
            let common = chunk[0]
                .chars()
                .filter(|&c| chunk[1].contains(c) && chunk[2].contains(c))
                .nth(0)
                .unwrap_or_default();

            (alphabet
                .clone()
                .position(|c| c == common)
                .unwrap_or_default()
                + 1) as i32
        })
        .sum::<i32>();

    println!("{}", sum);
}

///
/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
///
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
