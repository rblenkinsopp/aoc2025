use aoc2025::get_input_as_string;
use atoi::atoi;

const MAX_BATTERIES_PER_BANK_PART_ONE: usize = 2;
const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;

#[inline(always)]
fn get_max_joltage(bank: &str, max_batteries: usize) -> i64 {
    let bytes = bank.as_bytes();
    let mut to_remove = bytes.len() - max_batteries;
    let mut stack: Vec<u8> = Vec::with_capacity(bytes.len());

    for &d in bytes {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(max_batteries);
    atoi(&stack).unwrap()
}

#[inline(always)]
fn day3(input: &str) -> (i64, i64) {
    input.lines().fold((0, 0), |(p1, p2), line| {
        (
            p1 + get_max_joltage(line, MAX_BATTERIES_PER_BANK_PART_ONE),
            p2 + get_max_joltage(line, MAX_BATTERIES_PER_BANK_PART_TWO),
        )
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
        // Part 1.
        assert_eq!(
            get_max_joltage("987654321111111", MAX_BATTERIES_PER_BANK_PART_ONE),
            98
        );
        assert_eq!(
            get_max_joltage("811111111111119", MAX_BATTERIES_PER_BANK_PART_ONE),
            89
        );
        assert_eq!(
            get_max_joltage("234234234234278", MAX_BATTERIES_PER_BANK_PART_ONE),
            78
        );
        assert_eq!(
            get_max_joltage("818181911112111", MAX_BATTERIES_PER_BANK_PART_ONE),
            92
        );

        // Part 2.
        assert_eq!(
            get_max_joltage("987654321111111", MAX_BATTERIES_PER_BANK_PART_TWO),
            987654321111
        );
        assert_eq!(
            get_max_joltage("811111111111119", MAX_BATTERIES_PER_BANK_PART_TWO),
            811111111119
        );
        assert_eq!(
            get_max_joltage("234234234234278", MAX_BATTERIES_PER_BANK_PART_TWO),
            434234234278
        );
        assert_eq!(
            get_max_joltage("818181911112111", MAX_BATTERIES_PER_BANK_PART_TWO),
            888911112111
        );
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
