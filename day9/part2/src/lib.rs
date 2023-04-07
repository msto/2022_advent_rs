use clap::Parser;
use std::{
    collections::HashSet,
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
    let mut rope = Rope {
        knots: [[0, 0]; 10],
        tail_positions: HashSet::new(),
    };

    fin.lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| parse_line(x).ok())
        .for_each(|(dir, dist)| rope.mv(dir, dist));

    println!("{}", rope.tail_positions.len());

    Ok(())
}

fn parse_line(line: String) -> Result<(char, i32), Box<dyn Error>> {
    let mut data = line.split_whitespace().into_iter();
    let direction = data.next().unwrap().chars().next().unwrap();
    let distance = data.next().unwrap().parse::<i32>()?;

    Ok((direction, distance))
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}

struct Rope {
    knots: [[i32; 2]; 10],
    tail_positions: HashSet<[i32; 2]>,
}

impl Rope {
    fn mv(&mut self, direction: char, dist: i32) {
        for _ in 0..dist {
            self.move_head(direction);
            self.tail_positions.insert(self.knots[9]);
        }
    }

    fn move_head(&mut self, direction: char) {
        match direction {
            'R' => self.knots[0][0] += 1,
            'L' => self.knots[0][0] -= 1,
            'U' => self.knots[0][1] += 1,
            'D' => self.knots[0][1] -= 1,
            _ => panic!("Unknown direction: {}", direction),
        };

        for i in 1..10 {
            if !self.is_touching(i) {
                self.move_knot(i, direction);
            } else {
                break;
            }
        }
    }

    fn is_touching(&self, idx: usize) -> bool {
        (self.knots[idx][0] - self.knots[idx - 1][0]).abs() <= 1
            && (self.knots[idx][1] - self.knots[idx - 1][1]).abs() <= 1
    }

    fn move_knot(&mut self, idx: usize, direction: char) {
        match direction {
            'R' | 'L' => self.knots[idx][1] = self.knots[idx - 1][1],
            'U' | 'D' => self.knots[idx][0] = self.knots[idx - 1][0],
            _ => panic!("Unknown direction: {}", direction),
        };
        match direction {
            'R' => self.knots[idx][0] = self.knots[idx - 1][0] - 1,
            'L' => self.knots[idx][0] = self.knots[idx - 1][0] + 1,
            'U' => self.knots[idx][1] = self.knots[idx - 1][1] - 1,
            'D' => self.knots[idx][1] = self.knots[idx - 1][1] + 1,
            _ => panic!("Unknown direction: {}", direction),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_is_touching() {
        let mut rope = Rope {
            knots: [[0, 0]; 10],
            tail_positions: HashSet::new(),
        };

        for pos in [[0, 0], [0, 1], [1, 1], [-1, 0], [-1, -1], [-1, 1]] {
            rope.knots[0] = pos;
            assert!(rope.is_touching(1));
        }

        for pos in [[0, 2], [1, 2], [-1, 2], [-2, 1]] {
            rope.knots[0] = pos;
            assert!(!rope.is_touching(1));
        }
    }

    #[test]
    fn test_rope_move() {
        let mut rope = Rope {
            knots: [[0, 0]; 10],
            tail_positions: HashSet::new(),
        };

        rope.mv('R', 4);
        assert_eq!(rope.knots[0], [4, 0]);
        assert_eq!(rope.knots[1], [3, 0]);
        assert_eq!(rope.knots[2], [2, 0]);
        assert_eq!(rope.knots[3], [1, 0]);
        for i in 4..10 {
            assert_eq!(rope.knots[i], [0, 0]);
        }

        rope.mv('U', 4);
        assert_eq!(rope.knots[0], [4, 4]);
        assert_eq!(rope.knots[1], [4, 3]);
        assert_eq!(rope.knots[2], [4, 2]);
        assert_eq!(rope.knots[3], [3, 2]);
        assert_eq!(rope.knots[4], [2, 2]);
        assert_eq!(rope.knots[5], [1, 1]);
        for i in 6..10 {
            assert_eq!(rope.knots[i], [0, 0]);
        }

        rope.mv('L', 3);
        assert_eq!(rope.knots[0], [1, 4]);
        assert_eq!(rope.knots[1], [2, 4]);

        rope.mv('D', 1);
        assert_eq!(rope.knots[0], [1, 3]);
        assert_eq!(rope.knots[1], [2, 4]);

        rope.mv('R', 4);
        assert_eq!(rope.knots[0], [5, 3]);
        assert_eq!(rope.knots[1], [4, 3]);

        rope.mv('D', 1);
        assert_eq!(rope.knots[0], [5, 2]);
        assert_eq!(rope.knots[1], [4, 3]);

        rope.mv('L', 5);
        assert_eq!(rope.knots[0], [0, 2]);
        assert_eq!(rope.knots[1], [1, 2]);

        rope.mv('R', 2);
        assert_eq!(rope.knots[0], [2, 2]);
        assert_eq!(rope.knots[1], [1, 2]);
    }
}
