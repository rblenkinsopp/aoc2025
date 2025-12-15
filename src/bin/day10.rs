use aoc2025::get_input_as_str;
use good_lp::{Expression, Solution, SolverModel, constraint, microlp, variable, variables};
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<i64>,
}
use atoi::atoi;

impl Machine {
    fn from_input_line(line: &str) -> Machine {
        let (indicator_part, rest) = line.split_once(']').unwrap();
        let indicators_str = indicator_part.trim_start_matches('[');
        let indicator_lights = indicators_str.bytes().map(|b| b == b'#').collect();

        let (buttons_part, jolts_part) = rest.split_once('{').unwrap();
        let joltage_requirements = jolts_part
            .trim_end_matches('}')
            .split(',')
            .map(|s| atoi::<i64>(s.as_bytes()).unwrap())
            .collect();

        let buttons = buttons_part
            .split_whitespace()
            .map(|part| {
                part[1..part.len() - 1]
                    .split(',')
                    .map(|s| atoi::<usize>(s.as_bytes()).unwrap())
                    .collect()
            })
            .collect();

        Machine {
            indicator_lights,
            buttons,
            joltage_requirements,
        }
    }

    fn shortest_press_count_lights(&self) -> i64 {
        let num_lights = self.indicator_lights.len();

        let target_state = self
            .indicator_lights
            .iter()
            .enumerate()
            .fold(0u64, |bits, (i, &on)| bits | ((on as u64) << i));

        let button_masks: Vec<u64> = self
            .buttons
            .iter()
            .filter_map(|button| {
                let mask = button.iter().fold(0u64, |m, &idx| {
                    debug_assert!(idx < num_lights);
                    m | (1u64 << idx)
                });
                (mask != 0).then_some(mask)
            })
            .collect();

        let num_states = 1usize << num_lights;
        let mut min_presses = vec![u32::MAX; num_states];
        let mut queue: VecDeque<u64> = VecDeque::from([0]);

        min_presses[0] = 0;
        while let Some(state) = queue.pop_front() {
            let presses = min_presses[state as usize];

            if state == target_state {
                return presses as i64;
            }

            for &button_mask in &button_masks {
                let next_state = state ^ button_mask;
                let idx = next_state as usize;
                if min_presses[idx] == u32::MAX {
                    min_presses[idx] = presses + 1;
                    queue.push_back(next_state);
                }
            }
        }

        unreachable!("Solving not possible");
    }

    fn shortest_press_count_joltage(&self) -> i64 {
        // Solve this as a linear programming problem, using good_lp.
        let mut vars = variables!();
        let x: Vec<_> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = vars.minimise(x.iter().sum::<Expression>()).using(microlp);

        for (i, &target) in self.joltage_requirements.iter().enumerate() {
            let expr: Expression = self
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(j, button)| button.contains(&i).then_some(x[j]))
                .sum();
            problem = problem.with(constraint!(expr == target as f64));
        }

        let solution = problem.solve().unwrap();
        x.iter().map(|&v| solution.value(v).round() as i64).sum()
    }
}

#[inline(always)]
fn day10(input: &str) -> (i64, i64) {
    let machines: Vec<Machine> = input.lines().map(Machine::from_input_line).collect();

    let part_one = machines
        .iter()
        .map(Machine::shortest_press_count_lights)
        .sum::<i64>();

    let part_two = machines
        .iter()
        .map(Machine::shortest_press_count_joltage)
        .sum::<i64>();

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day10(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    #[rustfmt::skip]
    fn test_machine_from_input_line() {
        assert_eq!(
            Machine::from_input_line(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            ),
            Machine {
                indicator_lights: vec![false, true, true, false],
                buttons: vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]],
                joltage_requirements: vec![3, 5, 4, 7],
            }
        );
        assert_eq!(
            Machine::from_input_line(
                "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            ),
            Machine {
                indicator_lights: vec![false, false, false, true, false],
                buttons: vec![vec![0, 2, 3, 4], vec![2, 3], vec![0, 4], vec![0, 1, 2], vec![1, 2, 3, 4]],
                joltage_requirements: vec![7, 5, 12, 7, 2],
            }
        );
        assert_eq!(
            Machine::from_input_line(
                "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ),
            Machine {
                indicator_lights: vec![false, true, true, true, false, true],
                buttons: vec![vec![0, 1, 2, 3, 4], vec![0, 3, 4], vec![0, 1, 2, 4, 5], vec![1, 2]],
                joltage_requirements: vec![10, 11, 11, 5, 10, 5],
            }
        );
    }

    #[test]
    fn test_shortest_press_count_() {}

    #[test]
    fn test_day10() {
        const SAMPLE_INPUT: &str = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        const SAMPLE_PART1_ANSWER: i64 = 7;
        const SAMPLE_PART2_ANSWER: i64 = 33;

        let (part1_answer, part2_answer) = day10(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    #[test]
    fn test_day10_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day10.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day10.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day10(ACTUAL_INPUT), answers);
    }
}
