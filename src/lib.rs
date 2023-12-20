use std::path::Path;
use std::time::Instant;

pub mod day_1_calorie_counting;
pub mod day_2_rock_paper_scissors;
pub mod day_3_rucksack_reorganization;
pub mod day_4_camp_cleanup;

pub fn solve_all() {
    println!("Advent of Code 2022\n");

    day_1();
    day_2();
    day_3();
    day_4();
}

fn day_1() {
    let file = Path::new("./data/day_1_calorie_counting.txt").to_str().unwrap();
    let timer = Instant::now();
    let calories = day_1_calorie_counting::determine_max_calories(file);

    println!(
        "Day 1: Calorie Counting\n\
        Run Time: {:?}\n\
        Maximum Calories Top1: {}\n\
        Maximum Calories Top3: {}\n",
        timer.elapsed(), calories.0, calories.1
    );
}

fn day_2() {
    let file = Path::new("./data/day_2_rock_paper_scissors.txt").to_str().unwrap();
    let timer = Instant::now();
    let total_score = day_2_rock_paper_scissors::determine_total_score(file);

    println!(
        "Day 2: Rock Paper Scissors\n\
        Run Time: {:?}\n\
        Total Score 1: {}\n\
        Total Score 2: {}\n",
        timer.elapsed(), total_score.0, total_score.1
    );
}

fn day_3() {
    let file = Path::new("./data/day_3_rucksack_reorganization.txt").to_str().unwrap();
    let timer = Instant::now();
    let priorities = day_3_rucksack_reorganization::determine_sum_of_priorities(file);

    println!(
        "Day 3: Rucksack Reorganization\n\
        Run Time: {:?}\n\
        Sum of Priorities 1: {}\n\
        Sum of Priorities 2: {}\n",
        timer.elapsed(), priorities.0, priorities.1
    );
}

fn day_4() {
    let file = Path::new("./data/day_4_camp_cleanup.txt").to_str().unwrap();
    let timer = Instant::now();
    let (contained_pairs, overlap_count) = day_4_camp_cleanup::determine_assignment_pairs(file);

    println!(
        "Day 4: Camp Cleanup\n\
        Run Time: {:?}\n\
        Fully Contained Pairs: {}\n\
        Any Overlap Count: {}\n",
        timer.elapsed(), contained_pairs, overlap_count
    );
}