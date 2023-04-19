use regex::Regex;
use std::error::Error;

const RELIEF_FACTOR: usize = 3;

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<usize>,
    pub operation: Operation,
    pub divisor: usize,
    pub true_dst: usize,
    pub false_dst: usize,
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

#[derive(Debug)]
pub struct Operation {
    operator: Operator,
    operand: Option<usize>,
}

impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        let operand = match self.operand {
            Some(val) => val,
            None => old,
        };

        match self.operator {
            Operator::Add => old + operand,
            Operator::Mult => old * operand,
        }
    }
}

pub fn parse_monkey(monkey_str: &str) -> Result<Monkey, Box<dyn Error>> {
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

    let operation = parse_operation(&cap[3])?;

    let parse_int = |x: &str| -> usize { x.parse::<usize>().unwrap() };

    Ok(Monkey {
        id: parse_int(&cap[1]),
        items,
        operation,
        divisor: parse_int(&cap[4]),
        true_dst: parse_int(&cap[5]),
        false_dst: parse_int(&cap[6]),
    })
}

// TODO: would like to directly parse into a closure
fn parse_operation(op_str: &str) -> Result<Operation, Box<dyn Error>> {
    let re = Regex::new(r"new = old (\+|\*) (.*)").unwrap();
    let cap = re.captures(op_str).unwrap();

    let operator: Result<Operator, Box<dyn Error>> = match &cap[1] {
        "+" => Ok(Operator::Add),
        "*" => Ok(Operator::Mult),
        _ => Err("Invalid operation".into()),
    };

    let operand = match &cap[2] {
        "old" => None,
        _ => cap[2].parse::<usize>().ok(),
    };

    Ok(Operation {
        operator: operator?,
        operand: operand,
    })
}

impl Monkey {
    // fn inspect_items(&mut self) -> impl Iterator<Item = (usize, usize)> {
    //     self.items.drain(..).map(|x| inspect_item(x))
    // }

    pub fn add(&mut self, item: usize) {
        self.items.push(item);
    }
}

pub fn inspect_item(monkey: &Monkey, item: usize) -> (usize, usize) {
    let worry_level = monkey.operation.apply(item) / RELIEF_FACTOR;

    let dst = if worry_level % monkey.divisor == 0 {
        monkey.true_dst
    } else {
        monkey.false_dst
    };

    (dst, worry_level)
}

#[cfg(test)]
mod tests {
    use std::matches;

    use super::*;

    fn make_test_monkey() -> Monkey {
        Monkey {
            id: 0,
            items: vec![79, 98],
            operation: Operation {
                operator: Operator::Mult,
                operand: Some(19),
            }, // |x| x * 19,
            divisor: 23,
            true_dst: 2,
            false_dst: 3,
        }
    }

    #[test]
    fn test_monkey_init() {
        let monkey = make_test_monkey();
        assert_eq!(monkey.operation.apply(monkey.items[0]), 79 * 19)
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

        let (dst, worry_level) = inspect_item(&monkey, monkey.items[0]);
        assert_eq!(worry_level, 500);
        assert_eq!(dst, 3);

        let (dst, worry_level) = inspect_item(&monkey, monkey.items[1]);
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

        let monkey = parse_monkey(monkey_str).unwrap();

        assert_eq!(monkey.items, vec![92, 73, 86, 83, 65, 51, 55, 93]);
        assert_eq!(monkey.divisor, 11);
        assert_eq!(monkey.true_dst, 3);
        assert_eq!(monkey.false_dst, 4);
    }

    #[test]
    fn test_parse_operation() {
        let op_str = "new = old * 5";
        let f = parse_operation(op_str).unwrap();
        assert!(matches!(f.operator, Operator::Mult));
        assert_eq!(f.operand, Some(5));
        assert_eq!(f.apply(3), 15);

        let op_str = "new = old + old";
        let f = parse_operation(op_str).unwrap();
        assert!(matches!(f.operator, Operator::Add));
        assert_eq!(f.operand, None);
        assert_eq!(f.apply(3), 6);
    }

    #[test]
    fn test_operation() {
        let operation = Operation {
            operator: Operator::Add,
            operand: Some(3),
        };
        assert_eq!(operation.apply(4), 7);

        let operation = Operation {
            operator: Operator::Mult,
            operand: Some(3),
        };
        assert_eq!(operation.apply(4), 12);

        let operation = Operation {
            operator: Operator::Mult,
            operand: None,
        };
        assert_eq!(operation.apply(4), 16);
    }
}
