use aoc2025::get_input_as_str;
use memchr::{memchr, memchr_iter};

const SPACE: u8 = b'.';
const ROLL: u8 = b'@';
const MIN_NEIGHBOURS: u8 = 4;

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
    fn remove_accessible_rolls(&mut self) -> (usize, usize) {
        let stride = self.stride;
        let length = self.data.len();
        let mut degree = vec![0u8; length];
        let mut current: Vec<usize> = Vec::new();
        let mut next: Vec<usize> = Vec::new();

        let bytes: &[u8] = &self.data;
        current.reserve(bytes.len() / 64);

        unsafe {
            for i in 0..length {
                if *bytes.get_unchecked(i) != ROLL {
                    continue;
                }

                let up = i - stride;
                let down = i + stride;
                let d = (*bytes.get_unchecked(up - 1) == ROLL) as u8
                    + (*bytes.get_unchecked(up) == ROLL) as u8
                    + (*bytes.get_unchecked(up + 1) == ROLL) as u8
                    + (*bytes.get_unchecked(i - 1) == ROLL) as u8
                    + (*bytes.get_unchecked(i + 1) == ROLL) as u8
                    + (*bytes.get_unchecked(down - 1) == ROLL) as u8
                    + (*bytes.get_unchecked(down) == ROLL) as u8
                    + (*bytes.get_unchecked(down + 1) == ROLL) as u8;

                *degree.get_unchecked_mut(i) = d;
                if d < MIN_NEIGHBOURS {
                    current.push(i);
                }
            }
        }

        let bytes: &mut [u8] = &mut self.data;
        let mut first = 0;
        let mut total = 0;

        while !current.is_empty() {
            if first == 0 {
                first = current.len();
            }
            total += current.len();

            for i in current.drain(..) {
                unsafe {
                    *bytes.get_unchecked_mut(i) = SPACE;
                    let up = i - stride;
                    let down = i + stride;
                    let mut decrement_neighbour = |n: usize| {
                        if *bytes.get_unchecked(n) == ROLL {
                            let d = degree.get_unchecked_mut(n);
                            if *d == MIN_NEIGHBOURS {
                                next.push(n);
                            }
                            *d -= 1;
                        }
                    };

                    decrement_neighbour(up - 1);
                    decrement_neighbour(up);
                    decrement_neighbour(up + 1);
                    decrement_neighbour(i - 1);
                    decrement_neighbour(i + 1);
                    decrement_neighbour(down - 1);
                    decrement_neighbour(down);
                    decrement_neighbour(down + 1);
                }
            }

            std::mem::swap(&mut current, &mut next);
            next.clear();
        }

        (first, total)
    }
}

#[inline(always)]
fn day4(input: &str) -> (usize, usize) {
    let mut grid = PaperRollGrid::from_input(input);
    grid.remove_accessible_rolls()
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
}
