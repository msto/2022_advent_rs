use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Input file", id = "FILE", default_value = "-")]
    fin: String,
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn get_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();

    Ok(args)
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;

    let n_overlap = fin
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| day4::parse_ranges(&(line)))
        // .filter(|x| day4::either_contains(&x[0], &x[1]))
        .filter(|x| day4::overlaps(&x[0], &x[1]))
        .count();

    // println!("{} pairs fully overlap", n_overlap);
    println!("{} pairs partially overlap", n_overlap);

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
