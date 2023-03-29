use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

fn parse_range(s: &str) -> Range<i32> {
    let r: Vec<i32> = s.split("-").filter_map(|x| x.parse::<i32>().ok()).collect();

    Range {
        start: r[0],
        end: r[1] + 1,
    }
}

fn parse_ranges(line: &str) -> Vec<Range<i32>> {
    line.split(",").map(|x| parse_range(x)).collect()
}

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
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;

    fin.lines().for_each(|x| println!("{}", x.unwrap()));

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let start = 2;
        let end = 6;
        let r = parse_range(&format!("{}-{}", start, end));

        assert_eq!(r.start, start);
        assert_eq!(r.end, end + 1);
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges("2-6,3-4");

        assert_eq!(ranges[0].start, 2);
        assert_eq!(ranges[0].end, 7);
        assert_eq!(ranges[1].start, 3);
        assert_eq!(ranges[1].end, 5);
    }
}
