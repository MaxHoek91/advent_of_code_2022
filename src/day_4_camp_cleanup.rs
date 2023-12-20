use std::fs;

pub fn determine_assignment_pairs(file_path: &str) -> (u32, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut contained_pairs: u32 = 0;
    let mut any_overlap_count: u32 = 0;

    for line in data.lines() {
        let (pair_one, pair_two) = line.split_once(',').unwrap();
        let pair_one = parse_pair(pair_one);
        let pair_two = parse_pair(pair_two);

        if is_fully_contained(pair_one, pair_two) {
            contained_pairs += 1;
        }
        if any_overlap(pair_one, pair_two) {
            any_overlap_count += 1;
        }
    }

    (contained_pairs, any_overlap_count)
}


fn parse_pair(pair: &str) -> (u8, u8) {
    let (low, high) = pair.split_once('-').unwrap();
    (low.parse().unwrap(), high.parse().unwrap())
}

fn is_fully_contained<T>(pair_one: (T, T), pair_two: (T, T)) -> bool where T: Ord {
    (pair_one.0 <= pair_two.0 && pair_one.1 >= pair_two.1)
    || (pair_one.0 >= pair_two.0 && pair_one.1 <= pair_two.1)
}

fn any_overlap<T>(pair_one: (T, T), pair_two: (T, T)) -> bool where T: Ord {
    pair_one.0 <= pair_two.1  && pair_two.0 <= pair_one.1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("2-4"), (2, 4));
        assert_eq!(parse_pair("0-99"), (0, 99));
    }

    #[test]
    fn test_is_fully_contained() {
        assert!(is_fully_contained((2, 8), (3, 7)));
        assert!(is_fully_contained((6, 6), (4, 6)));
        assert!(!is_fully_contained((2, 4), (6, 8)));
    }

    #[test]
    fn test_any_overlap() {
        assert!(any_overlap((5, 7), (7, 9)));
        assert!(any_overlap((2, 8), (3, 7)));
        assert!(!any_overlap((2, 4), (6, 8)));
    }
}