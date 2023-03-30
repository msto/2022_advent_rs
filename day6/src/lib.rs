use std::collections::HashSet;

pub fn find_marker(buffer: &str) -> usize {
    let pos = buffer
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|x| x.into_iter().collect::<HashSet<_>>().len() == 4)
        .unwrap_or(0);

    pos + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let buffer = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(&buffer), 5);
    }

    #[test]
    fn test_example2() {
        let buffer = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(&buffer), 6);
    }

    #[test]
    fn test_example3() {
        let buffer = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(&buffer), 10);
    }

    #[test]
    fn test_example4() {
        let buffer = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(&buffer), 11);
    }
}
