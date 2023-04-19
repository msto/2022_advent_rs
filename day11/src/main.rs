use clap::Parser;
use itertools::Itertools;
use num::Integer;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use day11::parse_monkey;

const N_ROUNDS: usize = 10000;
const RELIEF_FACTOR: usize = 1;

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Parse input and apply logic
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;

    // parse monkeys
    let mut monkeys = fin
        .lines()
        .filter_map(|x| x.ok())
        .chunks(7)
        .into_iter()
        .map(|x| {
            x.filter(|x| !x.is_empty())
                .collect::<Vec<String>>()
                .join("\n")
        })
        .filter_map(|x| parse_monkey(&x).ok())
        .collect::<Vec<_>>();

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .reduce(|x, y| x.lcm(&y))
        .unwrap();

    for _ in 0..N_ROUNDS {
        // apply a round of inspection
        for i in 0..monkeys.len() {
            // split monkeys into flanking mutable vectors so we can
            // push contents from one monkey onto others
            // https://stackoverflow.com/questions/49143770/efficiently-mutate-a-vector-while-also-iterating-over-the-same-vector
            let (left, mid_right) = monkeys.split_at_mut(i);
            let (mid, right) = mid_right.split_at_mut(1);
            let monkey = &mut mid[0];

            // track total number of inspections
            monkey.n_inspections += monkey.items.len();

            // update the worry level and then move the item appropriately
            monkey
                .items
                .drain(..)
                .map(|item| monkey.operation.apply(item) / RELIEF_FACTOR)
                .for_each(|new| {
                    let dst = match new % monkey.divisor {
                        0 => monkey.true_dst,
                        _ => monkey.false_dst,
                    };
                    let managed = match new % lcm {
                        0 => lcm,
                        _ => new % lcm,
                    };
                    match dst < i {
                        true => left[dst].add(managed),
                        false => right[(dst - i - 1)].add(managed),
                    }
                });
        }
    }

    monkeys.sort_by_key(|x| x.n_inspections);

    let biz = monkeys[monkeys.len() - 1].n_inspections * monkeys[monkeys.len() - 2].n_inspections;
    println!("monkey business: {}", biz);

    Ok(())
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Input file", id = "FILE", default_value = "-")]
    fin: String,
}

pub fn get_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();

    Ok(args)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
