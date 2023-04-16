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

fn find_common_character<I>(lines: I) -> char
where
    I: IntoIterator<Item = String>,
{
    // TODO: look into why .copied() is necessary here
    // https://users.rust-lang.org/t/intersection-of-hashsets/32351
    lines
        .into_iter()
        .map(|x| string_to_hashset(x.as_str()))
        .reduce(|x, y| x.intersection(&y).copied().collect())
        .unwrap()
        .drain()
        .next()
        .unwrap()
}

fn main() {
    let reader = io::stdin().lock();
    let mut total_priority = 0;

    for chunk in &reader.lines().filter_map(|x| x.ok()).chunks(3) {
        let common = find_common_character(chunk);
        // println!("{:?}", common);
        total_priority += priority(common);
    }

    println!("{}", total_priority);
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
        let strings = vec!["s1", "s2", "s3"].into_iter().map(|x| x.to_owned());
        assert_eq!(find_common_character(strings), 's');
    }
}
