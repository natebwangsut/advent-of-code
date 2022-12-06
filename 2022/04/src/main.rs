use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 4
///
fn main() {
    println!("advent-of-code: four");

    let Ok(lines) = read_lines("./input") else { return };
    let pairs = lines
        .flatten()
        .map(|line| {
            let mut range = line.split(",");
            let mut first_range = range.next().unwrap().split("-");
            let mut second_range = range.next().unwrap().split("-");
            (
                (
                    first_range
                        .next()
                        .unwrap()
                        .to_owned()
                        .parse::<i32>()
                        .unwrap_or_default(),
                    first_range
                        .next()
                        .unwrap()
                        .to_owned()
                        .parse::<i32>()
                        .unwrap_or_default(),
                ),
                (
                    second_range
                        .next()
                        .unwrap()
                        .to_owned()
                        .parse::<i32>()
                        .unwrap_or_default(),
                    second_range
                        .next()
                        .unwrap()
                        .to_owned()
                        .parse::<i32>()
                        .unwrap_or_default(),
                ),
            )
        })
        .map(|((n1, n2), (m1, m2))| {
            // ((n1 <= m1 && n2 >= m2)
            //     || (m1 <= n1 && m2 >= n2)) // part 1
            (n1 <= m1 && n2 >= m1)
                || (n1 <= m2 && n2 >= m2)
                || (m1 <= n1 && m2 >= n1)
                || (m1 <= n2 && m2 >= n2) // part 2
        })
        .filter(|&logic| logic == true)
        .collect::<Vec<_>>();

    println!("{:?}", pairs.len())
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
