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
fn day5(input: &str) -> (usize, usize) {
    let (fresh_ranges, ingredients) = split_input_parts(input);

    let (fresh_ranges, part_two) =
        merge_ranges(fresh_ranges.lines().map(parse_range).collect());

    let fresh = |x: i64| {
        let i = fresh_ranges.partition_point(|&(_, end)| end < x);
        i < fresh_ranges.len() && x >= fresh_ranges[i].0
    };

    let part_one = ingredients
        .lines()
        .map(parse_ingredient)
        .filter(|&x| fresh(x))
        .count();

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
