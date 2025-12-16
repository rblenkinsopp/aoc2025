use aoc2025::get_input_as_str;
use atoi::atoi;

#[inline(always)]
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (a_str, b_str) = line.split_once('-').unwrap();
            let mut a: u64 = atoi(a_str.as_bytes()).unwrap();
            let mut b: u64 = atoi(b_str.as_bytes()).unwrap();
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            (a, b)
        })
        .collect();

    let ingredients: Vec<u64> = ingredients_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| atoi(line.as_bytes()).unwrap())
        .collect();

    (fresh_ranges, ingredients)
}

#[inline(always)]
fn day5(input: &str) -> (usize, usize) {
    let (mut ranges, mut ingredient_ids) = parse_input(input);
    let num_ingredients = ingredient_ids.len();

    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ingredient_ids.sort_unstable();

    let mut part_one = 0usize;
    let mut part_two = 0usize;

    let mut it = ranges.into_iter();
    let mut current = it.next().unwrap();
    let mut i = 0usize;
    for (start, end) in it {
        if start <= current.1.saturating_add(1) {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            let (lo, hi) = current;
            part_two += (hi - lo + 1) as usize;

            while i < num_ingredients && ingredient_ids[i] < lo {
                i += 1;
            }

            while i < num_ingredients && ingredient_ids[i] <= hi {
                part_one += 1;
                i += 1;
            }

            current = (start, end);
        }
    }

    let (lo, hi) = current;
    part_two += (hi - lo + 1) as usize;

    while i < num_ingredients && ingredient_ids[i] < lo {
        i += 1;
    }

    while i < num_ingredients && ingredient_ids[i] <= hi {
        part_one += 1;
        i += 1;
    }

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day5(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day5() {
        const SAMPLE_INPUT: &str = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        const SAMPLE_PART1_ANSWER: usize = 3;
        const SAMPLE_PART2_ANSWER: usize = 14;

        let (part1_answer, part2_answer) = day5(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
