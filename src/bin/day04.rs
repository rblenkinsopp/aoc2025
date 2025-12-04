use aoc2025::get_input_as_str;
use memchr::{memchr, memchr_iter};

const SPACE: u8 = b'.';
const PAPER_ROLL: u8 = b'@';

struct PaperRollGrid {
    data: Vec<u8>,
    stride: usize,
}

impl PaperRollGrid {
    fn from_input(input: &str) -> Self {
        let bytes = input.as_bytes();
        let width = memchr(b'\n', bytes).unwrap_or(bytes.len());
        let input_stride = width + 1;
        let height = (bytes.len() + 1) / input_stride;
        let stride = width + 2;

        let mut data = vec![SPACE; stride * (height + 2)];

        // Safety: These ranges have been checked against the input above and must be valid.
        unsafe {
            for r in 0..height {
                let src = r * input_stride;
                let src = bytes.get_unchecked(src..src + width);
                let dst = (r + 1) * stride + 1;
                let dst = data.get_unchecked_mut(dst..dst + width);
                dst.copy_from_slice(src);
            }
        }

        Self { data, stride }
    }

    #[inline(always)]
    fn remove_accessible_rolls(&mut self) -> usize {
        let stride = self.stride;
        let bytes: &[u8] = &self.data;
        let mut to_remove = Vec::with_capacity(bytes.len() / 64);

        for i in memchr_iter(PAPER_ROLL, bytes) {
            unsafe {
                let mut c = 0u8;
                c += (*bytes.get_unchecked(i - stride - 1) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i - stride) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i - stride + 1) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i - 1) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i + 1) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i + stride - 1) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i + stride) == PAPER_ROLL) as u8;
                c += (*bytes.get_unchecked(i + stride + 1) == PAPER_ROLL) as u8;
                if c < 4 {
                    to_remove.push(i);
                }
            }
        }

        for &i in &to_remove {
            unsafe { *self.data.get_unchecked_mut(i) = SPACE; }
        }

        to_remove.len()
    }
}

#[inline(always)]
fn day4(input: &str) -> (usize, usize) {
    let mut grid = PaperRollGrid::from_input(input);

    let part_one = grid.remove_accessible_rolls();
    let part_two = part_one
        + std::iter::repeat_with(|| grid.remove_accessible_rolls())
            .take_while(|&n| n != 0)
            .sum::<usize>();

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
        let mut grid = PaperRollGrid::from_input(SAMPLE_INPUT);

        // Puzzle test cases.
        assert_eq!(grid.remove_accessible_rolls(), 13);
        assert_eq!(grid.remove_accessible_rolls(), 12);
        assert_eq!(grid.remove_accessible_rolls(), 7);
        assert_eq!(grid.remove_accessible_rolls(), 5);
        assert_eq!(grid.remove_accessible_rolls(), 2);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 1);
        assert_eq!(grid.remove_accessible_rolls(), 0);

        // Extra one to ensure "off the end" behaviour.
        assert_eq!(grid.remove_accessible_rolls(), 0);
    }

    #[test]
    fn test_day4() {
        const SAMPLE_PART1_ANSWER: usize = 13;
        const SAMPLE_PART2_ANSWER: usize = 43;

        let (part1_answer, part2_answer) = day4(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
