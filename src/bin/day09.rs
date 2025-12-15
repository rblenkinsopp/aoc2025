use aoc2025::get_input_as_str;
use atoi::atoi;
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    fn from_csv_str(s: &str) -> Point2D {
        let mut parts = s.split(',');
        let x = atoi(parts.next().unwrap().as_bytes()).unwrap();
        let y = atoi(parts.next().unwrap().as_bytes()).unwrap();
        Self { x, y }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Rect2D {
    pub min_x: i64,
    pub min_y: i64,
    pub max_x: i64,
    pub max_y: i64,
}

impl Rect2D {
    fn from_corners(p1: &Point2D, p2: &Point2D) -> Rect2D {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        Rect2D {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn area(&self) -> i64 {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }

    fn intersects(&self, other: &Rect2D) -> bool {
        other.max_y > self.min_y
            && other.min_y < self.max_y
            && other.max_x > self.min_x
            && other.min_x < self.max_x
    }
}

fn intersected(rect: Rect2D, points: &[Point2D]) -> bool {
    points
        .iter()
        .cycle()
        .tuple_windows()
        .take(points.len())
        .any(|(a, b)| rect.intersects(&Rect2D::from_corners(a, b)))
}

#[inline(always)]
fn day9(input: &str) -> (i64, i64) {
    let points: Vec<Point2D> = input.lines().map(Point2D::from_csv_str).collect();

    // Part one: Work out red tile corners.
    let part_one = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rect2D::from_corners(a, b))
        .map(|r| r.area())
        .max()
        .unwrap();

    // Part two: Calculate the rects which don't overlap the edges.
    let part_two = points
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let rect = Rect2D::from_corners(a, b);
            (!intersected(rect, &points)).then_some(rect.area())
        })
        .max()
        .unwrap();

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day9(input);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day9() {
        const SAMPLE_INPUT: &str = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};
        const SAMPLE_PART1_ANSWER: i64 = 50;
        const SAMPLE_PART2_ANSWER: i64 = 24;

        let (part1_answer, part2_answer) = day9(SAMPLE_INPUT);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    // #[test]
    // fn test_day9_actual() {
    //     const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day09.txt");
    //     const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day09.txt");
    //     let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
    //     let answers = (
    //         str::parse::<i64>(answers.0).unwrap(),
    //         str::parse::<i64>(answers.1).unwrap(),
    //     );
    //     assert_eq!(day8(ACTUAL_INPUT), answers);
    // }
}
