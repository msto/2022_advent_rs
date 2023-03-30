use clap::Parser;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    rc::Rc,
};

enum FsNode {
    Dir {
        name: String,
        contents: HashMap<String, Rc<FsNode>>,
        parent: Option<Rc<FsNode>>,
    },
    File {
        name: String,
        size: usize,
        parent: Option<Rc<FsNode>>,
    },
}

fn parse_node(line: &str, curr_dir: Option<Rc<FsNode>>) -> FsNode {
    let mut data = line.split_whitespace();
    let dtype = data.next().unwrap();
    let name = data.next().unwrap();

    if line.starts_with("dir ") {
        FsNode::Dir {
            name: name.to_string(),
            contents: HashMap::new(),
            parent: curr_dir,
        }
    } else {
        FsNode::File {
            name: name.to_string(),
            size: dtype.parse::<usize>().unwrap(),
            parent: curr_dir,
        }
    }
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

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let fin = open(&args.fin)?;
    let mut lines = fin.lines().filter_map(|x| x.ok());

    // skip cd to root
    lines.next();
    let mut root = FsNode::Dir {
        name: "/".to_string(),
        contents: HashMap::new(),
        parent: None,
    };

    let mut curr_dir = Rc::new(root);

    for line in lines {
        if line.starts_with("$") {
            continue;
        }

        let node = parse_node(&line, Some(Rc::clone(&curr_dir)));

        // if line.starts_with("$ cd") {
        //     let name = line.trim_start_matches("$ cd ");
        //     curr_dir = FsEntry::Dir {
        //         name: name.to_string(),
        //         contents: HashMap::new(),
        //     };
        //     curr_tree.push(curr_dir);
        // } else if line.starts_with("$ ls") {
        // } else {
        // }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_parse_node_dir() {
        let line = "dir abc";
        let node = parse_node(&line, None);

        assert!(matches!(node, FsNode::Dir { .. }));

        match node {
            FsNode::Dir {
                name,
                contents,
                parent,
            } => {
                assert_eq!(name, "abc");
                assert_eq!(contents.len(), 0);
                assert!(parent.is_none());
            }
            FsNode::File { .. } => {}
        };
    }

    #[test]
    fn test_parse_node_file() {
        let line = "123 def";
        let node = parse_node(&line, None);

        assert!(matches!(node, FsNode::File { .. }));

        match node {
            FsNode::File { name, size, parent } => {
                assert_eq!(name, "def");
                assert_eq!(size, 123);
                assert!(parent.is_none());
            }
            FsNode::Dir { .. } => {}
        };
    }
}
