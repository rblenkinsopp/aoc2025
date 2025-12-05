#![feature(slice_swap_unchecked)]

use aoc2025::{get_input_as_str, parse_range, split_input_parts};
use atoi::atoi;

#[inline(always)]
fn parse_ingredient(ingredient: &str) -> i64 {
    atoi(ingredient.as_bytes()).unwrap()
}

#[inline(always)]
fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> (Vec<(i64, i64)>, usize) {
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut write_index = 0;
    let mut total_length = 0;

    unsafe {
        for read_index in 1..ranges.len() {
            let (a_start, a_end) = *ranges.get_unchecked(write_index);
            let (b_start, b_end) = *ranges.get_unchecked(read_index);

            if b_start <= a_end.saturating_add(1) {
                let a_end = a_end.max(b_end);
                *ranges.get_unchecked_mut(write_index) = (a_start, a_end);
            } else {
                total_length += (a_end - a_start + 1) as usize;
                write_index += 1;
                if write_index != read_index {
                    ranges.swap_unchecked(write_index, read_index);
                }
            }
        }

        let (a_start, a_end) = *ranges.get_unchecked(write_index);
        total_length += (a_end - a_start + 1) as usize;
    }

    ranges.truncate(write_index + 1);
    (ranges, total_length)
}

#[inline(always)]
fn count_in_ranges_sorted(sorted_values: &[i64], ranges: &[(i64, i64)]) -> usize {
    let mut range_index = 0;
    let mut count = 0;
    let ranges_len = ranges.len();

    // Safety: Range values have already been checked.
    unsafe {
        for &x in sorted_values {
            while ranges.get_unchecked(range_index).1 < x {
                range_index += 1;
                if range_index == ranges_len {
                    return count;
                }
            }

            count += (x >= ranges.get_unchecked(range_index).0) as usize;
        }
    }
    count
}

#[inline(always)]
fn day5(input: &str) -> (usize, usize) {
    let (fresh_ranges, ingredients) = split_input_parts(input);

    // Part 2 and ranges for part 1.
    let (fresh_ranges, part_two) = merge_ranges(fresh_ranges.lines().map(parse_range).collect());

    // Part 1.
    let mut ingredient_ids: Vec<i64> = ingredients.lines().map(parse_ingredient).collect();
    ingredient_ids.sort_unstable();
    let part_one = count_in_ranges_sorted(&ingredient_ids, &fresh_ranges);

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day5(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_merge_ranges() {
        // Simple example.
        let test_ranges = vec![(10, 20), (0, 10), (30, 40)];
        assert_eq!(merge_ranges(test_ranges), (vec![(0, 20), (30, 40)], 32));

        // Actual sample test input.
        let test_ranges = vec![(3, 5), (10, 14), (16, 20)];
        assert_eq!(merge_ranges(test_ranges), (vec![(3, 5), (10, 20)], 14));
    }

    #[test]
    fn test_day5() {
        const SAMPLE_INPUT: &str = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        const SAMPLE_PART1_ANSWER: usize = 3;
        const SAMPLE_PART2_ANSWER: usize = 14;

        let (part1_answer, part2_answer) = day5(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
