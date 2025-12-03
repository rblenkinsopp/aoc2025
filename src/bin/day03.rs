use aoc2025::get_input_as_string;
use atoi::atoi;

const MAX_BANK_LENGTH: usize = 100;

#[inline(always)]
fn get_max_joltage_part_one(bytes: &[u8]) -> i64 {
    let mut max_left = b'0';
    let mut max_right = b'0';
    let mut best_right = bytes[bytes.len() - 1];

    let mut i = bytes.len() - 1;
    while i > 0 {
        i -= 1;

        let a = bytes[i];
        let b = best_right;

        if a > max_left || (a == max_left && b > max_right) {
            max_left = a;
            max_right = b;
            if max_left == b'9' && max_right == b'9' {
                return 99;
            }
        }

        if a > best_right {
            best_right = a;
        }
    }

    atoi(&[max_left, max_right]).unwrap()
}

#[inline(always)]
fn get_max_joltage_part_two(bytes: &[u8]) -> i64 {
    const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;

    let mut stack = [0u8; MAX_BANK_LENGTH];
    let mut stack_length = 0usize;
    let mut left_to_remove = bytes.len() - MAX_BATTERIES_PER_BANK_PART_TWO;

    for &digit in bytes {
        while left_to_remove > 0 && stack_length > 0 && stack[stack_length - 1] < digit {
            stack_length -= 1;
            left_to_remove -= 1;
        }
        stack[stack_length] = digit;
        stack_length += 1;
    }

    atoi(&stack[..MAX_BATTERIES_PER_BANK_PART_TWO]).unwrap()
}

#[inline(always)]
fn get_max_joltage(bank: &str) -> (i64, i64) {
    let bytes = bank.as_bytes();
    (
        get_max_joltage_part_one(bytes),
        get_max_joltage_part_two(bytes),
    )
}

#[inline(always)]
fn day3(input: &str) -> (i64, i64) {
    input.lines().fold((0, 0), |(p1, p2), line| {
        let (v1, v2) = get_max_joltage(line);
        (p1 + v1, p2 + v2)
    })
}

#[inline(always)]
fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day3(&input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_get_max_joltage() {
        assert_eq!(get_max_joltage("987654321111111"), (98, 987654321111));
        assert_eq!(get_max_joltage("811111111111119"), (89, 811111111119));
        assert_eq!(get_max_joltage("234234234234278"), (78, 434234234278));
        assert_eq!(get_max_joltage("818181911112111"), (92, 888911112111));
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
