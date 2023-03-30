use clap::Parser;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    rc::Rc,
};

enum FsNodeType {
    Dir,
    File,
}

struct FsNode {
    kind: FsNodeType,
    name: String,
    dsize: usize,
    parent: Option<Rc<FsNode>>,
    contents: Option<HashMap<String, Rc<FsNode>>>,
}

fn parse_node(line: &str, curr_dir: Option<Rc<FsNode>>) -> FsNode {
    let mut data = line.split_whitespace();
    let dtype = data.next().unwrap();
    let name = data.next().unwrap();

    let kind = if line.starts_with("dir ") {
        FsNodeType::Dir
    } else {
        FsNodeType::File
    };

    let dsize = match kind {
        FsNodeType::Dir => 0,
        FsNodeType::File => dtype.parse::<usize>().unwrap(),
    };

    let contents = match kind {
        FsNodeType::Dir => Some(HashMap::new()),
        FsNodeType::File => None,
    };

    FsNode {
        kind: kind,
        name: name.to_string(),
        dsize: dsize,
        parent: curr_dir,
        contents: contents,
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

        assert!(matches!(node.kind, FsNodeType::Dir));
        assert_eq!(node.name, "abc");
        assert_eq!(node.dsize, 0);
        assert!(node.parent.is_none());
        assert!(node.contents.is_some());
        assert!(node.contents.unwrap().len() == 0);
    }

    #[test]
    fn test_parse_node_file() {
        let line = "123 def";
        let node = parse_node(&line, None);

        assert!(matches!(node.kind, FsNodeType::File));
        assert_eq!(node.name, "def");
        assert_eq!(node.dsize, 123);
        assert!(node.parent.is_none());
        assert!(node.contents.is_none());
    }
}
