use aoc2025::get_input_as_str;
use std::collections::HashMap;

struct PathCounter<'a> {
    devices: &'a HashMap<&'a str, Vec<&'a str>>,
    target_cache: HashMap<&'a str, HashMap<&'a str, i64>>,
}

impl<'a> PathCounter<'a> {
    fn new(devices: &'a HashMap<&'a str, Vec<&'a str>>) -> Self {
        Self {
            devices,
            target_cache: HashMap::new(),
        }
    }

    fn count(&mut self, from: &'a str, to: &'a str) -> i64 {
        fn dfs<'a>(
            devices: &'a HashMap<&'a str, Vec<&'a str>>,
            node: &'a str,
            target: &'a str,
            cache: &mut HashMap<&'a str, i64>,
        ) -> i64 {
            if node == target {
                return 1;
            }

            if let Some(&cached) = cache.get(node) {
                return cached;
            }

            let total = devices
                .get(node)
                .map(|outputs| {
                    outputs
                        .iter()
                        .map(|&next| dfs(devices, next, target, cache))
                        .sum()
                })
                .unwrap_or(0);

            cache.insert(node, total);
            total
        }

        let memo = self.target_cache.entry(to).or_default();

        dfs(self.devices, from, to, memo)
    }
}

#[inline(always)]
fn day11(input: &str) -> (i64, i64) {
    let devices: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(':').unwrap();
            let outputs = rest.split_ascii_whitespace().collect();
            (name, outputs)
        })
        .collect();

    let mut counter = PathCounter::new(&devices);

    // Part 1: Simple single path.
    let part_one = counter.count("you", "out");

    // Part 2: Trace via DAC then FFT, and FFT then DAC separately.
    let svr_dac_fft_out =
        counter.count("svr", "dac") * counter.count("dac", "fft") * counter.count("fft", "out");
    let svr_fft_dac_out =
        counter.count("svr", "fft") * counter.count("fft", "dac") * counter.count("dac", "out");
    let part_two = svr_dac_fft_out + svr_fft_dac_out;

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day11(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day11() {
        const SAMPLE_PART1_INPUT: &str = indoc! {"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
        "};
        const SAMPLE_PART2_INPUT: &str = indoc! {"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
        "};
        const SAMPLE_PART1_ANSWER: i64 = 5;
        const SAMPLE_PART2_ANSWER: i64 = 2;

        let (part1_answer, _) = day11(SAMPLE_PART1_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);

        let (_, part2_answer) = day11(SAMPLE_PART2_INPUT);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    #[test]
    fn test_day11_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day11.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day11.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day11(ACTUAL_INPUT), answers);
    }
}
