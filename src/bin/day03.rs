use aoc2025::get_input_as_string;
use atoi::atoi;

const MAX_BATTERIES_PER_BANK_PART_ONE: usize = 2;
const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;
const MAX_BANK_LENGTH: usize = 100;

#[inline(always)]
fn get_max_joltage(bank: &str) -> (i64, i64) {
    let bytes = bank.as_bytes();
    let length = bytes.len();

    // Part 1 state.
    let mut stack1 = [0u8; MAX_BANK_LENGTH];
    let mut length1 = 0usize;
    let mut left_to_remove1 = length - MAX_BATTERIES_PER_BANK_PART_ONE;
    let mut done1 = false;

    // Part 2 state.
    let mut stack2 = [0u8; MAX_BANK_LENGTH];
    let mut length2 = 0usize;
    let mut left_to_remove2 = length - MAX_BATTERIES_PER_BANK_PART_TWO;

    for &digit in bytes {
        // Part 1 (with early exit when 99 has been found).
        if !done1 {
            let mut new_len1 = length1;
            let mut remove1 = left_to_remove1;
            while remove1 > 0 && new_len1 > 0 && stack1[new_len1 - 1] < digit {
                new_len1 -= 1;
                remove1 -= 1;
            }
            length1 = new_len1;
            left_to_remove1 = remove1;
            stack1[length1] = digit;
            length1 += 1;

            if length1 == MAX_BATTERIES_PER_BANK_PART_ONE && stack1[..2] == [b'9', b'9'] {
                done1 = true;
            }
        }

        // Part 2 (no early exit as seems non-existent, at least in my input data)
        let mut new_len2 = length2;
        let mut remove2 = left_to_remove2;
        while remove2 > 0 && new_len2 > 0 && stack2[new_len2 - 1] < digit {
            new_len2 -= 1;
            remove2 -= 1;
        }
        length2 = new_len2;
        left_to_remove2 = remove2;
        stack2[length2] = digit;
        length2 += 1;
    }

    (
        atoi(&stack1[..MAX_BATTERIES_PER_BANK_PART_ONE]).unwrap(),
        atoi(&stack2[..MAX_BATTERIES_PER_BANK_PART_TWO]).unwrap(),
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
