use std::collections::HashSet;
use memchr::{memchr, memchr_iter};
use aoc2025::{get_input_as_str, UniformInputIterator};

#[inline(always)]
fn day7(input: &str) -> (i64, i64) {
    let input = UniformInputIterator::from_bytes(input.as_bytes());

    // Part One.
    let mut part_one = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    for (y, line) in input.enumerate() {
        // Find the start position.
        if y == 0 {
           beams.insert(memchr(b'S', line).unwrap());
        } else if y % 2 == 0 {
            for x in memchr_iter(b'^', line) {
                if beams.contains(&x) {
                    beams.remove(&x);
                    part_one += 1;
                    beams.insert(x - 1);
                    beams.insert(x + 1);
                }
            }
        }
    }

    (part_one, 0)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day7(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day7() {
        const SAMPLE_INPUT: &str = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};
        const SAMPLE_PART1_ANSWER: i64 = 21;
        const SAMPLE_PART2_ANSWER: i64 = 0;

        let (part1_answer, part2_answer) = day7(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    // #[test]
    // fn test_day7_actual() {
    //     const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day07.txt");
    //     const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day07.txt");
    //     let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
    //     let answers = (
    //         str::parse::<i64>(answers.0).unwrap(),
    //         str::parse::<i64>(answers.1).unwrap(),
    //     );
    //     assert_eq!(day7(ACTUAL_INPUT), answers);
    // }
}
