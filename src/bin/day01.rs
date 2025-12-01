use aoc2025::get_input_as_string;

const DIAL_POSITION_START: i32 = 50;
const DIAL_POSITION_COUNT: i32 = 100;

#[inline(always)]
fn rotate_dial(position: i32, rotation: i32) -> (i32, i32) {
    let new_positon = position + rotation;
    let mut quotient = new_positon / DIAL_POSITION_COUNT;
    let mut remainder = new_positon - quotient * DIAL_POSITION_COUNT;

    if remainder < 0 {
        quotient -= 1;
        remainder += DIAL_POSITION_COUNT;
    }

    let times_past_zero = if rotation > 0 {
        quotient
    } else if rotation < 0 {
        (position > 0) as i32 - quotient - (remainder > 0) as i32
    } else {
        0
    };

    (remainder, times_past_zero)
}

#[inline(always)]
fn parse_rotation(bytes: &[u8]) -> i32 {
    let sign = ((bytes[0] == b'R') as i32) * 2 - 1;
    sign * match bytes.len() {
        2 => (bytes[1] - b'0') as i32,
        3 => ((bytes[1] - b'0') as i32) * 10 + (bytes[2] - b'0') as i32,
        4 => {
            ((bytes[1] - b'0') as i32) * 100
                + ((bytes[2] - b'0') as i32) * 10
                + (bytes[3] - b'0') as i32
        }
        _ => panic!("Rotation magnitude is > 3 digits"),
    }
}

#[inline(always)]
fn day1(input: &str) -> (i32, i32) {
    let mut position = DIAL_POSITION_START;
    let mut part_one = 0;
    let mut part_two = 0;

    for rotation in input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|b| !b.is_empty())
        .map(parse_rotation)
    {
        let (new_position, times_past_zero) = rotate_dial(position, rotation);
        part_one += (new_position == 0) as i32;
        part_two += times_past_zero;
        position = new_position;
    }

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day1(&input);
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
