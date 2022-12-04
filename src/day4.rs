use crate::{DaySolution, FromInput};

pub struct Day4 {
    pub pairs: Vec<Pair>,
}

pub struct Pair {
    pub assignment_a: (u32, u32),
    pub assignment_b: (u32, u32),
}

impl Pair {
    fn from_line(line: String) -> Pair {
        let mut split = line.split(",");
        let mut pair_a = split.nth(0).unwrap().split("-");
        let pair_a_start = pair_a.next().unwrap().parse::<u32>().unwrap();
        let pair_a_end = pair_a.next().unwrap().parse::<u32>().unwrap();
        let mut pair_b = split.next().unwrap().split("-");
        let pair_b_start = pair_b.next().unwrap().parse::<u32>().unwrap();
        let pair_b_end = pair_b.next().unwrap().parse::<u32>().unwrap();
        Pair {
            assignment_a: (pair_a_start, pair_a_end),
            assignment_b: (pair_b_start, pair_b_end),
        }
    }
}

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Day4 {
            pairs: lines.map(|l| Pair::from_line(l)).collect(),
        }
    }
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        let mut count = 0;
        for pair in &self.pairs {
            if (pair.assignment_a.0 <= pair.assignment_b.0
                && pair.assignment_a.1 >= pair.assignment_b.1)
                || (pair.assignment_b.0 <= pair.assignment_a.0
                    && pair.assignment_b.1 >= pair.assignment_a.1)
            {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        let mut count = 0;
        for pair in &self.pairs {
            if (pair.assignment_a.0 <= pair.assignment_b.0
                && pair.assignment_a.1 >= pair.assignment_b.0)
                || (pair.assignment_a.0 <= pair.assignment_b.1
                    && pair.assignment_a.1 >= pair.assignment_b.1)
                || (pair.assignment_b.0 <= pair.assignment_a.0
                    && pair.assignment_b.1 >= pair.assignment_a.0)
                || (pair.assignment_b.0 <= pair.assignment_a.1
                    && pair.assignment_b.1 >= pair.assignment_a.1)
            {
                count += 1;
            }
        }
        count.to_string()
    }
}
