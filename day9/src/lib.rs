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
    let mut rope = Rope {
        head: [0, 0],
        tail: [0, 0],
    };

    fin.lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| parse_line(x).ok())
        .for_each(|(dir, dist)| rope.mv(dir, dist));

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
    head: [i32; 2],
    tail: [i32; 2],
}

impl Rope {
    fn mv(&mut self, direction: char, dist: i32) {
        for _ in 0..dist {
            self.move_head(direction);
        }
    }

    fn move_head(&mut self, direction: char) {
        match direction {
            'R' => self.head[0] += 1,
            'L' => self.head[0] -= 1,
            'U' => self.head[1] += 1,
            'D' => self.head[1] -= 1,
            _ => panic!("Unknown direction: {}", direction),
        };

        if !self.is_touching() {
            self.move_tail(direction);
        }
    }

    fn is_touching(&self) -> bool {
        (self.head[0] - self.tail[0]).abs() <= 1 && (self.head[1] - self.tail[1]).abs() <= 1
    }

    fn move_tail(&mut self, direction: char) {
        match direction {
            'R' | 'L' => self.tail[1] = self.head[1],
            'U' | 'D' => self.tail[0] = self.head[0],
            _ => panic!("Unknown direction: {}", direction),
        };
        match direction {
            'R' => self.tail[0] = self.head[0] - 1,
            'L' => self.tail[0] = self.head[0] + 1,
            'U' => self.tail[1] = self.head[1] - 1,
            'D' => self.tail[1] = self.head[1] + 1,
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
            head: [0, 0],
            tail: [0, 0],
        };

        for pos in [[0, 0], [0, 1], [1, 1], [-1, 0], [-1, -1], [-1, 1]] {
            rope.head = pos;
            assert!(rope.is_touching());
        }

        for pos in [[0, 2], [1, 2], [-1, 2], [-2, 1]] {
            rope.head = pos;
            assert!(!rope.is_touching());
        }
    }

    #[test]
    fn test_rope_move() {
        let mut rope = Rope {
            head: [0, 0],
            tail: [0, 0],
        };

        rope.mv('R', 4);
        assert_eq!(rope.head, [4, 0]);
        assert_eq!(rope.tail, [3, 0]);

        rope.mv('U', 4);
        assert_eq!(rope.head, [4, 4]);
        assert_eq!(rope.tail, [4, 3]);

        rope.mv('L', 3);
        assert_eq!(rope.head, [1, 4]);
        assert_eq!(rope.tail, [2, 4]);

        rope.mv('D', 1);
        assert_eq!(rope.head, [1, 3]);
        assert_eq!(rope.tail, [2, 4]);

        rope.mv('R', 4);
        assert_eq!(rope.head, [5, 3]);
        assert_eq!(rope.tail, [4, 3]);

        rope.mv('D', 1);
        assert_eq!(rope.head, [5, 2]);
        assert_eq!(rope.tail, [4, 3]);

        rope.mv('L', 5);
        assert_eq!(rope.head, [0, 2]);
        assert_eq!(rope.tail, [1, 2]);

        rope.mv('R', 2);
        assert_eq!(rope.head, [2, 2]);
        assert_eq!(rope.tail, [1, 2]);
    }
}
