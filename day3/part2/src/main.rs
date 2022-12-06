use std::collections::HashSet;
use std::io;
use std::io::BufRead;

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

fn find_common_character(s1: &str, s2: &str, s3: &str) -> char {
    let m1 = string_to_hashset(s1);
    let m2 = string_to_hashset(s2);
    let m3 = string_to_hashset(s3);

    // TODO: look into why .copied() is necessary here
    // https://users.rust-lang.org/t/intersection-of-hashsets/32351
    let sect1: HashSet<char> = m1.intersection(&m2).copied().collect();
    let sect2: Vec<char> = sect1.intersection(&m3).copied().collect();

    sect2[0]
}

fn parse_rucksacks<R: BufRead>(reader: &mut R) -> Vec<char> {
    let mut badges: Vec<char> = Vec::new();
    let mut group: [&str; 3];

    for (i, line) in reader.lines().enumerate() {
        let contents = match line {
            Ok(x) => x,
            Err(_) => continue,
        };

        let idx = i % 3;

        group[idx] = &contents;

        let badge = find_common_character(group[0], group[1], group[2]);
        badges.push(badge);
    }

    badges
}

fn main() {
    let mut reader = io::stdin().lock();
    let badges = parse_rucksacks(&mut reader);

    let mut total_priority = 0;
    for badge in badges {
        total_priority += priority(badge);
    }

    println!("Total priority: {}", total_priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_string_to_hashmap() {
    //     let mut map = string_to_hashmap("ada");

    //     // TODO: look up idiomatic way to get values out of hashmap,
    //     // i.e. .entry() vs .get()
    //     assert_eq!(*map.entry('a').or_default(), 2);
    //     assert_eq!(*map.entry('d').or_default(), 1);
    // }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_find_common_character() {
        assert_eq!(find_common_character("s1", "s2", "s3"), 's');
    }
}
