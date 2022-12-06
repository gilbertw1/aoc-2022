use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day5 {
    crates: Crates,
    commands: Vec<Command>,
}

#[derive(Clone)]
pub struct Crates {
    stacks: Vec<Vec<char>>,
}

pub struct Command {
    count: u32,
    source: usize,
    target: usize,
}

impl Crates {
    fn add_stack_line(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        self.add_crate_at_position(&chars, 0, 1);
        self.add_crate_at_position(&chars, 1, 5);
        self.add_crate_at_position(&chars, 2, 9);
        self.add_crate_at_position(&chars, 3, 13);
        self.add_crate_at_position(&chars, 4, 17);
        self.add_crate_at_position(&chars, 5, 21);
        self.add_crate_at_position(&chars, 6, 25);
        self.add_crate_at_position(&chars, 7, 29);
        self.add_crate_at_position(&chars, 8, 33);
    }

    fn add_crate_at_position(&mut self, line: &Vec<char>, idx: usize, pos: usize) {
        if let Some(ch) = line.get(pos) {
            if *ch != ' ' {
                self.stacks[idx].push(*ch);
            }
        }
    }

    fn run_command(&mut self, command: &Command) {
        for _ in 0..command.count {
            if let Some(scrate) = self.stacks[command.source - 1].pop() {
                self.stacks[command.target - 1].push(scrate);
            }
        }
    }

    fn run_command_multi(&mut self, command: &Command) {
        let mut to_add = Vec::new();
        for _ in 0..command.count {
            if let Some(ch) = self.stacks[command.source - 1].pop() {
                to_add.push(ch);
            }
        }
        to_add.reverse();
        self.stacks[command.target - 1].append(&mut to_add);
    }
}

impl Command {
    fn from_line(line: &str) -> Command {
        let split: Vec<&str> = line.split(" ").collect();
        Command {
            count: split[1].parse::<u32>().unwrap(),
            source: split[3].parse::<usize>().unwrap(),
            target: split[5].parse::<usize>().unwrap(),
        }
    }
}

impl FromInput for Day5 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut crates = Crates {
            stacks: vec![Vec::new(); 9],
        };
        let mut commands = Vec::new();
        let mut in_stack_lines = true;
        for line in lines {
            if in_stack_lines {
                if line.is_empty() {
                    in_stack_lines = false;
                } else if line.chars().nth(1).unwrap() != '1' {
                    crates.add_stack_line(&line);
                }
            } else {
                commands.push(Command::from_line(&line));
            }
        }

        for stack in &mut crates.stacks {
            stack.reverse();
        }

        Day5 { crates, commands }
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> String {
        let mut crates = self.crates.clone();
        for command in &self.commands {
            crates.run_command(command);
        }
        let mut result = "".to_string();
        for stack in crates.stacks {
            if let Some(ch) = stack.last() {
                result.push(*ch);
            }
        }
        result
    }

    fn part_two(&self) -> String {
        let mut crates = self.crates.clone();
        for command in &self.commands {
            crates.run_command_multi(command);
        }
        let mut result = "".to_string();
        for stack in crates.stacks {
            if let Some(ch) = stack.last() {
                result.push(*ch);
            }
        }
        result
    }
}
