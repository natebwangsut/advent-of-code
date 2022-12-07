use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

///
/// Advent of code 2022 - Day 5
///
fn main() {
    println!("advent-of-code: five");

    //
    // Input
    //
    //             [J] [Z] [G]
    //             [Z] [T] [S] [P] [R]
    // [R]         [Q] [V] [B] [G] [J]
    // [W] [W]     [N] [L] [V] [W] [C]
    // [F] [Q]     [T] [G] [C] [T] [T] [W]
    // [H] [D] [W] [W] [H] [T] [R] [M] [B]
    // [T] [G] [T] [R] [B] [P] [B] [G] [G]
    // [S] [S] [B] [D] [F] [L] [Z] [N] [L]
    //  1   2   3   4   5   6   7   8   9
    //
    // Input (example)
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    //

    // File hosts must exist in current path before this produces output

    // Part 1
    // let stack: &mut Vec<Vec<char>> = &mut vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    // let Ok(lines) = read_lines("./input.example") else { return };
    // for line in lines.flatten().skip(5) {

    // Part 2
    let stack: &mut Vec<Vec<char>> = &mut vec![
        vec!['S', 'T', 'H', 'F', 'W', 'R'],
        vec!['S', 'G', 'D', 'Q', 'W'],
        vec!['B', 'T', 'W'],
        vec!['D', 'R', 'W', 'T', 'N', 'Q', 'Z', 'J'],
        vec!['F', 'B', 'H', 'G', 'L', 'V', 'T', 'Z'],
        vec!['L', 'P', 'T', 'C', 'V', 'B', 'S', 'G'],
        vec!['Z', 'B', 'R', 'T', 'W', 'G', 'P'],
        vec!['N', 'G', 'M', 'T', 'C', 'J', 'R'],
        vec!['L', 'G', 'B', 'W'],
    ];
    let Ok(lines) = read_lines("./input") else { return };
    for line in lines.flatten().skip(10) {
        let mut command = line.split(" ");

        command.next();
        let move_item = command.next().unwrap().parse::<usize>().unwrap();

        command.next();
        let from_item = command.next().unwrap().parse::<usize>().unwrap() - 1;

        command.next();
        let to_item = command.next().unwrap().parse::<usize>().unwrap() - 1;

        // println!("{:?}", (move_item, from_item, to_item));

        let mut from = stack[from_item].clone();
        let length = from.len();

        // let to = stack.index(to_item);
        let (new_from, moving_items) = from.split_at_mut(length - move_item);

        println!(
            "moving {:?} from {:?} to {:?}",
            moving_items, stack[from_item], stack[to_item]
        );

        // Part 1 - comment this
        // moving_items.reverse();

        // from.truncate(from.len() - move_item);
        stack[to_item].append(moving_items.to_vec().as_mut());
        stack[from_item] = new_from.to_vec();
    }

    print_stack(stack);
}

///
/// Print result
///
fn print_stack(stacks: &mut Vec<Vec<char>>) -> () {
    let result = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();
    println!("{:?}", result);
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
