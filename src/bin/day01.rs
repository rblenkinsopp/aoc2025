use aoc2025::get_input_as_string;

const DIAL_POSITION_START: i32 = 50;
const DIAL_POSITION_COUNT: i32 = 100;
const OFFSET: i32 = 1000000000;

#[inline(always)]
fn day1(input: &str) -> (i32, i32) {
    let bytes = input.as_bytes();
    let len = bytes.len();

    let mut abs_pos = OFFSET + DIAL_POSITION_START;
    let mut remainder = abs_pos % DIAL_POSITION_COUNT;
    let mut part_one = 0;
    let mut part_two = 0;
    let mut i = 0;

    while i < len {
        let is_right: i32;
        let mut steps: i32;
        unsafe {
            is_right = (*bytes.get_unchecked(i) == b'R') as i32;
            i += 1;
            steps = (*bytes.get_unchecked(i) - b'0') as i32;
            i += 1;
            let d = (*bytes.get_unchecked(i)).wrapping_sub(b'0');
            if d <= 9 {
                steps = steps * 10 + d as i32;
                i += 1;
            }
            let d = (*bytes.get_unchecked(i)).wrapping_sub(b'0');
            if d <= 9 {
                steps = steps * 10 + d as i32;
                i += 1;
            }
            i += 1;

            // Part 2.
            if is_right != 0 {
                part_two += (steps + remainder) / DIAL_POSITION_COUNT;
            } else {
                let to_next = (remainder != 0) as i32 * (DIAL_POSITION_COUNT - remainder);
                part_two += (steps + to_next) / DIAL_POSITION_COUNT;
            }

            // Part 1.
            let delta = (is_right * 2 - 1) * steps;
            abs_pos += delta;
            remainder = abs_pos % DIAL_POSITION_COUNT;
            part_one += (remainder == 0) as i32;
        }
    }

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let (p1, p2) = day1(get_input_as_string().as_str());
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day1() {
        const SAMPLE_INPUT: &str = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        const SAMPLE_PART1_ANSWER: i32 = 3;
        const SAMPLE_PART2_ANSWER: i32 = 6;

        let (part1_answer, part2_answer) = day1(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    #[test]
    fn test_day1_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day01.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day01.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i32>(answers.0).unwrap(),
            str::parse::<i32>(answers.1).unwrap(),
        );
        assert_eq!(day1(ACTUAL_INPUT), answers);
    }
}
