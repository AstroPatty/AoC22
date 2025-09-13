use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Sack {
    left_items: HashSet<char>,
    right_items: HashSet<char>,
}

impl Sack {
    fn get_common(&self) -> HashSet<char> {
        let common = self.left_items.intersection(&self.right_items);
        return common.copied().collect();
    }
    fn get_priority_score(&self) -> usize {
        let common = self.get_common();
        let result = common.iter().map(|c| to_priority(c).unwrap()).sum();
        return result;
    }
    fn get_all_entries(&self) -> HashSet<char> {
        let result: HashSet<char> = self.left_items.union(&self.right_items).copied().collect();
        return result;
    }
}

fn to_priority(c: &char) -> Option<usize> {
    if !c.is_alphabetic() {
        return None;
    }

    if c.is_uppercase() {
        return Some((*c as usize) - 38);
    } else {
        return Some((*c as usize) - 96);
    }
}

impl TryFrom<&str> for Sack {
    type Error = String;
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let cs = data.chars();
        let string_length = cs.count();
        if string_length % 2 != 0 {
            return Err(String::from("String does not have even length!"));
        }
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        let split = string_length / 2;
        for (i, c) in data.chars().enumerate() {
            if i < split {
                left.insert(c);
            } else {
                right.insert(c);
            }
        }
        return Ok(Sack {
            left_items: left,
            right_items: right,
        });
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let sacks: Vec<Sack> = data
        .lines()
        .filter_map(|s| Sack::try_from(s).ok())
        .collect();

    let result = sacks
        .chunks(3)
        .map(|chunk| {
            let s1: HashSet<_> = chunk[0]
                .get_all_entries()
                .intersection(&chunk[1].get_all_entries())
                .copied()
                .collect();
            let res: HashSet<_> = s1
                .intersection(&chunk[2].get_all_entries())
                .copied()
                .collect();
            res
        })
        .map(|common| {
            common
                .iter()
                .map(|c| to_priority(c).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{:?}", result);
}
