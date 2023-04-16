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

/// Track current value in register X, and the number of cycles ticked so far.
/// Also, store the "interesting" cycles and track the cumulative strength
/// observed at these cycles.
pub struct CPU {
    pub register_x: i32,
    pub cycle_count: i32,
    pub interesting_cycles: Vec<i32>,
    pub interesting_strength: i32,
}

impl CPU {
    /// Execute an instruction
    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            AddX(val) => {
                self.increment_cycles(2);
                self.register_x += val;
            }
            NoOp => self.increment_cycles(1),
        }
    }

    /// Tick a number of cycles, adding "interesting" strength when appropriate.
    fn increment_cycles(&mut self, n_cycles: u32) {
        for _ in 0..n_cycles {
            self.cycle_count += 1;

            // TODO: track next interesting cycle and do equality comparison
            if self.interesting_cycles.contains(&self.cycle_count) {
                self.interesting_strength += self.cycle_count * self.register_x;
            }
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
