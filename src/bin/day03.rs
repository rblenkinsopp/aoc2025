use aoc2025::{UniformInputIterator, get_input_as_str};
use argminmax::ArgMinMax;

#[inline(always)]
fn get_max_joltage(bytes: &[u8]) -> (i64, i64) {
    const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;

    // Part 1.
    let a_index = (&bytes[..bytes.len() - 1]).argmax();
    let b_index = a_index + 1 + (&bytes[a_index + 1..]).argmax();
    let part_one: i64 = unsafe {
        // Safety: We've just confirmed these indexes are valid from the search above.
        let a = *bytes.get_unchecked(a_index);
        let b = *bytes.get_unchecked(b_index);
        ((a - b'0') as i64) * 10 + ((b - b'0') as i64)
    };

    // Part 2.
    let mut start: usize = 0;
    let mut end = bytes.len() - MAX_BATTERIES_PER_BANK_PART_TWO;
    let mut part_two: i64 = 0;

    for _ in 0..MAX_BATTERIES_PER_BANK_PART_TWO {
        let offset = (&bytes[start..=end]).argmax();
        let index = start + offset;
        let digit = unsafe { *bytes.get_unchecked(index) };
        part_two = part_two * 10 + (digit - b'0') as i64;
        start = index + 1;
        end += 1;
    }

    (part_one, part_two)
}

#[inline(always)]
fn day3(input: &str) -> (i64, i64) {
    UniformInputIterator::from_bytes(input.as_bytes()).fold((0, 0), |(p1, p2), line| {
        let (v1, v2) = get_max_joltage(line);
        (p1 + v1, p2 + v2)
    })
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day3(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    #[rustfmt::skip]
    fn test_get_max_joltage() {
        assert_eq!(get_max_joltage("987654321111111".as_bytes()), (98, 987654321111));
        assert_eq!(get_max_joltage("811111111111119".as_bytes()), (89, 811111111119));
        assert_eq!(get_max_joltage("234234234234278".as_bytes()), (78, 434234234278));
        assert_eq!(get_max_joltage("818181911112111".as_bytes()), (92, 888911112111));
    }

    #[test]
    fn test_day3() {
        // Final answer tests from the puzzle description.
        const SAMPLE_INPUT: &str = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        const SAMPLE_PART1_ANSWER: i64 = 357;
        const SAMPLE_PART2_ANSWER: i64 = 3121910778619;

        let (part1_answer, part2_answer) = day3(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
