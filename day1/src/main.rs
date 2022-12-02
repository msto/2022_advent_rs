use std::io;
use std::io::BufRead;
// use std::io::BufReader;

/// Parse calorie totals from input files.
///
/// Input files are formatted with calories listed on consecutive lines, with
/// blank lines separating inventories of different elves.
///
/// The total calories carried by each elf are stored and returned in a vector.
///
fn parse_totals<R: BufRead>(reader: &mut R) -> Vec<u32> {
    let mut totals: Vec<u32> = Vec::new();
    let mut curr_total: u32 = 0;

    for line in reader.lines() {
        let calories = line.expect("Failed to read");

        // Empty lines denote breaks between "elves"
        if calories.is_empty() {
            totals.push(curr_total);
            curr_total = 0;
        // Add calories to current elf
        } else {
            let calories: u32 = match calories.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            curr_total += calories;
        }
    }

    totals
}

fn part1(totals: &Vec<u32>) {
    println!("Part 1: {}", totals.iter().max().unwrap());
}

fn part2(totals: &mut Vec<u32>) {
    totals.sort();
    totals.reverse();

    let total: u32 = totals[..3].iter().sum();

    println!("Part 2: {}", total);
}

fn main() {
    // let mut reader = BufReader::new(io::stdin());
    let mut reader = io::stdin().lock(); // StdinLock implements BufRead
    let mut totals = parse_totals(&mut reader);

    part1(&totals);
    part2(&mut totals);
}
