use memchr::{memchr, memchr_iter};
use aoc2025::{get_input_as_str, UniformInputIterator};

#[inline(always)]
fn day7(input: &str) -> (i64, i64) {
    let input = UniformInputIterator::from_bytes(input.as_bytes());
    let width = input.line_length();

    let mut part_one = 0;
    let mut classical_beams: Vec<bool> = vec![false; width];
    let mut quantum_beams: Vec<i64> = vec![0; width];

    for (y, line) in input.enumerate() {
        if y == 0 {
            let start = memchr(b'S', line).unwrap();
            classical_beams[start] = true;
            quantum_beams[start] = 1;
            continue;
        } else if y % 2 != 0 {
            // Ignore blank lines.
            continue;
        }

        let mut new_quantum_beams = quantum_beams.clone();
        for x in memchr_iter(b'^', line) {
            // Classical beams.
            if classical_beams[x] {
                part_one += 1;
                classical_beams[x] = false;
                classical_beams[x - 1] = true;
                classical_beams[x + 1] = true;
            }

            // Quantum beams.
            let quantum_beams_at_splitter = quantum_beams[x];
            if quantum_beams_at_splitter > 0 {
                new_quantum_beams[x] -= quantum_beams_at_splitter;
                new_quantum_beams[x - 1] += quantum_beams_at_splitter;
                new_quantum_beams[x + 1] += quantum_beams_at_splitter;
            }
        }
        quantum_beams = new_quantum_beams;
    }

    let part_two = quantum_beams.iter().sum::<i64>();

    (part_one, part_two)
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
        const SAMPLE_PART2_ANSWER: i64 = 40;

        let (part1_answer, part2_answer) = day7(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    #[test]
    fn test_day7_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day07.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day07.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day7(ACTUAL_INPUT), answers);
    }
}
