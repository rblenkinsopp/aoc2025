use aoc2025::{get_input_as_string, parse_range};

#[inline(always)]
pub fn pow10(x: u64) -> u64 {
    const LUT: [u64; 12] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
    ];
    debug_assert!(x < LUT.len() as u64);
    LUT[x as usize]
}

#[inline(always)]
fn digits(x: u64) -> usize {
    debug_assert!(x > 0);
    (x.ilog10() + 1) as usize
}

const DIVISORS: [u64; 12] = [
    0,
    0,
    0,
    111,
    0,
    11111,
    10101,
    1111111,
    0,
    1001001,
    101010101,
    11111111111,
];

#[inline(always)]
fn sum_multiples(divisor: u64, l: u64, u: u64) -> u64 {
    let rem = l % divisor;
    let first = if rem == 0 { l } else { l + (divisor - rem) };
    if first > u {
        return 0;
    }

    let n = (u - first) / divisor + 1;
    n * (first + first + (n - 1) * divisor) / 2
}

#[inline(always)]
fn sum_invalid_range(start: u64, end: u64) -> (u64, u64) {
    let mut part_one: u64 = 0;
    let mut part_two: u64 = 0;

    for n_digits in digits(start)..=digits(end) {
        let n_digits = n_digits as u64;
        let lo = start.max(pow10(n_digits - 1));
        let hi = end.min(pow10(n_digits) - 1);

        if n_digits % 2 == 0 {
            let div = pow10(n_digits / 2) + 1;
            let invalid = sum_multiples(div, lo, hi);
            part_one += invalid;
            part_two += invalid;
        }

        if let div @ 1.. = DIVISORS[n_digits as usize] {
            part_two += sum_multiples(div, lo, hi);
        }

        // Have to handle these cases separately.
        if n_digits == 6 {
            part_two -= sum_multiples(111111, lo, hi);
        } else if n_digits == 10 {
            part_two -= sum_multiples(1111111111, lo, hi);
        }
    }

    (part_one, part_two)
}

#[inline(always)]
fn day2(input: &str) -> (u64, u64) {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(parse_range)
        .map(|(start, end)| sum_invalid_range(start as u64, end as u64))
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

#[inline(always)]
fn main() {
    let input = get_input_as_string();
    let (p1, p2) = day2(&input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        // Final answer tests from the puzzle description.
        const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        const SAMPLE_PART1_ANSWER: u64 = 1227775554;
        const SAMPLE_PART2_ANSWER: u64 = 4174379265;

        let (part1_answer, part2_answer) = day2(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER, "Part 1 is incorrect");
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER, "Part 2 is incorrect");
    }
}
