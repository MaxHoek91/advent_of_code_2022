use std::fs;

pub fn determine_total_score(file_path: &str) -> (u32, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let total_score_part_one = data
        .lines()
        .map(rock_paper_scissors_score_part_one)
        .sum();

    let total_score_part_two = data
        .lines()
        .map(rock_paper_scissors_score_part_two)
        .sum();

    (total_score_part_one, total_score_part_two)
}


fn rock_paper_scissors_score_part_one(line: &str) -> u32 {
    // A, X = Rock (1)
    // B, Y = Paper (2)
    // C, Z = Scissors (3)
    //
    // Loss = 0
    // Draw = 3
    // Win = 6
    match line.split_once(' ').unwrap() {
        ("A", "X") => 4,  // 3 + 1
        ("A", "Y") => 8,  // 6 + 2
        ("A", "Z") => 3,  // 0 + 3
        ("B", "X") => 1,  // 0 + 1
        ("B", "Y") => 5,  // 3 + 2
        ("B", "Z") => 9,  // 6 + 3
        ("C", "X") => 7,  // 6 + 1
        ("C", "Y") => 2,  // 0 + 2
        ("C", "Z") => 6,  // 3 + 3
        _ => unreachable!()
    }
}

fn rock_paper_scissors_score_part_two(line: &str) -> u32 {
    // A = Rock (1)
    // B = Paper (2)
    // C = Scissors (3)
    //
    // X = Loss = 0
    // Y = Draw = 3
    // Z = Win = 6
    match line.split_once(' ').unwrap() {
        ("A", "X") => 3,  // Scissors   0 + 3
        ("A", "Y") => 4,  // Rock       3 + 1
        ("A", "Z") => 8,  // Paper      6 + 2
        ("B", "X") => 1,  // Rock       0 + 1
        ("B", "Y") => 5,  // Paper      3 + 2
        ("B", "Z") => 9,  // Scissors   6 + 3
        ("C", "X") => 2,  // Paper      0 + 2
        ("C", "Y") => 6,  // Scissors   3 + 3
        ("C", "Z") => 7,  // Rock       6 + 1
        _ => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors_score_part_one() {
        assert_eq!(rock_paper_scissors_score_part_one("A Y"), 8);
        assert_eq!(rock_paper_scissors_score_part_one("B X"), 1);
        assert_eq!(rock_paper_scissors_score_part_one("C Z"), 6);
    }

    #[test]
    fn test_rock_paper_scissors_score_part_two() {
        assert_eq!(rock_paper_scissors_score_part_two("A Y"), 4);
        assert_eq!(rock_paper_scissors_score_part_two("B X"), 1);
        assert_eq!(rock_paper_scissors_score_part_two("C Z"), 7);
    }
}