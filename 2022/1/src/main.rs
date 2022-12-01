use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 1
///
fn main() {
    println!("advent-of-code: one");
    let mut elves: Vec<i32> = vec![0];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut counter = 0;
        for line in lines.flatten() {
            // Empty line - the current elf is done, adding new elf
            if line.is_empty() {
                counter += 1;
                elves.push(0);
                continue;
            }

            // Add the cals to the current elf that's carrying the food
            elves[counter] += line.parse::<i32>().unwrap_or_default();
        }
    }

    // Show final result
    println!("max cal by an elves: {}", elves.iter().max().unwrap_or(&0));

    // Extract largest N numbers and sum it up
    elves.sort();
    let top = 3;
    let top_cal = elves
        .rchunks(top)
        .next()
        .unwrap_or_default()
        .iter()
        .sum::<i32>();

    println!("max cal by {} number of elves: {}", top, top_cal);
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
