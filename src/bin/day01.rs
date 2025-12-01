use aoc2025::get_input_as_string;
use atoi::atoi;

const DIAL_POSITION_START: i32 = 50;
const DIAL_POSITION_COUNT: i32 = 100;

#[inline(always)]
fn div_euclid_with_rem(x: i32) -> (i32, i32) {
    // This is more efficient than performing multiple `rem_euclid` and `div_euclid` calls.
    let quotient = x / DIAL_POSITION_COUNT;
    let remainder = x - quotient * DIAL_POSITION_COUNT;

    if remainder < 0 {
        (quotient - 1, remainder + DIAL_POSITION_COUNT)
    } else {
        (quotient, remainder)
    }
}

#[inline(always)]
fn rotate_dial(position: i32, rotation: i32) -> (i32, i32) {
    let (new_position_quotient, new_position_remainder) = div_euclid_with_rem(position + rotation);

    let times_past_zero = if rotation > 0 {
        new_position_quotient
    } else if rotation < 0 {
        (position > 0) as i32 - new_position_quotient - (new_position_remainder > 0) as i32
    } else {
        0
    };

    (new_position_remainder, times_past_zero)
}

#[inline(always)]
fn day1(input: &str) -> (i32, i32) {
    let initial_state = (DIAL_POSITION_START, 0, 0);
    let (_, part_one, part_two) = input
        .lines()
        .map(|line| {
            let direction = line.as_bytes().first().expect("Invalid direction");
            let magnitude: i32 = atoi(&line.as_bytes()[1..]).expect("Invalid magnitude");
            match direction {
                b'R' => magnitude,
                b'L' => -magnitude,
                _ => panic!("Invalid direction"),
            }
        })
        .fold(initial_state, |(position, part_one, part_two), rotation| {
            let (new_position, times_zero) = rotate_dial(position, rotation);
            (
                new_position,
                part_one + (new_position == 0) as i32,
                part_two + times_zero,
            )
        });

    (part_one, part_two)
}

fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day1(input.as_str());
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_rotate_dial() {
        // Sample input operations from the puzzle description.
        assert_eq!(rotate_dial(50, -68), (82, 1));
        assert_eq!(rotate_dial(82, -30), (52, 0));
        assert_eq!(rotate_dial(52, 48), (0, 1));
        assert_eq!(rotate_dial(0, -5), (95, 0));
        assert_eq!(rotate_dial(95, 60), (55, 1));
        assert_eq!(rotate_dial(55, -55), (0, 1));
        assert_eq!(rotate_dial(0, -1), (99, 0));
        assert_eq!(rotate_dial(99, -99), (0, 1));
        assert_eq!(rotate_dial(0, 14), (14, 0));
        assert_eq!(rotate_dial(14, -82), (32, 1));

        // Additional tests for edge cases.
        assert_eq!(rotate_dial(0, 100), (0, 1));
        assert_eq!(rotate_dial(99, 1), (0, 1));
        assert_eq!(rotate_dial(0, -1), (99, 0));
        assert_eq!(rotate_dial(50, 100), (50, 1));
        assert_eq!(rotate_dial(50, 1000), (50, 10));
        assert_eq!(rotate_dial(50, -100), (50, 1));
        assert_eq!(rotate_dial(50, -1000), (50, 10));
    }

    #[test]
    fn test_day1() {
        // Final answer tests from the puzzle description.
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
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
