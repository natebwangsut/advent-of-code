use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 6
///
fn main() {
    println!("advent-of-code: six");

    let Ok(lines) = read_lines("./input") else { return };
    // let Ok(lines) = read_lines("./input.example") else { return };

    let marker = 14;
    let mut counter = 0;

    for line in lines.flatten() {
        loop {
            let window = &line[counter..counter + marker];
            let set: HashSet<char> = HashSet::from_iter(window.chars().into_iter());

            // println!("{} {} {:?}", counter, window, set);
            if set.len() == marker || counter + marker > line.len() {
                break;
            }

            counter += 1;
        }
    }

    println!("{:?}", counter + marker);
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
