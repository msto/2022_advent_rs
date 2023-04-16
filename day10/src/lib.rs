use std::error::Error;
use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    NoOp,
    AddX(i32),
}

/// Parse line of input
pub fn parse_line(line: Result<String, std::io::Error>) -> Result<Instruction, Box<dyn Error>> {
    let line_str = line?;
    let mut data = line_str.split_whitespace().into_iter();
    let op_name = data.next();

    match op_name {
        Some("noop") => Ok(NoOp),
        Some("addx") => Ok(AddX(data.next().unwrap().parse::<i32>()?)),
        _ => Err("Invalid operation".into()),
    }
}

pub struct CPU {
    pub register_x: i32,
    pub n_cycles: u32,
}

impl CPU {
    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            AddX(val) => {
                self.n_cycles += 2;
                self.register_x += val;
            }
            NoOp => self.n_cycles += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_parse_line() {
        let instr = parse_line(Ok("noop".to_string()));
        assert!(matches!(instr, Ok(NoOp)));

        let instr = parse_line(Ok("addx 10".to_string()));
        assert!(matches!(instr, Ok(AddX(10))));
    }
}
