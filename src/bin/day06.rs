use aoc2025::get_input_as_str;

#[inline(always)]
fn day6(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    let (&ops_line, arg_lines) = lines.split_last().unwrap();

    // Part 1
    let args: Vec<Vec<i64>> = arg_lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let part_one = ops_line
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, s)| match s.as_bytes()[0] {
            b'+' => args.iter().map(|row| row[i]).sum::<i64>(),
            b'*' => args.iter().map(|row| row[i]).product::<i64>(),
            _ => panic!("Unsupported operator"),
        })
        .sum();

    // Part 2
    let mut part_two = 0;
    {
        let op_bytes = ops_line.as_bytes();
        let mut is_add = true;
        let mut acc = (0, 1);

        for i in 0..ops_line.len() {
            let mut x = 0;
            let mut number_column = false;

            for &row in arg_lines {
                if let Some(&b) = row.as_bytes().get(i)
                    && b.is_ascii_digit()
                {
                    x = x * 10 + (b - b'0') as i64;
                    number_column = true;
                }
            }

            if number_column {
                acc = (acc.0 + x, acc.1 * x);
                if op_bytes[i] == b'*' {
                    is_add = false;
                }
            } else {
                part_two += if is_add { acc.0 } else { acc.1 };
                acc = (0, 1);
                is_add = true;
            }
        }

        // Handle the final column.
        part_two += if is_add { acc.0 } else { acc.1 };
    }

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

    #[test]
    fn test_day6_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day06.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day06.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day6(ACTUAL_INPUT), answers);
    }
}
