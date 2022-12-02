use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_move(move_str: &str) -> Option<Self> {
        match move_str {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            "X" => Some(Shape::Rock),
            "Y" => Some(Shape::Paper),
            "Z" => Some(Shape::Scissors),
            &_ => None,
        }
    }

    /// Score for the move
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round {
    opponent: Shape,
    response: Shape,
}

impl Round {
    /// TODO: clean this up, can we implement comparison on Shape?
    fn outcome(&self) -> Outcome {
        match self.response {
            Shape::Rock => match self.opponent {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match self.opponent {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match self.opponent {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> u32 {
        self.response.score() + self.outcome().score()
    }
}

fn parse_rounds<R: BufRead>(reader: &mut R) -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();

    for line in reader.lines() {
        let moves = line.expect("Failed to read");

        // parse line into Shapes
        let moves: Vec<&str> = moves.split(" ").collect();

        let opponent = match Shape::from_move(moves[0]) {
            Some(x) => x,
            None => continue,
        };

        let response = match Shape::from_move(moves[1]) {
            Some(x) => x,
            None => continue,
        };

        let round = Round { opponent, response };

        rounds.push(round);
    }

    rounds
}

fn main() {
    let mut reader = io::stdin().lock(); // StdinLock implements BufRead
    let rounds = parse_rounds(&mut reader);

    let mut score = 0;
    for round in rounds {
        score += round.score();
    }

    println!("Total score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throw_paper() {
        let round = Round {
            response: Shape::Paper,
            opponent: Shape::Rock,
        };
        assert_eq!(round.outcome(), Outcome::Win);
        assert_eq!(round.score(), 2 + 6);

        let round = Round {
            response: Shape::Paper,
            opponent: Shape::Paper,
        };
        assert_eq!(round.outcome(), Outcome::Draw);
        assert_eq!(round.score(), 2 + 3);

        let round = Round {
            response: Shape::Paper,
            opponent: Shape::Scissors,
        };
        assert_eq!(round.outcome(), Outcome::Loss);
        assert_eq!(round.score(), 2 + 0);
    }

    #[test]
    fn test_throw_rock() {
        let round = Round {
            response: Shape::Rock,
            opponent: Shape::Rock,
        };
        assert_eq!(round.outcome(), Outcome::Draw);
        assert_eq!(round.score(), 1 + 3);

        let round = Round {
            response: Shape::Rock,
            opponent: Shape::Paper,
        };
        assert_eq!(round.outcome(), Outcome::Loss);
        assert_eq!(round.score(), 1 + 0);

        let round = Round {
            response: Shape::Rock,
            opponent: Shape::Scissors,
        };
        assert_eq!(round.outcome(), Outcome::Win);
        assert_eq!(round.score(), 1 + 6);
    }

    #[test]
    fn test_throw_scissors() {
        let round = Round {
            response: Shape::Scissors,
            opponent: Shape::Rock,
        };
        assert_eq!(round.outcome(), Outcome::Loss);
        assert_eq!(round.score(), 3 + 0);

        let round = Round {
            response: Shape::Scissors,
            opponent: Shape::Paper,
        };
        assert_eq!(round.outcome(), Outcome::Win);
        assert_eq!(round.score(), 3 + 6);

        let round = Round {
            response: Shape::Scissors,
            opponent: Shape::Scissors,
        };
        assert_eq!(round.outcome(), Outcome::Draw);
        assert_eq!(round.score(), 3 + 3);
    }
}
