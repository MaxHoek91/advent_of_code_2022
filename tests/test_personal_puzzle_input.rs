use std::path::Path;

use advent_of_code_2022::*;

#[test]
fn test_day_1() {
    let file = Path::new("./data/day_1_calorie_counting.txt").to_str().unwrap();
    let calories = day_1_calorie_counting::determine_max_calories(file);

    assert_eq!(calories.0, 69177);
    assert_eq!(calories.1, 207456);
}

#[test]
fn test_day_2() {
    let file = Path::new("./data/day_2_rock_paper_scissors.txt").to_str().unwrap();
    let total_score = day_2_rock_paper_scissors::determine_total_score(file);

    assert_eq!(total_score.0, 8933);
    assert_eq!(total_score.1, 11998);
}

#[test]
fn test_day_3() {
    let file = Path::new("./data/day_3_rucksack_reorganization.txt").to_str().unwrap();
    let priorities = day_3_rucksack_reorganization::determine_sum_of_priorities(file);

    assert_eq!(priorities.0, 8515);
    assert_eq!(priorities.1, 2434);
}

#[test]
fn test_day_4() {
    let file = Path::new("./data/day_4_camp_cleanup.txt").to_str().unwrap();
    let (contained_pairs, overlap_count) = day_4_camp_cleanup::determine_assignment_pairs(file);

    assert_eq!(contained_pairs, 413);
    assert_eq!(overlap_count, 806);
}

#[test]
fn test_day_5() {
    let file = Path::new("./data/day_5_supply_stacks.txt").to_str().unwrap();
    let crates_on_top = day_5_supply_stacks::determine_top_crates(file);

    assert_eq!(crates_on_top.0, "FWNSHLDNZ");
    assert_eq!(crates_on_top.1, "RNRGDNFQG");
}