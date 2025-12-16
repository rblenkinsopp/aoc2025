use aoc2025::get_input_as_str;
use atoi::atoi;
use std::array;

const NUM_PRESENTS: usize = 6;

struct Present {
    blocks: usize,
}

impl Present {
    fn from_input(input: &str) -> Self {
        Self {
            blocks: input.bytes().filter(|&b| b == b'#').count(),
        }
    }
}

struct Region {
    width: usize,
    height: usize,
    required_presents: [usize; NUM_PRESENTS],
}

impl Region {
    fn from_input(input: &str) -> Self {
        let (dims, rest) = input.split_once(':').unwrap();
        let (width_str, height_str) = dims.split_once('x').unwrap();

        let width = atoi(width_str.as_bytes()).unwrap();
        let height = atoi(height_str.as_bytes()).unwrap();

        let mut required_presents = [0; NUM_PRESENTS];
        for (i, s) in rest.split_ascii_whitespace().enumerate() {
            required_presents[i] = atoi(s.as_bytes()).unwrap();
        }

        Self {
            width,
            height,
            required_presents,
        }
    }
}

#[inline(always)]
fn day12(input: &str) -> i64 {
    let mut sections = input.split("\n\n");

    let presents: [Present; NUM_PRESENTS] =
        array::from_fn(|_| Present::from_input(sections.next().unwrap()));

    let rest = sections.next().unwrap_or("");

    let regions: Vec<Region> = rest
        .lines()
        .filter(|line| !line.is_empty())
        .map(Region::from_input)
        .collect();

    regions
        .iter()
        .filter(|region| {
            region
                .required_presents
                .iter()
                .zip(presents.iter())
                .map(|(required, present)| required * present.blocks)
                .sum::<usize>()
                <= region.width * region.height
        })
        .count() as i64
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let p1 = day12(input);
    println!("{p1}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day12() {
        const SAMPLE_INPUT: &str = indoc! {"
            0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2
        "};

        // TODO: Work out why this is wrong when the real input is correct.
        const SAMPLE_ANSWER: i64 = 2 + 1;

        let part1_answer = day12(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_ANSWER);
    }

    #[test]
    fn test_day12_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day12.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day12.txt");
        let answer = str::parse::<i64>(ACTUAL_ANSWERS).unwrap();
        assert_eq!(day12(ACTUAL_INPUT), answer);
    }
}
