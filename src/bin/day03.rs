use aoc2025::get_input_as_str;
use atoi::atoi;

const MAX_BATTERIES_PER_BANK_PART_ONE: usize = 2;
const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;

#[inline(always)]
fn get_max_joltage(bank: &str) -> (i64, i64) {
    let bytes = bank.as_bytes();
    let length = bytes.len();
    let mut stacks = [
        (Vec::with_capacity(length), length - MAX_BATTERIES_PER_BANK_PART_ONE),
        (Vec::with_capacity(length), length - MAX_BATTERIES_PER_BANK_PART_TWO),
    ];

    for &digit in bytes {
        for (stack, left_to_remove) in stacks.iter_mut() {
            while *left_to_remove > 0 {
                match stack.last() {
                    Some(&last) if last < digit => {
                        stack.pop();
                        *left_to_remove -= 1;
                    }
                    _ => break,
                }
            }
            stack.push(digit);
        }
    }

    stacks[0].0.truncate(MAX_BATTERIES_PER_BANK_PART_ONE);
    stacks[1].0.truncate(MAX_BATTERIES_PER_BANK_PART_TWO);
    (atoi(&stacks[0].0).unwrap(), atoi(&stacks[1].0).unwrap())
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
    let (p1, p2) = day3(get_input_as_str());
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
