use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use day10::{parse_line, CPU};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Parse input and apply logic
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU {
        ..Default::default()
    };

    let fin = open(&args.fin)?;
    fin.lines()
        .filter_map(|x| parse_line(x.ok().unwrap()).ok()) // TODO: is there a cleaner way to unpack the Result lines?
        .for_each(|instruction| cpu.execute(instruction));

    println!("{}", cpu.interesting_strength);

    cpu.display();

    Ok(())
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Input file", id = "FILE", default_value = "-")]
    fin: String,

    #[arg(long = "part2", help = "Use part2 logic", default_value_t = false)]
    part2: bool,
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
