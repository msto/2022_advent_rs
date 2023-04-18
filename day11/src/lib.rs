use std::error::Error;

const RELIEF_FACTOR: usize = 3;

pub struct Monkey {
    items: Vec<usize>,
    op: fn(usize) -> usize,
    divisor: usize,
    true_dst: usize,
    false_dst: usize,
}

impl Monkey {
    fn inspect(&self) -> usize {
        0
    }

    fn add(&mut self, item: usize) {
        self.items.push(item);
    }
}

/// Parse line of input
pub fn parse_line(line: String) -> Result<Monkey, Box<dyn Error>> {
    let mut data = line.split_whitespace();
    let op_name = data.next();

    match op_name {
        _ => Err("Invalid operation".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    fn make_test_monkey() -> Monkey {
        Monkey {
            items: vec![92, 73, 86],
            op: |x| x * 5,
            divisor: 11,
            true_dst: 3,
            false_dst: 4,
        }
    }

    #[test]
    fn test_monkey_init() {
        let monkey = make_test_monkey();
        assert_eq!((monkey.op)(monkey.items[0]), 92 * 5)
    }

    #[test]
    fn test_monkey_add() {
        let mut monkey = make_test_monkey();
        monkey.add(7);
        assert_eq!(monkey.items, vec![92, 73, 86, 7]);
    }
}
