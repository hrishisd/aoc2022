#![feature(unchecked_math)]
use std::collections::VecDeque;

type WorryLevel = i32;
type MonkeyIdx = usize;
struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    next_monkey_idx: Box<dyn Fn(WorryLevel) -> MonkeyIdx>,
}
fn main() {
    let input = include_str!("../../sample");
    input.split("\n\n").for_each(|m| println!("{m:?}"));
    println!("Hello, world!");
}

impl Monkey {
    fn parse(s: &str) -> Monkey {
        // scan_fmt!(s, "Monkey {}:\n  Starting items: ")
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use scan_fmt::scan_fmt;

    use crate::{MonkeyIdx, WorryLevel};

    #[test]
    fn test_parse_monkey() {
        let s = include_str!("../../sample").split("\n\n").next().unwrap();
        let lines: [&str; 6] = s
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let monkey_id = scan_fmt!(lines[0], "Monkey {}:", usize).unwrap();
        let starting_items = scan_fmt!(lines[1], "Starting items: {/.*/}", String).unwrap();
        let starting_items: VecDeque<WorryLevel> = starting_items
            .split(',')
            .map(|item| item.trim().parse::<i32>().unwrap())
            .collect();
        let (l, op, r) = scan_fmt!(
            lines[2],
            "Operation: new = {} {} {}",
            String,
            String,
            String
        )
        .unwrap();
        let op: fn(i32, i32) -> i32 = if &op == "*" {
            i32::wrapping_add
        } else {
            i32::wrapping_mul
        };
        let operation = match (l.as_str(), r.as_str()) {
            ("old", "old") => Box::new(|worry_level| op(worry_level, worry_level)),
            ("old", _) => Box::new(|worry_level| op(worry_level, r.parse::<i32>().unwrap())),
            _ => panic!("invalid input"),
        };
        let divisor = scan_fmt!(lines[3], "Test: divisible by {}", i32).unwrap();
        let true_monkey_idx = scan_fmt!(lines[4], "If true: throw to monkey {}", usize).unwrap();
        let false_monkey_idx = scan_fmt!(lines[4], "If false: throw to monkey {}", usize).unwrap();
        let next_monkey_idx = |worry_level| {
            if worry_level % divisor == 0 {
                true_monkey_idx
            } else {
                false_monkey_idx
            }
        };
        println!("{l}{op:?}{r}");
        println!("{monkey_id}, {starting_items:?}");
        println!("{lines:?}");
    }
}
