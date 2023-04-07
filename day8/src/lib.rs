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
    let heights = fin
        .lines()
        .map(|line| line_to_values(line.unwrap()))
        .collect::<Vec<_>>();

    if args.part2 {
        let answer = part2(heights)?;
        println!("{}", answer);
    } else {
        let answer = part1(heights)?;
        println!("{}", answer);
    }

    Ok(())
}

fn part2(heights: Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
    let mut max_score = 0;
    let n = heights.len();

    for i in 0..n {
        for j in 0..n {
            let mut score = 1;
            let curr_height = heights[i][j];

            // left
            let mut view_dist = 0;
            for ix in (0..i).rev() {
                view_dist += 1;
                if heights[ix][j] >= curr_height {
                    break;
                }
            }
            score *= view_dist;

            // right
            let mut view_dist = 0;
            for ix in (i + 1)..n {
                view_dist += 1;
                if heights[ix][j] >= curr_height {
                    break;
                }
            }
            score *= view_dist;

            let mut view_dist = 0;
            for jx in (0..j).rev() {
                view_dist += 1;
                if heights[i][jx] >= curr_height {
                    break;
                }
            }
            score *= view_dist;

            let mut view_dist = 0;
            for jx in (j + 1)..n {
                view_dist += 1;
                if heights[i][jx] >= curr_height {
                    break;
                }
            }
            score *= view_dist;

            if score > max_score {
                max_score = score;
            }
        }
    }

    Ok(max_score)
}

pub fn viewing_dist(height: u32, others: &[u32]) -> u32 {
    let mut dist = 0;

    for h in others {
        dist += 1;
        if h >= &height {
            break;
        }
    }

    dist
}

fn part1(heights: Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
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

    let n_visible = visibility.iter().map(|x| x.iter().sum::<u32>()).sum();

    Ok(n_visible)
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
    use super::*;

    #[test]
    fn test_viewing_dist() {
        let others = vec![4, 5, 3];
        assert_eq!(viewing_dist(2, &others), 1);
        assert_eq!(viewing_dist(4, &others), 1);
        assert_eq!(viewing_dist(5, &others), 2);
        assert_eq!(viewing_dist(6, &others), 3);
    }
}
