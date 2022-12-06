use std::collections::{HashMap, HashSet, VecDeque};

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day6 {
    data_stream: Vec<char>,
}

impl FromInput for Day6 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let line_vec: Vec<String> = lines.collect();
        Day6 {
            data_stream: line_vec[0].chars().collect(),
        }
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        let mut queue = VecDeque::new();
        for (i, ch) in self.data_stream.iter().enumerate() {
            queue.push_back(ch);
            if queue.len() == 4 && is_unique(&queue) {
                return (i + 1).to_string();
            } else if queue.len() == 4 {
                queue.pop_front();
            }
        }
        return "-1".to_string();
    }

    fn part_two(&self) -> String {
        let mut queue = VecDeque::new();
        for (i, ch) in self.data_stream.iter().enumerate() {
            queue.push_back(ch);
            if queue.len() == 14 && is_unique(&queue) {
                return (i + 1).to_string();
            } else if queue.len() == 14 {
                queue.pop_front();
            }
        }
        return "-1".to_string();
    }
}

fn is_unique(queue: &VecDeque<&char>) -> bool {
    let mut hash = HashSet::new();
    for ch in queue.iter() {
        if hash.contains(ch) {
            return false;
        }
        hash.insert(ch);
    }
    return true;
}
