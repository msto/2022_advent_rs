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

    #[arg(long = "part2", help = "Use part2 logic", default_value_t = false)]
    part2: bool,
}

pub fn get_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();

    Ok(args)
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;
    // let mut lines = fin.lines().filter_map(|x| x.ok());

    let x = fin
        .lines()
        .map(|line| line_to_values(line.unwrap()))
        .collect::<Vec<_>>();

    println!("{}", x[1][2]);

    Ok(())
}

fn line_to_values(line: String) -> Vec<u32> {
    line.chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>()
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}
