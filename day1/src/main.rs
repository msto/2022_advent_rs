use std::io;

// fn part1() {
//     let mut max_total: u32 = 0;
//     let mut curr_total: u32 = 0;

//     for line in io::stdin().lines() {
//         let calories = line.expect("Failed to read");

//         // Empty lines denote breaks between "elves"
//         if calories.is_empty() {
//             if curr_total > max_total {
//                 max_total = curr_total;
//             }
//             curr_total = 0;
//         // Add calories to current elf
//         } else {
//             let calories: u32 = match calories.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => continue,
//             };
//             curr_total += calories;
//         }
//     }

//     println!("Max calories: {max_total}");
// }

fn main() {
    let mut totals: Vec<u32> = Vec::new();
    let mut curr_total = 0;

    for line in io::stdin().lines() {
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

    totals.sort();
    totals.reverse();

    let total: u32 = totals[..3].iter().sum();

    // let max = totals[0:2].sum();
    // let max2 = totals[1];

    println!("{:?}", &totals[..3]);
    println!("{:?}", total);

    // println!("{totals[0]}, {max2}");
}
