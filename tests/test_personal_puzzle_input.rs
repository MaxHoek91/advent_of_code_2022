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