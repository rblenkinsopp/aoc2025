use aoc2025::get_input_as_string;
use atoi::atoi;

#[inline(always)]
fn get_max_digit_in_slice(slice: &[u8]) -> (u8, usize) {
    let mut max_value = slice[0];
    let mut max_off = 0;

    if max_value != b'9' {
        for (off, &d) in slice[1..].iter().enumerate() {
            if d > max_value {
                max_value = d;
                max_off = off + 1;
                if d == b'9' {
                    break;
                }
            }
        }
    }

    (max_value, max_off + 1)
}

#[inline(always)]
fn get_max_joltage(bank: &str) -> (i64, i64) {
    const MAX_BATTERIES_PER_BANK_PART_TWO: usize = 12;

    let bytes = bank.as_bytes();

    // Part 1.
    let (a, offset) = get_max_digit_in_slice(&bytes[..bytes.len() - 1]);
    let (b, _) = get_max_digit_in_slice(&bytes[offset..]);
    let part_one = atoi(&[a, b]).unwrap();

    // Part 2.
    let mut slice = bytes;
    let mut buffer = [0; MAX_BATTERIES_PER_BANK_PART_TWO];
    for (i, digit) in buffer.iter_mut().enumerate() {
        let remaining = MAX_BATTERIES_PER_BANK_PART_TWO - i;
        let end = slice.len() - remaining;
        let (v, offset) = get_max_digit_in_slice(&slice[..=end]);
        *digit = v;
        slice = &slice[offset..];
    }
    let part_two = atoi(&buffer).unwrap();

    (part_one, part_two)
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
