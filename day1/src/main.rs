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

/// Report the maximum calories carried by a single elf
/// (aka unpack and dereference the max of a Vector)
fn max_total(totals: &Vec<u32>) -> u32 {
    *totals.iter().max().unwrap()
}

/// Report the total calories carried by the elves with the three highest loads
fn top3_total(totals: &mut Vec<u32>) -> u32 {
    totals.sort();
    totals.reverse();

    totals[..3].iter().sum()
}

fn main() {
    // let mut reader = BufReader::new(io::stdin());
    let mut reader = io::stdin().lock(); // StdinLock implements BufRead
    let mut totals = parse_totals(&mut reader);

    println!("Part 1: {}", max_total(&totals));
    println!("Part 2: {}", top3_total(&mut totals));
}
