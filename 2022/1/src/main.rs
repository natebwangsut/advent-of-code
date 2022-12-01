use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("advent-of-code: one");
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

    // We wanted to extract top 3
    let top = 3;
    let mut top_cal = 0;

    let mut n = 1;
    while n <= top {
        let max = elves.iter().max().unwrap_or(&0);
        top_cal += max;

        elves.remove(
            elves
                .iter()
                .position(|x| x == max)
                .expect("an elf carrying that food was not found"),
        );

        n += 1;
    }
    println!("max cal by {} number of elves: {}", top, top_cal);
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
