extern crate core;

use aoc2025::get_input_as_str;
use std::cmp::PartialEq;

#[derive(Eq, PartialEq)]
enum Operator {
    ADD,
    MULTIPLY,
}

#[inline(always)]
fn day6(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<_>>();

    // Part 1.
    let blocks: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let (ops, args) = blocks.split_last().unwrap();
    let args: Vec<Vec<i64>> = args
        .iter()
        .map(|row| row.iter().map(|s| s.parse().unwrap()).collect())
        .collect();
    let ops: Vec<Operator> = ops
        .iter()
        .map(|op| match op.as_bytes()[0] {
            b'+' => Operator::ADD,
            b'*' => Operator::MULTIPLY,
            _ => panic!("Unsupported operator"),
        })
        .collect();
    let part_one = ops
        .iter()
        .enumerate()
        .map(|(i, op)| match op {
            Operator::ADD => args.iter().map(|row| row[i]).sum::<i64>(),
            Operator::MULTIPLY => args.iter().map(|row| row[i]).product::<i64>(),
        })
        .sum();

    // Part 2 (Horrible).
    let (ops_line, arg_lines) = lines.split_last().unwrap();
    let op_bytes = ops_line.as_bytes();
    let arg_bytes: Vec<&[u8]> = arg_lines.iter().map(|l| l.as_bytes()).collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let (total, group, mul) = (0..width)
        .map(|x| {
            let op_mul = matches!(op_bytes.get(x), Some(&b'*'));
            let n = arg_bytes
                .iter()
                .filter_map(|row| row.get(x))
                .filter(|&&b| b.is_ascii_digit())
                .fold(None, |acc, &b| {
                    Some(acc.unwrap_or(0) * 10 + (b - b'0') as i64)
                });

            (op_mul, n)
        })
        .fold(
            (0, Vec::<i64>::with_capacity(arg_bytes.len().max(1)), false),
            |(mut total, mut group, mut mul), (op_mul, n)| {
                mul |= op_mul;

                match n {
                    Some(v) => group.push(v),
                    None if !group.is_empty() => {
                        total += if mul {
                            group.iter().product::<i64>()
                        } else {
                            group.iter().sum()
                        };
                        group.clear();
                        mul = false;
                    }
                    _ => {}
                }

                (total, group, mul)
            },
        );

    let part_two = total
        + if group.is_empty() {
            0
        } else if mul {
            group.iter().product()
        } else {
            group.iter().sum()
        };

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day6(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day6() {
        const SAMPLE_INPUT: &str = indoc! {"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};
        const SAMPLE_PART1_ANSWER: i64 = 4277556;
        const SAMPLE_PART2_ANSWER: i64 = 3263827;

        let (part1_answer, part2_answer) = day6(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    // #[test]
    // fn test_day6_actual() {
    //     const ACTUAL_INPUT: &[u8] = include_bytes!("../../data/inputs/day01.txt");
    //     const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day01.txt");
    //     let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
    //     let answers = (
    //         str::parse::<i64>(answers.0).unwrap(),
    //         str::parse::<i64>(answers.1).unwrap(),
    //     );
    //     assert_eq!(day6(ACTUAL_INPUT), answers);
    // }
}
