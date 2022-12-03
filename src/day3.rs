use std::collections::{HashMap, HashSet};

use crate::{DaySolution, FromInput};

pub struct Day3 {
    pub sacks: Vec<RuckSack>,
    pub char_map: HashMap<char, u32>,
}

pub struct RuckSack {
    pub compartment_a: Vec<char>,
    pub compartment_b: Vec<char>,
}

impl RuckSack {
    fn from_string(contents: String) -> RuckSack {
        let split = contents.split_at(contents.len() / 2);
        RuckSack {
            compartment_a: split.0.chars().collect(),
            compartment_b: split.1.chars().collect(),
        }
    }

    fn find_intersect(&self) -> Option<char> {
        let setA = HashSet::<char>::from_iter(self.compartment_a.clone().iter().map(|ch| *ch));
        let setB = HashSet::<char>::from_iter(self.compartment_b.clone().iter().map(|ch| *ch));
        setA.intersection(&setB).map(|ch| ch.to_owned()).nth(0)
    }

    fn full_sack(&self) -> Vec<char> {
        [self.compartment_a.clone(), self.compartment_b.clone()].concat()
    }
}

impl Day3 {
    fn create_char_map() -> HashMap<char, u32> {
        let lower_vec = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
        let upper_vec = (b'A'..=b'Z').map(char::from).collect::<Vec<_>>();
        let char_vec = [lower_vec, upper_vec].concat();
        char_vec
            .iter()
            .enumerate()
            .map(|(idx, ch)| (*ch, (idx + 1) as u32))
            .collect::<HashMap<_, _>>()
    }

    fn create(sacks: Vec<RuckSack>) -> Day3 {
        Day3 {
            sacks,
            char_map: Self::create_char_map(),
        }
    }
}

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut sacks = Vec::new();
        for line in lines {
            sacks.push(RuckSack::from_string(line));
        }
        Day3::create(sacks)
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        for sack in &self.sacks {
            if let Some(ch) = sack.find_intersect() {
                sum += self.char_map.get(&ch).unwrap();
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;
        for group in self.sacks.chunks(3) {
            if let Some(ch) = find_intersect_group(group) {
                sum += self.char_map.get(&ch).unwrap();
            }
        }
        sum.to_string()
    }
}

fn find_intersect_group(sacks: &[RuckSack]) -> Option<char> {
    let setA = HashSet::<char>::from_iter(sacks[0].full_sack());
    let setB = HashSet::<char>::from_iter(sacks[1].full_sack());
    let setC = HashSet::<char>::from_iter(sacks[2].full_sack());
    let setAB = HashSet::<char>::from_iter(setA.intersection(&setB).map(|ch| *ch));
    setAB.intersection(&setC).nth(0).map(|ch| *ch)
}
