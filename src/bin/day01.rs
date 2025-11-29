use aoc2025::get_input_as_string;


const INITIAL_DIAL_POSITION: i32 = 50;

#[inline(always)]
fn rotate_dial(position: i32, rotation: i32) -> (i32, i32) {
    let new_position_raw = position + rotation;
    let new_position = new_position_raw.rem_euclid(100);

    let crossings = match rotation.signum() {
        1 => new_position_raw.div_euclid(100) - position.div_euclid(100),
        -1 => (position + 99).div_euclid(100) - (new_position_raw + 99).div_euclid(100),
        _ => 0,
    };

    (new_position, crossings)
}


#[inline(always)]
fn day1(input: &str) -> (i32, i32) {
    let mut position: i32 = INITIAL_DIAL_POSITION;
    let mut total_times_zero: i32 = 0;
    let mut total_times_zero_passed: i32 = 0;

    for line in input.lines() {
        let (dir_str, mag_str) = line.split_at(1);
        let direction = dir_str.as_bytes().first().expect("Invalid direction");
        let magnitude: i32 = mag_str.parse().expect("Invalid magnitude");

        let rotation = match direction {
            b'R' => magnitude,
            b'L' => -magnitude,
            _ => panic!("Invalid direction"),
        };

        let (new_position, times_zero) = rotate_dial(position, rotation);
        total_times_zero += (new_position == 0) as i32;
        total_times_zero_passed += times_zero;
        position = new_position;
    }

    (total_times_zero, total_times_zero_passed)
}

fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day1(input.as_str());
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

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

        // Additional tests for edge cases to ensure correct operation.
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
