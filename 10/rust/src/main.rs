use std::vec;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Add(i32),
}

fn main() {
    let program: Vec<Instruction> = include_str!("../../input")
        .lines()
        .map(Instruction::parse)
        .collect();

    let register_states = run(program);
    println!("part 1: {}", part1(&register_states));
    part2(&register_states);
}

fn run(program: Vec<Instruction>) -> Vec<i32> {
    let mut register_value = 1;
    let mut register_states = vec![1];
    for instruction in program {
        match instruction {
            Instruction::Noop => register_states.push(register_value),
            Instruction::Add(i) => {
                register_states.extend([register_value, register_value]);
                register_value += i;
            }
        }
    }
    register_states.push(register_value);
    register_states
}

fn part1(register_states: &[i32]) -> i32 {
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&cycle| register_states[cycle] * cycle as i32)
        .sum()
}

fn part2(register_states: &[i32]) {
    let mut screen = [['.'; 40]; 6];
    for cycle in 1..=240 {
        let row = (cycle - 1) / 40;
        let pixel_position = (cycle - 1) % 40;
        if register_states[cycle].abs_diff(pixel_position as i32) <= 1 {
            screen[row][pixel_position] = '#';
        }
    }
    // render screen
    for row in screen {
        let row = String::from_iter(row);
        println!("{row}");
    }
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        match s {
            "noop" => Instruction::Noop,
            _ => {
                assert!(s.starts_with("addx"));
                let i = s[5..].parse::<i32>().expect("invalid instruction");
                Instruction::Add(i)
            }
        }
    }
}
