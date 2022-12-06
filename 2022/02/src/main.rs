use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 2
///
fn main() {
    println!("advent-of-code: two");

    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("./input") else { return };
    let rounds = lines
        .flatten()
        .map(|str| {
            let mut split = str.split(' ');
            (
                split.next().unwrap().to_owned(),
                split.next().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    println!(
        "part 1: {}",
        rounds
            .clone()
            .into_iter()
            .map(score_mapper_part1)
            .sum::<i32>()
    );

    println!(
        "part 2: {}",
        rounds
            .clone()
            .into_iter()
            .map(score_mapper_part2)
            .sum::<i32>()
    );
}

fn score_mapper_part1(data: (String, String)) -> i32 {
    // A, X = rock | B, Y = paper | C, Z = scissor
    match (data.0.as_ref(), data.1.as_ref()) {
        ("A", "X") => 4,
        ("B", "X") => 1,
        ("C", "X") => 7,

        ("A", "Y") => 8,
        ("B", "Y") => 5,
        ("C", "Y") => 2,

        ("A", "Z") => 3,
        ("B", "Z") => 9,
        ("C", "Z") => 6,
        (_, _) => 0,
    }
}

fn score_mapper_part2(data: (String, String)) -> i32 {
    // A = rock, B = paper, C = scissor
    // X = lose, Y = draw,  Z = win
    match (data.0.as_ref(), data.1.as_ref()) {
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,

        ("A", "Y") => 4,
        ("B", "Y") => 5,
        ("C", "Y") => 6,

        ("A", "Z") => 8,
        ("B", "Z") => 9,
        ("C", "Z") => 7,

        (_, _) => 0,
    }
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
