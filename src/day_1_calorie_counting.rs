use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

pub fn determine_max_calories(file_path: &str) -> (u32, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let calories_per_elf: Vec<u32> = data
        .split("\n\n")
        .map(
            | chunk | chunk
                .lines()
                .map(| line | line.parse::<u32>().unwrap())
                .sum::<u32>()
        ).collect();

    (
        *calories_per_elf.iter().max().unwrap(),
        sum_n_largest_elements(&calories_per_elf, 3)
    )
}


fn sum_n_largest_elements(values: &[u32], topn: usize) -> u32 {
    // Get the sum of the n-largest elements using a binary heap.
    let mut heap: BinaryHeap<_> = BinaryHeap::with_capacity(topn);
    let mut value_iter = values.iter();

    for _ in 0..topn {
        heap.push(Reverse(value_iter.next().unwrap_or(&0)));
    }

    for item in value_iter {
        if Reverse(item) < *heap.peek().unwrap() {
            heap.pop();
            heap.push(Reverse(item));
        }
    }

    heap.drain().map(| item| item.0).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    // const EXAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //
    // #[test]
    // fn test_calc_max_calories() {
    //     assert_eq!(get_max_calories(EXAMPLE), 24000);
    // }

    #[test]
    fn test_get_n_largest() {
        assert_eq!(sum_n_largest_elements(&[1, 2, 9, 4, 5, 6, 7, 8, 3, 10], 3), 27);
        assert_eq!(sum_n_largest_elements(&[1, 2], 4), 3);
    }
}