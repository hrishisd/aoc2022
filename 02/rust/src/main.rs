use Hand::{Paper, Rock, Scissor};

#[derive(PartialEq, Eq, Clone, Copy)]
struct Round {
    opponent: Hand,
    me: Hand,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}
enum Outcome {
    Win,
    Loss,
    Draw,
}
type LineToRound = fn(u8, u8) -> Round;

fn main() {
    let input = include_str!("../../input");
    let input = parse_input(input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn solve(input: &[(u8, u8)], line_to_round: LineToRound) -> i32 {
    input
        .iter()
        .map(|(a, b)| line_to_round(*a, *b))
        .map(|round| round.score())
        .sum()
}

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            (bytes[0], bytes[2])
        })
        .collect()
}

fn part1(input: &[(u8, u8)]) -> i32 {
    solve(input, |opponent, me| Round {
        opponent: Hand::from(opponent),
        me: Hand::from(me),
    })
}

fn part2(input: &[(u8, u8)]) -> i32 {
    solve(input, |opponent, outcome| {
        let opponent = Hand::from(opponent);
        let outcome = Outcome::from(outcome);
        Round {
            opponent,
            me: opponent.get_satisfying_hand(outcome),
        }
    })
}

impl Hand {
    fn get_satisfying_hand(&self, outcome: Outcome) -> Hand {
        match outcome {
            Outcome::Win => self.get_beaten_by(),
            Outcome::Loss => self.get_beats(),
            Outcome::Draw => *self,
        }
    }

    fn get_beaten_by(&self) -> Hand {
        match self {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        }
    }

    fn get_beats(&self) -> Hand {
        match self {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        }
    }
}

impl From<u8> for Hand {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissor,
            _ => panic!("Illegal character for hand"),
        }
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (self.opponent, self.me) {
            (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => Outcome::Win,
            (Scissor, Paper) | (Rock, Scissor) | (Paper, Rock) => Outcome::Loss,
            (Scissor, Scissor) | (Rock, Rock) | (Paper, Paper) => Outcome::Draw,
        }
    }

    fn score(&self) -> i32 {
        let outcome = self.outcome();
        let my_hand = self.me;
        (match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }) + (match my_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        })
    }
}

impl From<u8> for Outcome {
    fn from(c: u8) -> Self {
        match c {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            a => panic!("Illegal character for Outcome: {}", a),
        }
    }
}
