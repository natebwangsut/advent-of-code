use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("one");
    let mut elves: Vec<i32> = vec![0];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut counter = 0;

        for line in lines {
            if let Ok(content) = line {
                // empty line - the current elf is done, adding new elf
                if content.is_empty() {
                    counter += 1;
                    elves.push(0);
                    continue;
                }

                // add the cals to the current elf that's carring the food
                elves[counter] += content.parse::<i32>().unwrap_or_default();
            }
        }
    }

    // Show final result
    println!("max cal by an elves: {}", elves.iter().max().unwrap_or(&0));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
