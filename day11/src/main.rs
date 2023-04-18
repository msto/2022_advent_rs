use clap::Parser;
use itertools::Itertools;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use day11::parse_line;

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Parse input and apply logic
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;

    fin.lines()
        .filter_map(|x| x.ok())
        .chunks(7)
        .into_iter()
        .map(|x| {
            x.filter(|x| !x.is_empty())
                .collect::<Vec<String>>()
                .join("\n")
        })
        .for_each(|x| println!("===\n{}\n===", x));

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
