use clap::Parser;
use itertools::Itertools;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use day11::parse_monkey;

const N_ROUNDS: usize = 20;

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Parse input and apply logic
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;

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

    println!("{}", monkeys.len());

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
