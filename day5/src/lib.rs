fn parse_crates(line: &str) -> Vec<Option<char>> {
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
}
