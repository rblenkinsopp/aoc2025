use aoc2025::{UniformInputIterator, get_input_as_str};

const SPACE: u8 = b'.';
const ROLL: u8 = b'@';
const MIN_NEIGHBOURS: u8 = 4;

fn parse_grid(input: &str) -> (Vec<u8>, usize, usize, usize) {
    let mut rows = UniformInputIterator::from_bytes(input.as_bytes());
    let width = rows.line_length();
    let height = width;
    let stride = width + 2;

    // Copy each logical into the padded Vec, leaving a one-cell "." border.
    let mut grid = vec![SPACE; stride * stride];
    for (row, line) in (&mut rows).enumerate() {
        let start = (row + 1) * stride + 1;
        let end = start + width;
        grid[start..end].copy_from_slice(line);
    }

    (grid, width, height, stride)
}

#[inline(always)]
#[rustfmt::skip]
fn count_neighbouring_rolls(grid: &[u8], stride: usize, index: usize) -> u8 {
    let up = index - stride;
    let down = index + stride;

    let mut rolls = 0;
    if grid[up - 1] == ROLL { rolls += 1; }
    if grid[up]     == ROLL { rolls += 1; }
    if grid[up + 1] == ROLL { rolls += 1; }
    if grid[index - 1] == ROLL { rolls += 1; }
    if grid[index + 1] == ROLL { rolls += 1; }
    if grid[down - 1] == ROLL { rolls += 1; }
    if grid[down]     == ROLL { rolls += 1; }
    if grid[down + 1] == ROLL { rolls += 1; }
    rolls
}

#[inline(always)]
fn day4(input: &str) -> (usize, usize) {
    let (mut grid, width, height, stride) = parse_grid(input);

    let mut degree = vec![0; grid.len()];
    let mut current: Vec<usize> = Vec::new();
    let mut next: Vec<usize> = Vec::new();

    // Initial degree computation and frontier setup.
    for row in 1..=height {
        let base = row * stride + 1;
        for col in 0..width {
            let idx = base + col;
            if grid[idx] != ROLL {
                continue;
            }

            let d = count_neighbouring_rolls(&grid, stride, idx);
            degree[idx] = d;
            if d < MIN_NEIGHBOURS {
                current.push(idx);
            }
        }
    }

    let part_one = current.len();
    let mut part_two = 0;

    while !current.is_empty() {
        part_two += current.len();

        for i in current.drain(..) {
            grid[i] = SPACE;
            let mut decrement_neighbour = |n: usize| {
                if grid[n] == ROLL {
                    let d = &mut degree[n];
                    if *d == MIN_NEIGHBOURS {
                        next.push(n);
                    }
                    *d -= 1;
                }
            };

            let up = i - stride;
            let down = i + stride;
            decrement_neighbour(up - 1);
            decrement_neighbour(up);
            decrement_neighbour(up + 1);
            decrement_neighbour(i - 1);
            decrement_neighbour(i + 1);
            decrement_neighbour(down - 1);
            decrement_neighbour(down);
            decrement_neighbour(down + 1);
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day4(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day4() {
        const SAMPLE_INPUT: &str = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        const SAMPLE_PART1_ANSWER: usize = 13;
        const SAMPLE_PART2_ANSWER: usize = 43;

        let (part1_answer, part2_answer) = day4(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }

    #[test]
    fn test_day4_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day04.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day04.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<usize>(answers.0).unwrap(),
            str::parse::<usize>(answers.1).unwrap(),
        );
        assert_eq!(day4(ACTUAL_INPUT), answers);
    }
}
