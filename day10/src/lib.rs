use std::error::Error;
use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    NoOp,
    AddX(isize),
}

/// Parse line of input
pub fn parse_line(line: String) -> Result<Instruction, Box<dyn Error>> {
    let mut data = line.split_whitespace();
    let op_name = data.next();

    match op_name {
        Some("noop") => Ok(NoOp),
        Some("addx") => Ok(AddX(data.next().unwrap().parse::<isize>()?)),
        _ => Err("Invalid operation".into()),
    }
}

/// Track current value in register X, and the number of cycles ticked so far.
/// Also, store the "interesting" cycles and track the cumulative strength
/// observed at these cycles.
pub struct CPU {
    pub register_x: isize,
    pub cycle_count: usize,
    pub interesting_cycles: Vec<usize>,
    pub interesting_strength: isize,
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
    fn increment_cycles(&mut self, n_cycles: usize) {
        for _ in 0..n_cycles {
            self.cycle_count += 1;

            // TODO: track next interesting cycle and do equality comparison
            if self.interesting_cycles.contains(&self.cycle_count) {
                self.interesting_strength += (self.cycle_count as isize) * self.register_x;
            }
        }
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            register_x: 1,
            cycle_count: 0,
            interesting_cycles: (20..250).step_by(40).collect(), // manual is ugly but easy
            interesting_strength: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_parse_line() {
        let instr = parse_line("noop".to_string());
        assert!(matches!(instr, Ok(NoOp)));

        let instr = parse_line("addx 10".to_string());
        assert!(matches!(instr, Ok(AddX(10))));
    }
}
