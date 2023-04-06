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

    let heights = fin
        .lines()
        .map(|line| line_to_values(line.unwrap()))
        .collect::<Vec<_>>();

    let n = heights.len();
    let mut visibility = vec![vec![0; n]; n];

    // set visible exterior
    for i in 0..n {
        for j in 0..n {
            if i == 0 || i == n - 1 || j == 0 || j == n - 1 {
                visibility[i][j] = 1;
            }
        }
    }

    // check along rows
    for i in 0..n {
        // forward
        let mut idx = 0; // track location of highest tree so far
        for j in 1..n {
            if heights[i][j] > heights[i][idx] {
                idx = j;
                visibility[i][j] = 1;
            }
        }
        // and backwards
        let mut idx = n - 1; // track location of highest tree so far
        for j in (0..(n - 1)).rev() {
            if heights[i][j] > heights[i][idx] {
                idx = j;
                visibility[i][j] = 1;
            }
        }
    }

    // and along columns
    for j in 0..n {
        // forward
        let mut idx = 0; // track location of highest tree so far
        for i in 1..n {
            if heights[i][j] > heights[idx][j] {
                idx = i;
                visibility[i][j] = 1;
            }
        }
        // and backwards
        let mut idx = n - 1; // track location of highest tree so far
        for i in (0..(n - 1)).rev() {
            if heights[i][j] > heights[idx][j] {
                idx = i;
                visibility[i][j] = 1;
            }
        }
    }

    // println!("{:?}", visibility);

    let n_visible: i32 = visibility.iter().map(|x| x.iter().sum::<i32>()).sum();
    println!("{}", n_visible);

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
