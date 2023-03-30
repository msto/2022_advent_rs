use clap::Parser;
use std::{
    collections::VecDeque,
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

    #[arg(
        long = "part2",
        help = "Use part2 logic (move crates as stack)",
        default_value_t = false
    )]
    part2: bool,
}

fn get_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();

    Ok(args)
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut lines = open(&args.fin)?.lines();

    // Load "crate contents" (characters) by row
    let mut crate_rows: VecDeque<Vec<Option<char>>> = VecDeque::new();
    loop {
        let line = lines.next().unwrap()?;

        if line.trim_start().chars().nth(0).unwrap() != '[' {
            // skip blank line following numbers
            lines.next();
            break;
        }

        let row = day5::parse_crates(&line);
        crate_rows.push_front(row);
    }

    // Transpose rows of crates into vertical stacks, discarding non-existent crates
    let n_stacks = crate_rows[0].len();
    let mut stacks: Vec<VecDeque<char>> = std::iter::repeat(VecDeque::new())
        .take(n_stacks)
        .collect::<Vec<_>>();
    for row in crate_rows {
        for (i, copt) in row.iter().enumerate() {
            match copt {
                Some(c) => stacks[i].push_front(*c),
                None => (),
            };
        }
    }

    // Parse moves and rearrange crates
    for line in lines {
        let mv = day5::parse_move(&line.unwrap());

        if args.part2 {
            let bottom = stacks[mv.src].split_off(mv.n);
            for _ in 0..mv.n {
                let c = stacks[mv.src].pop_back().unwrap();
                stacks[mv.dst].push_front(c);
            }
            stacks[mv.src] = bottom;
        } else {
            for _ in 0..mv.n {
                let c = stacks[mv.src].pop_front().unwrap();
                stacks[mv.dst].push_front(c);
            }
        }
    }

    // Print first/top crate/character in each stack
    println!("{}", stacks.into_iter().map(|x| x[0]).collect::<String>());

    // for stack in stacks {
    // println!("{}", stack.into_iter().collect::<String>());
    // }

    // println!("{}", lines.next().unwrap()?);

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
