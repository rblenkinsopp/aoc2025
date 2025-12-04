use aoc2025::{Grid, get_input_as_str};

#[inline(always)]
fn remove_rolls(grid: &mut Grid) -> usize {
    let removable: Vec<usize> = grid
        .filter_iter(b'@')
        .filter(|p| p.adjacent_iter().filter(|n| n.value() == b'@').count() < 4)
        .map(|p| p.offset())
        .collect();

    let removed = removable.len();
    removable.into_iter().for_each(|i| grid.set_offset(i, b'.'));
    removed
}

#[inline(always)]
fn day4(input: &str) -> (i64, i64) {
    let mut grid = input.parse().unwrap();

    let (first, total) = std::iter::repeat_with(|| remove_rolls(&mut grid))
        .take_while(|&n| n != 0)
        .fold((None::<usize>, 0usize), |(first, total), n| {
            (first.or(Some(n)), total + n)
        });

    (first.unwrap_or(0) as i64, total as i64)
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

    #[test]
    fn test_remove_rools() {
        let mut grid = SAMPLE_INPUT.parse().unwrap();

        // Puzzle test cases.
        assert_eq!(remove_rolls(&mut grid), 13);
        assert_eq!(remove_rolls(&mut grid), 12);
        assert_eq!(remove_rolls(&mut grid), 7);
        assert_eq!(remove_rolls(&mut grid), 5);
        assert_eq!(remove_rolls(&mut grid), 2);
        assert_eq!(remove_rolls(&mut grid), 1);
        assert_eq!(remove_rolls(&mut grid), 1);
        assert_eq!(remove_rolls(&mut grid), 1);
        assert_eq!(remove_rolls(&mut grid), 1);
        assert_eq!(remove_rolls(&mut grid), 0);

        // Extra one to ensure "off the end" behaviour.
        assert_eq!(remove_rolls(&mut grid), 0);
    }

    #[test]
    fn test_day4() {
        const SAMPLE_PART1_ANSWER: i64 = 13;
        const SAMPLE_PART2_ANSWER: i64 = 43;

        let (part1_answer, part2_answer) = day4(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
