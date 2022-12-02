use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day2 {
    step1_rounds: Vec<RPSRound>,
    step2_rounds: Vec<RPSRound>,
}

#[derive(Debug, Copy, Clone)]
struct RPSRound {
    elf_pick: RPSSelection,
    self_pick: RPSSelection,
}

#[derive(Debug, Copy, Clone)]
enum RPSSelection {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum RPSResult {
    Win,
    Loss,
    Draw,
}

impl RPSRound {
    pub fn result(&self) -> RPSResult {
        match (&self.self_pick, &self.elf_pick) {
            (RPSSelection::Rock, RPSSelection::Paper) => RPSResult::Loss,
            (RPSSelection::Rock, RPSSelection::Scissors) => RPSResult::Win,
            (RPSSelection::Paper, RPSSelection::Rock) => RPSResult::Win,
            (RPSSelection::Paper, RPSSelection::Scissors) => RPSResult::Loss,
            (RPSSelection::Scissors, RPSSelection::Paper) => RPSResult::Win,
            (RPSSelection::Scissors, RPSSelection::Rock) => RPSResult::Loss,
            _ => RPSResult::Draw,
        }
    }

    pub fn result_score(&self) -> u32 {
        match self.result() {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }

    pub fn full_score(&self) -> u32 {
        self.result_score() + self.self_pick.value()
    }
}

impl RPSSelection {
    pub fn value(&self) -> u32 {
        match self {
            RPSSelection::Rock => 1,
            RPSSelection::Paper => 2,
            RPSSelection::Scissors => 3,
        }
    }

    pub fn selection_for_result(&self, result: RPSResult) -> RPSSelection {
        match (self, &result) {
            (RPSSelection::Rock, RPSResult::Win) => RPSSelection::Paper,
            (RPSSelection::Rock, RPSResult::Loss) => RPSSelection::Scissors,
            (RPSSelection::Paper, RPSResult::Win) => RPSSelection::Scissors,
            (RPSSelection::Paper, RPSResult::Loss) => RPSSelection::Rock,
            (RPSSelection::Scissors, RPSResult::Win) => RPSSelection::Rock,
            (RPSSelection::Scissors, RPSResult::Loss) => RPSSelection::Paper,
            _ => self.clone(),
        }
    }

    pub fn from_char(ch: char) -> RPSSelection {
        match ch {
            'A' | 'X' => RPSSelection::Rock,
            'B' | 'Y' => RPSSelection::Paper,
            'C' | 'Z' => RPSSelection::Scissors,
            _ => panic!("Unexpected selection char: {}", ch),
        }
    }
}

impl RPSResult {
    pub fn from_char(ch: char) -> RPSResult {
        match ch {
            'X' => RPSResult::Loss,
            'Y' => RPSResult::Draw,
            'Z' => RPSResult::Win,
            _ => panic!("Unexpected result char: {}", ch),
        }
    }
}

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut step1_rounds = Vec::new();
        let mut step2_rounds = Vec::new();
        for line in lines {
            let elf_pick = RPSSelection::from_char(line.chars().nth(0).unwrap());
            step1_rounds.push(RPSRound {
                elf_pick,
                self_pick: RPSSelection::from_char(line.chars().nth(2).unwrap()),
            });
            let expected = RPSResult::from_char(line.chars().nth(2).unwrap());
            step2_rounds.push(RPSRound {
                elf_pick,
                self_pick: elf_pick.selection_for_result(expected),
            });
        }
        Day2 {
            step1_rounds,
            step2_rounds,
        }
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        let mut score = 0;
        for round in &self.step1_rounds {
            score += round.full_score()
        }
        score.to_string()
    }

    fn part_two(&self) -> String {
        let mut score = 0;
        for round in &self.step2_rounds {
            score += round.full_score()
        }
        score.to_string()
    }
}
