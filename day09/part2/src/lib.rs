use std::{collections::HashSet, error::Error};
use Direction::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(dir: char) -> Direction {
        match dir {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => panic!("Unknown direction: {}", dir),
        }
    }
}

/// Parse line of input (tab-delimited direction (U/D/R/L) and distance)
pub fn parse_line(
    line: Result<String, std::io::Error>,
) -> Result<(Direction, i32), Box<dyn Error>> {
    let line_str = line?;
    let mut data = line_str.split_whitespace().into_iter();
    let dir_char = data.next().unwrap().chars().next().unwrap();
    let direction = Direction::new(dir_char);
    let distance = data.next().unwrap().parse::<i32>()?;

    Ok((direction, distance))
}

/// Track the current positions of all knots, and the positions the tail has been to.
pub struct Rope {
    pub knots: [[i32; 2]; 10],
    pub tail_positions: HashSet<[i32; 2]>,
}

impl Rope {
    /// Pull the rope a given distance
    pub fn pull(&mut self, direction: Direction, dist: i32) {
        for _ in 0..dist {
            self.move_head(&direction);
            self.tail_positions.insert(self.knots[9]);
        }
    }

    /// Move the head of the rope one square in a given direction,
    /// and pull the trailing knots accordingly
    fn move_head(&mut self, direction: &Direction) {
        // Move head
        match direction {
            Right => self.knots[0][0] += 1,
            Left => self.knots[0][0] -= 1,
            Up => self.knots[0][1] += 1,
            Down => self.knots[0][1] -= 1,
        };

        // Move each knot if it no longer touches its predecessor.
        for i in 1..10 {
            if !self.is_touching(i) {
                self.move_knot(i);
            } else {
                break;
            }
        }
    }

    /// Check if a knot is still touching the preceding knot
    fn is_touching(&self, idx: usize) -> bool {
        (self.knots[idx][0] - self.knots[idx - 1][0]).abs() <= 1
            && (self.knots[idx][1] - self.knots[idx - 1][1]).abs() <= 1
    }

    /// Move a knot towards its predecessor.
    ///
    /// `prev` and `knot` are equivalent to `head` and `tail` in the original formulation.
    fn move_knot(&mut self, idx: usize) {
        let knot = self.knots[idx];
        let prev = self.knots[idx - 1];

        // Assuming the two knots are not touching, the knot must be exactly two
        // units away from its predecessor along one axis, and zero or one unit
        // away along the other.
        //
        let new_knot_pos = |prev_pos: i32, knot_pos: i32| -> i32 {
            let diff = knot_pos - prev_pos;
            match diff {
                0 | 1 | -1 => prev_pos,
                2 => prev_pos + 1,
                -2 => prev_pos - 1,
                _ => panic!("Invalid pair: {:?} {:?}", prev, knot),
            }
        };

        self.knots[idx][0] = new_knot_pos(prev[0], knot[0]);
        self.knots[idx][1] = new_knot_pos(prev[1], knot[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_is_touching() {
        let mut rope = Rope {
            knots: [[0, 0]; 10],
            tail_positions: HashSet::new(),
        };

        for pos in [[0, 0], [0, 1], [1, 1], [-1, 0], [-1, -1], [-1, 1]] {
            rope.knots[0] = pos;
            assert!(rope.is_touching(1));
        }

        for pos in [[0, 2], [1, 2], [-1, 2], [-2, 1]] {
            rope.knots[0] = pos;
            assert!(!rope.is_touching(1));
        }
    }

    #[test]
    fn test_rope_move() {
        let mut rope = Rope {
            knots: [[0, 0]; 10],
            tail_positions: HashSet::new(),
        };

        rope.pull(Right, 4);
        assert_eq!(rope.knots[0], [4, 0]);
        assert_eq!(rope.knots[1], [3, 0]);
        assert_eq!(rope.knots[2], [2, 0]);
        assert_eq!(rope.knots[3], [1, 0]);
        for i in 4..10 {
            assert_eq!(rope.knots[i], [0, 0]);
        }

        rope.pull(Up, 4);
        assert_eq!(rope.knots[0], [4, 4]);
        assert_eq!(rope.knots[1], [4, 3]);
        assert_eq!(rope.knots[2], [4, 2]);
        assert_eq!(rope.knots[3], [3, 2]);
        assert_eq!(rope.knots[4], [2, 2]);
        assert_eq!(rope.knots[5], [1, 1]);
        for i in 6..10 {
            assert_eq!(rope.knots[i], [0, 0]);
        }

        rope.pull(Left, 3);
        assert_eq!(rope.knots[0], [1, 4]);
        assert_eq!(rope.knots[1], [2, 4]);

        rope.pull(Down, 1);
        assert_eq!(rope.knots[0], [1, 3]);
        assert_eq!(rope.knots[1], [2, 4]);

        rope.pull(Right, 4);
        assert_eq!(rope.knots[0], [5, 3]);
        assert_eq!(rope.knots[1], [4, 3]);

        rope.pull(Down, 1);
        assert_eq!(rope.knots[0], [5, 2]);
        assert_eq!(rope.knots[1], [4, 3]);

        rope.pull(Left, 5);
        assert_eq!(rope.knots[0], [0, 2]);
        assert_eq!(rope.knots[1], [1, 2]);

        rope.pull(Right, 2);
        assert_eq!(rope.knots[0], [2, 2]);
        assert_eq!(rope.knots[1], [1, 2]);
    }
}
