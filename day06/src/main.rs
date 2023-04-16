use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "Input file", id = "FILE", default_value = "-")]
    fin: String,

    #[arg(long = "part2", help = "Use part2 logic", default_value_t = false)]
    part2: bool,
}

fn get_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();

    Ok(args)
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut lines = open(&args.fin)?.lines();
    let buffer = lines.next().unwrap()?;

    let marker_pos: usize;
    if args.part2 {
        marker_pos = day6::find_marker(&buffer, 14);
    } else {
        marker_pos = day6::find_marker(&buffer, 4);
    }

    println!("First marker appears after character {}", marker_pos);

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
