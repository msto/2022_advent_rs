use std::collections::HashMap;
use std::io;
use std::io::BufRead;

struct Rucksack {
    compartment1: HashMap<char, u32>,
    compartment2: HashMap<char, u32>,
}

impl Rucksack {
    fn from_contents(contents: &str) -> Self {
        let median = contents.len() / 2;
        let compartment1 = string_to_hashmap(&contents[..median]);
        let compartment2 = string_to_hashmap(&contents[median..]);

        Self {
            compartment1,
            compartment2,
        }
    }
}

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
fn string_to_hashmap(s: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    map
}

// Find shared key between two maps.
// Assumes only key is shared and reports the first found.
// TODO: make generic
fn find_shared_key(m1: &HashMap<char, u32>, m2: &HashMap<char, u32>) -> Option<char> {
    for (key, _) in m1 {
        if m2.contains_key(key) {
            return Some(*key);
        }
    }

    None
}

fn parse_rucksacks<R: BufRead>(reader: &mut R) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();

    for line in reader.lines() {
        let contents = match line {
            Ok(x) => x,
            Err(_) => continue,
        };

        let rucksack = Rucksack::from_contents(&contents);
        rucksacks.push(rucksack);
    }

    rucksacks
}

fn main() {
    let mut reader = io::stdin().lock();
    let rucksacks = parse_rucksacks(&mut reader);

    let total_priority = 0;
    for rucksack in rucksacks {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_hashmap() {
        let mut map = string_to_hashmap("ada");

        // TODO: look up idiomatic way to get values out of hashmap,
        // i.e. .entry() vs .get()
        assert_eq!(*map.entry('a').or_default(), 2);
        assert_eq!(*map.entry('d').or_default(), 1);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_find_shared_key() {
        let mut m1 = HashMap::new();
        m1.insert('a', 1);
        m1.insert('b', 1);

        let mut m2 = HashMap::new();
        m2.insert('c', 1);
        m2.insert('b', 1);

        let shared_key = find_shared_key(&m1, &m2);
        assert_eq!(shared_key, Some('b'));
    }

    #[test]
    fn test_rucksack_from_contents() {
        let mut rucksack = Rucksack::from_contents("ABABDEAB");

        assert_eq!(*rucksack.compartment1.entry('A').or_default(), 2);
        assert_eq!(*rucksack.compartment1.entry('B').or_default(), 2);
        assert_eq!(*rucksack.compartment2.entry('A').or_default(), 1);
        assert_eq!(*rucksack.compartment2.entry('D').or_default(), 1);
    }
}
