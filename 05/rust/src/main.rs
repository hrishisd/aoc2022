use scan_fmt::scan_fmt;

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}

fn main() {
    let commands = parse_commands(include_str!("../../input"));
    // sample input
    // let initial_state = Stacks {
    //     stacks: vec!["ZN".to_string(), "MCD".to_string(), "P".to_string()],
    // };
    let initial_state = Stacks {
        stacks: vec![
            "RNFVLJSM".to_string(),
            "PNDZFJWH".to_string(),
            "WRCDG".to_string(),
            "NBS".to_string(),
            "MZWPCBFN".to_string(),
            "PRMW".to_string(),
            "RTNGLSW".to_string(),
            "QTHFNBV".to_string(),
            "LMHZNF".to_string(),
        ],
    };
    println!("part 1: {}", part1(initial_state.clone(), &commands));
    println!("part 2: {}", part2(initial_state, &commands));
}

fn part1(mut stacks: Stacks, commands: &[Command]) -> String {
    for &command in commands {
        stacks.execute_sequential_move(command);
    }
    stacks.top_row()
}

fn part2(mut stacks: Stacks, commands: &[Command]) -> String {
    for &command in commands {
        stacks.execute_bulk_move(command);
    }
    stacks.top_row()
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let (quantity, from, to) =
                scan_fmt!(line, "move {} from {} to {}", usize, usize, usize)
                    .unwrap_or_else(|_| panic!("invalid command: {}", line));
            Command {
                quantity,
                // make command 0-indexed
                from: from - 1,
                to: to - 1,
            }
        })
        .collect()
}

impl Stacks {
    fn execute_sequential_move(&mut self, command: Command) {
        for _ in 0..command.quantity {
            if let Some(c) = self.stacks[command.from].pop() {
                self.stacks[command.to].push(c)
            }
        }
    }

    fn execute_bulk_move(&mut self, command: Command) {
        // stacks are 0-indexed internally
        let mut to_move = String::new();
        for _ in 0..command.quantity {
            if let Some(c) = self.stacks[command.from].pop() {
                to_move.push(c);
            }
        }
        for c in to_move.chars().rev() {
            self.stacks[command.to].push(c);
        }
    }

    fn top_row(&self) -> String {
        self.stacks
            .iter()
            .flat_map(|stack| stack.chars().last())
            .collect()
    }
}
