use aoc2025::{get_input_as_string, parse_range};
use rayon::prelude::*;

const PERIOD_CANDIDATES: [&[usize]; 12] = [
    &[],     // 0
    &[],     // 1
    &[],     // 2 (ignore half-period 1 as already checked)
    &[1],    // 3
    &[1],    // 4 (ignore half-period 2 as already checked)
    &[1],    // 5
    &[1, 2], // 6 (ignore half-period 3 as already checked)
    &[1],    // 7
    &[1, 2], // 8 (ignore half-period 4 as already checked)
    &[1, 3], // 9
    &[1, 2], // 10 (ignore half-period 5 as already checked)
    &[1],    // 11
];

#[inline(always)]
fn is_invalid_product_id_bytes(bytes: &[u8]) -> (bool, bool) {
    let length = bytes.len();
    let half = length / 2;

    // Part 1: Check if first half equals second half (only if even length)
    let invalid_part_one = bytes[..half] == bytes[half..];

    // Part 2: Invalid if it has repeating patterns.
    let invalid_part_two = invalid_part_one
        || PERIOD_CANDIDATES[length]
            .iter()
            .copied()
            .any(|p| (p..length).all(|i| bytes[i] == bytes[i % p]));

    (invalid_part_one, invalid_part_two)
}

#[inline(always)]
fn day2(input: &str) -> (i64, i64) {
    // Collect the ranges we need to check for valid ids.
    let ranges: Vec<_> = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(parse_range)
        .collect();

    // Distribute the work over all cores and reduce the result.
    ranges
        .par_iter()
        .fold(
            || (0, 0),
            |(p1, p2), &(start, end)| {
                let (mut part_one, mut part_two) = (p1, p2);
                let mut buffer = itoa::Buffer::new();

                for id in start..=end {
                    let bytes = buffer.format(id).as_bytes();
                    let (inv_p1, inv_p2) = is_invalid_product_id_bytes(bytes);
                    part_one += inv_p1 as i64 * id;
                    part_two += inv_p2 as i64 * id;
                }

                (part_one, part_two)
            },
        )
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

#[inline(always)]
fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day2(&input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_invalid_product_id(id: i64) -> (bool, bool) {
        let mut buffer = itoa::Buffer::new();
        let bytes = buffer.format(id).as_bytes();
        is_invalid_product_id_bytes(bytes)
    }

    fn get_invalid_product_ids_part_one(range: (i64, i64)) -> Vec<i64> {
        (range.0..=range.1)
            .filter(|&id| is_invalid_product_id(id).0)
            .collect()
    }

    fn get_invalid_product_ids_part_two(range: (i64, i64)) -> Vec<i64> {
        (range.0..=range.1)
            .filter(|&id| is_invalid_product_id(id).1)
            .collect()
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_invalid_product_ids() {
        // Part 1.
        assert_eq!(get_invalid_product_ids_part_one((11, 22)), vec![11, 22]);
        assert_eq!(get_invalid_product_ids_part_one((95, 115)), vec![99]);
        assert_eq!(get_invalid_product_ids_part_one((998, 1012)), vec![1010]);
        assert_eq!(get_invalid_product_ids_part_one((1188511880, 1188511890)), vec![1188511885]);
        assert_eq!(get_invalid_product_ids_part_one((222220, 222224)), vec![222222]);
        assert_eq!(get_invalid_product_ids_part_one((1698522, 1698528)), vec![]);
        assert_eq!(get_invalid_product_ids_part_one((446443, 446449)), vec![446446]);
        assert_eq!(get_invalid_product_ids_part_one((38593856, 38593862)), vec![38593859]);
        assert_eq!(get_invalid_product_ids_part_one((565653, 565659)), vec![]);
        assert_eq!(get_invalid_product_ids_part_one((824824821, 824824827)), vec![]);
        assert_eq!(get_invalid_product_ids_part_one((2121212118, 2121212124)), vec![]);

        // Part 2.
        assert_eq!(get_invalid_product_ids_part_two((11, 22)), vec![11, 22]);
        assert_eq!(get_invalid_product_ids_part_two((95, 115)), vec![99, 111]);
        assert_eq!(get_invalid_product_ids_part_two((998, 1012)), vec![999, 1010]);
        assert_eq!(get_invalid_product_ids_part_two((1188511880, 1188511890)), vec![1188511885]);
        assert_eq!(get_invalid_product_ids_part_two((222220, 222224)), vec![222222]);
        assert_eq!(get_invalid_product_ids_part_two((1698522, 1698528)), vec![]);
        assert_eq!(get_invalid_product_ids_part_two((446443, 446449)), vec![446446]);
        assert_eq!(get_invalid_product_ids_part_two((38593856, 38593862)), vec![38593859]);
        assert_eq!(get_invalid_product_ids_part_two((565653, 565659)), vec![565656]);
        assert_eq!(get_invalid_product_ids_part_two((824824821, 824824827)), vec![824824824]);
        assert_eq!(get_invalid_product_ids_part_two((2121212118, 2121212124)), vec![2121212121]);
    }

    #[test]
    fn test_day2() {
        // Final answer tests from the puzzle description.
        const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        const SAMPLE_PART1_ANSWER: i64 = 1227775554;
        const SAMPLE_PART2_ANSWER: i64 = 4174379265;

        let (part1_answer, part2_answer) = day2(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
