pub fn parse_crates(line: &str) -> Vec<Option<char>> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            if chunk[0] == '[' {
                Some(chunk[1])
            } else {
                None
            }
        })
        .collect()
}

pub struct Move {
    n: usize,
    src: usize,
    dst: usize,
}

pub fn parse_move(line: &str) -> Move {
    let data: Vec<usize> = line
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .filter_map(|(_, x)| x.parse::<usize>().ok())
        .collect();

    Move {
        n: data[0],
        src: data[1] - 1,
        dst: data[2] - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crates() {
        let line = "[A] [B]     [D]\n";
        let crates = parse_crates(&line);

        assert_eq!(crates.len(), 4);
        assert_eq!(crates[0], Some('A'));
        assert_eq!(crates[1], Some('B'));
        assert_eq!(crates[2], None);
        assert_eq!(crates[3], Some('D'));
    }

    #[test]
    fn test_parse_move() {
        let line = "move 1 from 3 to 5\n";
        let mv = parse_move(line);

        assert_eq!(mv.n, 1);
        assert_eq!(mv.src, 2);
        assert_eq!(mv.dst, 4);
    }
}
