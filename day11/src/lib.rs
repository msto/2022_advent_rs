use regex::Regex;
use std::error::Error;

const RELIEF_FACTOR: usize = 3;

pub struct Monkey {
    items: Vec<usize>,
    op: fn(usize) -> usize,
    divisor: usize,
    true_dst: usize,
    false_dst: usize,
}

enum Op {
    Add,
    Mult,
}

struct Operation {
    op: Op,
    term: Option<usize>,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        let term = match self.term {
            Some(val) => val,
            None => old,
        };

        match self.op {
            Op::Add => old + term,
            Op::Mult => old * term,
        }
    }
}

fn parse_monkey(monkey_str: &str) -> Monkey {
    let re = Regex::new(
        r"Monkey (\d+):
  Starting items: (.*)
  Operation: (.*)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )
    .unwrap();

    let cap = re.captures(monkey_str).unwrap();

    let items = cap[2]
        .split(", ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    // TODO: parse operation into closure

    let parse_int = |x: &str| -> usize { x.parse::<usize>().unwrap() };

    Monkey {
        items: items,
        op: |x| x,
        divisor: parse_int(&cap[4]),
        true_dst: parse_int(&cap[5]),
        false_dst: parse_int(&cap[6]),
    }
}

fn parse_operation(op_str: &str) -> (String, String) {
    let re = Regex::new(r"new = old (\+|\*) (.*)").unwrap();
    let cap = re.captures(op_str).unwrap();

    // match (cap[1], cap[2]) {
    //     ("+", "old") => |x| x + x,
    //     ("*", "old") => |x| x * x,
    //     ("+", _) => |x| x + cap[2].parse::<usize>().unwrap(),
    //     ("*", _) => |x| x + cap[2].parse::<usize>().unwrap(),
    // }

    (cap[1].to_string(), cap[2].to_string())
}

impl Monkey {
    // fn inspect_items(&mut self) -> impl Iterator<Item = (usize, usize)> {
    //     self.items.drain(..).map(|x| inspect_item(x))
    // }

    fn inspect_item(&self, item: usize) -> (usize, usize) {
        let worry_level = (self.op)(item) / RELIEF_FACTOR;

        let dst = if worry_level % self.divisor == 0 {
            self.true_dst
        } else {
            self.false_dst
        };

        (dst, worry_level)
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

    fn make_test_monkey() -> Monkey {
        Monkey {
            items: vec![79, 98],
            op: |x| x * 19,
            divisor: 23,
            true_dst: 2,
            false_dst: 3,
        }
    }

    #[test]
    fn test_monkey_init() {
        let monkey = make_test_monkey();
        assert_eq!((monkey.op)(monkey.items[0]), 79 * 19)
    }

    #[test]
    fn test_monkey_add() {
        let mut monkey = make_test_monkey();
        monkey.add(7);
        assert_eq!(monkey.items, vec![79, 98, 7]);
    }

    #[test]
    fn test_monkey_inspect_item() {
        let monkey = make_test_monkey();

        let (dst, worry_level) = monkey.inspect_item(monkey.items[0]);
        assert_eq!(worry_level, 500);
        assert_eq!(dst, 3);

        let (dst, worry_level) = monkey.inspect_item(monkey.items[1]);
        assert_eq!(worry_level, 620);
        assert_eq!(dst, 3);
    }

    #[test]
    fn test_parse_monkey() {
        let monkey_str = "Monkey 0:
  Starting items: 92, 73, 86, 83, 65, 51, 55, 93
  Operation: new = old * 5
  Test: divisible by 11
    If true: throw to monkey 3
    If false: throw to monkey 4";

        let monkey = parse_monkey(monkey_str);

        assert_eq!(monkey.items, vec![92, 73, 86, 83, 65, 51, 55, 93]);
        assert_eq!(monkey.divisor, 11);
        assert_eq!(monkey.true_dst, 3);
        assert_eq!(monkey.false_dst, 4);
    }

    #[test]
    fn test_parse_operation() {
        let op_str = "new = old * 5";
        let (op, term) = parse_operation(op_str);
        assert_eq!(op, "*");
        assert_eq!(term, "5");

        let op_str = "new = old + old";
        let (op, term) = parse_operation(op_str);
        assert_eq!(op, "+");
        assert_eq!(term, "old");
    }

    fn test_operation() {
        let operation = Operation {
            op: Op::Add,
            term: Some(3),
        };
        assert_eq!(operation.apply(4), 7);

        let operation = Operation {
            op: Op::Mult,
            term: Some(3),
        };
        assert_eq!(operation.apply(4), 12);

        let operation = Operation {
            op: Op::Mult,
            term: None,
        };
        assert_eq!(operation.apply(4), 16);
    }
}
