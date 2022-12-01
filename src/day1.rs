use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day1 {
    calorie_counts: Vec<u32>,
}

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut counts = Vec::new();
        let mut curr: u32 = 0;
        for line in lines {
            if line.is_empty() {
                counts.push(curr);
                curr = 0;
            } else {
                curr += line.parse::<u32>().unwrap();
            }
        }
        counts.push(curr);
        Day1 {
            calorie_counts: counts,
        }
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        self.calorie_counts.iter().max().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let mut counts = self.calorie_counts.clone();
        counts.sort();
        counts.reverse();
        counts.iter().take(3).sum::<u32>().to_string()
    }
}
