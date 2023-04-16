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

impl FsNode {
    fn size(&self) -> usize {
        match self.kind {
            FsNodeType::Dir => self
                .contents
                .as_ref()
                .unwrap()
                .values()
                .map(|x| x.size())
                .sum(),
            FsNodeType::File => self.dsize,
        }
    }

    fn add_child(&self, node: Rc<FsNode>>) {
        self.contents.insert(node.name, node);
    }
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
    let _fin = open(&args.fin)?;
    // let mut lines = fin.lines().filter_map(|x| x.ok());

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

    fn test_FsNode_size() {
        let f1 = FsNode {
            kind: FsNodeType::File,
            name: "file1".to_string(),
            dsize: 123,
            parent: None,
            contents: None,
        };
        let f2 = FsNode {
            kind: FsNodeType::File,
            name: "file2".to_string(),
            dsize: 456,
            parent: None,
            contents: None,
        };

        let test_dir = FsNode {
            kind: FsNodeType::Dir,
            name: "test_dir".to_string(),
            dsize: 0,
            parent: None,
            contents: Some(HashMap::new()),
        };

        test_dir.contents.unwrap().insert(f1.name, &Rc::clone(f1));
        test_dir.contents.unwrap().insert(f1.name, f1);
    }
}
