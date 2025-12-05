#![feature(slice_swap_unchecked)]

use aoc2025::get_input_as_str;
use atoi::atoi;
use memchr::{memchr, memmem::find};

#[inline(always)]
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> (Vec<(u64, u64)>, usize) {
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
fn count_in_ranges(values: &[u64], ranges: &[(u64, u64)]) -> usize {
    let ranges_len = ranges.len();
    let mut count = 0;

    unsafe {
        for &x in values {
            let mut lo = 0usize;
            let mut hi = ranges_len;

            while lo < hi {
                let mid = (lo + hi) >> 1;
                let (_, end) = *ranges.get_unchecked(mid);
                if end < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            if lo != ranges_len {
                let (start, _) = *ranges.get_unchecked(lo);
                count += (x >= start) as usize;
            }
        }
    }

    count
}


#[inline(always)]
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::with_capacity(200);
    let mut ingredients: Vec<u64> = Vec::with_capacity(1000);
    let bytes = input.as_bytes();
    let split = find(bytes, b"\n\n").unwrap();

    unsafe {
        let ranges_bytes = bytes.get_unchecked(..split);
        let ingredients_bytes = bytes.get_unchecked(split + 2..);

        let mut i = 0;
        while i < ranges_bytes.len() {
            let rest = ranges_bytes.get_unchecked(i..);
            let n = memchr(b'\n', rest).unwrap_or(rest.len());
            let line = rest.get_unchecked(..n);
            let dash = memchr(b'-', line).unwrap();
            let a: u64 = atoi(&line[..dash]).unwrap();
            let b: u64 = atoi(&line[dash + 1..]).unwrap();
            fresh_ranges.push(if a <= b { (a, b) } else { (b, a) });
            i += n + 1;
        }

        let mut i = 0;
        while i < ingredients_bytes.len() {
            let rest = ingredients_bytes.get_unchecked(i..);
            let n = memchr(b'\n', rest).unwrap_or(rest.len());
            let line = rest.get_unchecked(..n);
            ingredients.push(atoi(line).unwrap());

            i += n + 1;
        }
    }

    (fresh_ranges, ingredients)
}

#[inline(always)]
fn day5(input: &str) -> (usize, usize) {
    let (fresh_ranges, mut ingredient_ids) = parse_input(input);
    let (fresh_ranges, part_two) = merge_ranges(fresh_ranges);
    let part_one = count_in_ranges(&ingredient_ids, &fresh_ranges);
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
