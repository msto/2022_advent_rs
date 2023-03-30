use std::collections::HashSet;

pub fn find_marker(buffer: &str, n: usize) -> usize {
    let pos = buffer
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .position(|x| x.into_iter().collect::<HashSet<_>>().len() == n)
        .unwrap_or(0);

    pos + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let buffer = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(&buffer, 4), 5);
        assert_eq!(find_marker(&buffer, 14), 23);
    }

    #[test]
    fn test_example2() {
        let buffer = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(&buffer, 4), 6);
        assert_eq!(find_marker(&buffer, 14), 23);
    }

    #[test]
    fn test_example3() {
        let buffer = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(&buffer, 4), 10);
        assert_eq!(find_marker(&buffer, 14), 29);
    }

    #[test]
    fn test_example4() {
        let buffer = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(&buffer, 4), 11);
        assert_eq!(find_marker(&buffer, 14), 26);
    }
}
