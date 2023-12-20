use std::collections::{HashMap, VecDeque};
use std::fs;

type Crates = Vec<VecDeque<char>>;


pub fn determine_top_crates(file_path: &str) -> (String, String) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let (crates, moves) = data.split_once("\n\n").unwrap();
    let mut crates_part_one: Crates = construct_crates(crates);
    let mut crates_part_two: Crates = crates_part_one.clone();

    for move_ in moves.lines() {
        let (mv, from, to) = parse_move(move_);
        perform_single_moves(&mut crates_part_one, mv, from, to);
        perform_multi_moves(&mut crates_part_two, mv, from, to);
    }

    (top_crates_to_string(crates_part_one), top_crates_to_string(crates_part_two))
}

fn construct_crates(data: &str) -> Crates {
    let mut crates_raw = data.lines().rev();

    let crate_positions: HashMap<usize, usize> = crates_raw
        .next()
        .unwrap()
        .match_indices(char::is_numeric)
        .map(| (idx, num) | (idx, num.parse::<usize>().unwrap() - 1))
        .collect();

    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new(); crate_positions.len()];

    for line in crates_raw {
        line
            .chars()
            .enumerate()
            .filter(| (_idx, char) | char.is_alphabetic())
            .for_each(| (idx, char) | crates[crate_positions[&idx]].push_front(char));

    }

    crates
}

#[inline]
fn parse_move(move_: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = move_.split_whitespace().collect();
    (
        parts[1].parse().unwrap(),
        parts[3].parse::<usize>().unwrap() - 1,
        parts[5].parse::<usize>().unwrap() - 1
    )
}

#[inline]
fn perform_single_moves(crates: &mut Crates, move_: usize, from: usize, to: usize) {
    for _ in 0..move_ {
        match crates[from].pop_front() {
            Some(crt) => crates[to].push_front(crt),
            None => break
        };
    }
}

#[inline]
fn perform_multi_moves(crates: &mut Crates, move_: usize, from: usize, to: usize) {
    let mut moved_crates: VecDeque<char> = VecDeque::new();

    for _ in 0..move_ {
        match crates[from].pop_front() {
            Some(crt) => moved_crates.push_front(crt),
            None => break
        };
    }

    for crt in moved_crates.into_iter() {
        crates[to].push_front(crt);
    }
}

fn top_crates_to_string(crates: Crates) -> String {
    String::from_iter(crates
        .iter()
        .map(| deque|
            *deque.front().unwrap_or(&' ')
        )
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_crates() {
        let test_data = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
        let expected = vec![
            VecDeque::from(['N', 'Z']),
            VecDeque::from(['D', 'C', 'M']),
            VecDeque::from(['P'])
        ];

        assert_eq!(construct_crates(test_data),expected);
    }

    #[test]
    fn test_parse_move() {
        // "from" and "to" are 0-indexed
        assert_eq!(parse_move("move 1 from 2 to 1"), (1, 1, 0));
        assert_eq!(parse_move("move 2 from 2 to 1"), (2, 1, 0));
        assert_eq!(parse_move("move 28 from 91 to 1011"), (28, 90, 1010));

    }

    #[test]
    fn test_perform_single_move() {
        let mut crates = vec![
            VecDeque::from(['N', 'Z']),
            VecDeque::from(['D', 'C', 'M']),
            VecDeque::from(['P'])
        ];

        let expected = vec![
            VecDeque::from(['D', 'N', 'Z']),
            VecDeque::from(['C', 'M']),
            VecDeque::from(['P'])
        ];
        perform_single_moves(&mut crates, 1, 1, 0);
        assert_eq!(crates, expected);

        let expected = vec![
            VecDeque::from([]),
            VecDeque::from(['C', 'M']),
            VecDeque::from(['Z', 'N', 'D', 'P'])
        ];
        perform_single_moves(&mut crates, 3, 0, 2);
        assert_eq!(crates, expected);

        let expected = vec![
            VecDeque::from(['M', 'C']),
            VecDeque::from([]),
            VecDeque::from(['Z', 'N', 'D', 'P'])
        ];
        perform_single_moves(&mut crates, 2, 1, 0);
        assert_eq!(crates, expected);

        let expected = vec![
            VecDeque::from(['C']),
            VecDeque::from(['M']),
            VecDeque::from(['Z', 'N', 'D', 'P'])
        ];
        perform_single_moves(&mut crates, 1, 0, 1);
        assert_eq!(crates, expected);
    }

    #[test]
    fn test_perform_multi_moves() {
        let mut crates = vec![
            VecDeque::from(['D', 'N', 'Z']),
            VecDeque::from(['C', 'M']),
            VecDeque::from(['P'])
        ];

        let expected = vec![
            VecDeque::from([]),
            VecDeque::from(['C', 'M']),
            VecDeque::from(['D', 'N', 'Z', 'P'])
        ];

        perform_multi_moves(&mut crates, 3, 0, 2);
        assert_eq!(crates, expected);
    }
}
