use scan_fmt::scan_fmt;
use std::{collections::HashSet, fmt::Debug};

/// The state of a rope with K knots
#[derive(Debug)]
struct Rope<const K: usize> {
    knots: [Position; K],
}

#[derive(Clone, Copy)]
struct Move {
    magnitude: i32,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let moves: Vec<Move> = include_str!("../../input")
        .lines()
        .map(parse_move)
        .collect();

    println!("part 1: {}", simulate::<2>(&moves));
    println!("part 2: {}", simulate::<10>(&moves));
}

/// Simulate the movement of a rope of size K and return the number of unique positions of the tail
fn simulate<const K: usize>(moves: &[Move]) -> usize {
    let mut rope = Rope::<K>::initial();
    let mut tail_positions = HashSet::new();
    for m in moves {
        for _ in 0..(m.magnitude) {
            rope.update(m.direction);
            tail_positions.insert(rope.tail());
        }
    }
    tail_positions.len()
}

impl<const K: usize> Rope<K> {
    fn initial() -> Self {
        Rope {
            knots: [Position { x: 0, y: 0 }; K],
        }
    }

    /// Move the head knot and update the positions of the subsequent knots
    fn update(&mut self, direction: Direction) {
        let knots = &mut self.knots;
        knots[0] = knots[0].move_in(direction);
        for i in 1..K {
            knots[i].follow(knots[i - 1]);
            assert!(knots[i].is_touching(knots[i - 1]));
        }
    }

    fn tail(&self) -> Position {
        self.knots[K - 1]
    }
}

impl Position {
    fn follow(&mut self, leader: Position) {
        if !self.is_touching(leader) {
            self.x = Position::step(self.x, leader.x);
            self.y = Position::step(self.y, leader.y);
        }
    }

    fn is_touching(self, other: Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_in(self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn step(source: i32, target: i32) -> i32 {
        source + (target - source).clamp(-1, 1)
    }
}

fn parse_move(line: &str) -> Move {
    let (direction, magnitude) = scan_fmt!(line, "{} {}", char, i32).unwrap();
    let direction = match direction {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("invalid direction"),
    };
    Move {
        magnitude,
        direction,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Position, Rope};

    #[test]
    fn test_follow() {
        let leader = Position { x: 0, y: 2 };
        let mut follower = Position { x: 0, y: 0 };
        follower.follow(leader);
        assert_eq!(Position { x: 0, y: 1 }, follower);
    }

    #[test]
    fn test_rope_update() {
        let mut rope = Rope::<2>::initial();
        rope.update(Direction::Up);
        assert_eq!(Position { x: 0, y: 0 }, rope.tail());
        rope.update(Direction::Up);
        println!("{rope:?}");
        assert_eq!(Position { x: 0, y: 1 }, rope.tail());
    }
}
