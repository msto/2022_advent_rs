use clap::Parser;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use day9::{parse_line, Rope};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// Boilerplate - argument parsing and file IO
#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "Input file", id = "FILE", default_value = "-")]
    fin: String,

    #[arg(long = "part2", help = "Use part2 logic", default_value_t = false)]
    part2: bool,
}

fn get_args() -> Result<Args, Box<dyn Error>> {
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

/// Parse input and apply movement logic
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;
    let mut rope = Rope {
        knots: [[0, 0]; 10],
        tail_positions: HashSet::new(),
    };

    fin.lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| parse_line(x).ok())
        .for_each(|(direction, dist)| rope.pull(direction, dist));

    println!("{}", rope.tail_positions.len());

    Ok(())
}
