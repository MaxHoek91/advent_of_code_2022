use std::collections::HashSet;
use std::fs;

pub fn determine_sum_of_priorities(file_path: &str) -> (u32, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let sum_of_priorities_part_one: u32 = data
        .lines()
        .map(score_rucksack_priority_part_one)
        .sum();

    let sum_of_priorities_part_two: u32 = data
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .map(score_rucksack_priority_part_two)
        .sum();

    (sum_of_priorities_part_one, sum_of_priorities_part_two)
}

#[inline]
fn score_rucksack_priority_part_one(line: &str) -> u32 {
    let middle = line.len() / 2;
    let line = line.as_bytes();

    let compartment_1: HashSet<&u8> = HashSet::from_iter(line[0..middle].iter());
    let compartment_2: HashSet<&u8> = HashSet::from_iter(line[middle..].iter());

    let shared: u8 = **compartment_1
        .intersection(&compartment_2)
        .next()  // Assumption: there is only ever 1 shared item
        .unwrap();

    match shared.is_ascii_lowercase() {
        true => (shared - b'a' + 1) as u32,      // a..z = 1..26
        false => (shared - b'A' + 27) as u32     // A..Z = 27..52
    }
}


#[inline]
fn score_rucksack_priority_part_two(lines: &[&str]) -> u32 {
    let elf_1: HashSet<&u8> = HashSet::from_iter(lines[0].as_bytes());
    let elf_2: HashSet<&u8> = HashSet::from_iter(lines[1].as_bytes());
    let elf_3: HashSet<&u8> = HashSet::from_iter(lines[2].as_bytes());

    let shared: HashSet<&u8> = HashSet::from_iter(elf_1.intersection(&elf_2).copied());
    let shared = **shared
        .intersection(&elf_3)
        .next()  // Assumption: there is only ever 1 shared item
        .unwrap();

    match shared.is_ascii_lowercase() {
        true => (shared - b'a' + 1) as u32,      // a..z = 1..26
        false => (shared - b'A' + 27) as u32     // A..Z = 27..52
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_rucksack_priority_part_one() {
        assert_eq!(score_rucksack_priority_part_one("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(score_rucksack_priority_part_one("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(score_rucksack_priority_part_one("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(score_rucksack_priority_part_one("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(score_rucksack_priority_part_one("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(score_rucksack_priority_part_one("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
    }

    #[test]
    fn test_score_rucksack_priority_part_two() {
        let score = score_rucksack_priority_part_two(
            &[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]
        );
        assert_eq!(score, 18);

        let score = score_rucksack_priority_part_two(
            &[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]
        );
        assert_eq!(score, 52);
    }
}
