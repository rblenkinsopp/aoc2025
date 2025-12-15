use aoc2025::get_input_as_str;
use atoi::atoi;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    #[inline(always)]
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
    pub area: i64,
}

impl Rect2D {
    #[inline(always)]
    fn from_corner_pair(corners: (&Point2D, &Point2D)) -> Rect2D {
        let (p1, p2) = corners;
        Self::from_corners(p1, p2)
    }

    #[inline(always)]
    fn from_corners(p1: &Point2D, p2: &Point2D) -> Rect2D {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        let area = (max_x - min_x + 1) * (max_y - min_y + 1);
        Rect2D {
            min_x,
            min_y,
            max_x,
            max_y,
            area,
        }
    }

    #[inline(always)]
    fn intersects(&self, other: &Rect2D) -> bool {
        other.max_y > self.min_y
            && other.min_y < self.max_y
            && other.max_x > self.min_x
            && other.min_x < self.max_x
    }
}

#[inline(always)]
fn day9(input: &str) -> (i64, i64) {
    let points: Vec<Point2D> = input.lines().map(Point2D::from_csv_str).collect();
    let num_points = points.len();

    // Pre-calculate the edge pairs and 'rotate' the collection so that the edges which bisect the
    // circle input are near the start of the collection so pairs are quickly canceled out.
    let edges: Vec<Rect2D> = points
        .iter()
        .cycle()
        .skip(num_points / 2 - 1)
        .tuple_windows()
        .take(num_points)
        .map(Rect2D::from_corner_pair)
        .collect();

    // Pre-calculate the edge rectangles and corner pairs and sort by area.
    let mut rects: Vec<Rect2D> = points
        .iter()
        .tuple_combinations()
        .map(Rect2D::from_corner_pair)
        .collect();
    rects.par_sort_unstable_by_key(|r| r.area);
    rects.reverse();

    // Part one: Largest area rect.
    let part_one = rects.first().unwrap().area;

    // Part two: Calculate the rects which don't overlap the edges.
    let part_two = rects
        .iter()
        .find_map(|r| edges.iter().all(|e| !e.intersects(r)).then_some(r.area))
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

    #[test]
    fn test_day9_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day09.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day09.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day9(ACTUAL_INPUT), answers);
    }
}
