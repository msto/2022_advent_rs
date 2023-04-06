use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

/// Compute "priority" of an item (ascii value - offset)
fn priority(c: char) -> u32 {
    if !c.is_ascii_alphabetic() {
        panic!("only lower and uppercase letters permitted.")
    }

    // problem formulated to have different offsets for lower/upper
    // a..z = 1..26; A..Z = 27..52
    if c.is_ascii_uppercase() {
        (c as u32) - 38
    } else {
        (c as u32) - 96
    }
}

/// Count character instances in a string.
fn string_to_hashset(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for c in s.chars() {
        set.insert(c);
    }

    set
}

fn find_common_character<'a, I>(lines: I) -> char
where
    I: IntoIterator<Item = &'a str>,
{
    // TODO: look into why .copied() is necessary here
    // https://users.rust-lang.org/t/intersection-of-hashsets/32351
    lines
        .into_iter()
        .map(string_to_hashset)
        .reduce(|x, y| x.intersection(&y).copied().collect())
        .unwrap()
        .drain()
        .next()
        .unwrap()
}

fn main() {
    let mut reader = io::stdin().lock();

    // let common = reader
    //     .lines()
    //     .chunks(3)
    //     .into_iter()
    //     .map(find_common_character)
    //     .next();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_string_to_hashset() {
        let mut s = HashSet::new();
        s.insert('c');
        s.insert('b');

        assert_eq!(string_to_hashset(&"bcbccb"), s);
    }

    #[test]
    fn test_find_common_character() {
        assert_eq!(find_common_character(vec!["s1", "s2", "s3"]), 's');
    }
}
