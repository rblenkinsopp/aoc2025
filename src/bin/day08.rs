use aoc2025::get_input_as_str;
use atoi::atoi;
use rayon::prelude::*;
use union_find::{QuickFindUf, UnionBySize, UnionFind};

#[derive(Eq, PartialEq)]
struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    fn from_csv_str(s: &str) -> Point3D {
        let mut parts = s.split(',');
        let x = atoi(parts.next().unwrap().as_bytes()).unwrap();
        let y = atoi(parts.next().unwrap().as_bytes()).unwrap();
        let z = atoi(parts.next().unwrap().as_bytes()).unwrap();
        Self { x, y, z }
    }
}

#[inline(always)]
fn squared_distance(a: &Point3D, b: &Point3D) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

#[inline(always)]
fn day8(input: &str, edge_limit: usize) -> (i64, i64) {
    let points: Vec<Point3D> = input.lines().map(Point3D::from_csv_str).collect();
    let num_points = points.len();
    let num_edges = num_points * (num_points - 1) / 2;

    // Compute the graph edges and distances and sort by ascending distance.
    let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(num_edges);
    for i in 0..num_points {
        for j in (i + 1)..num_points {
            let point_a = &points[i];
            let point_b = &points[j];
            edges.push((squared_distance(point_a, point_b), i, j));
        }
    }
    edges.par_sort_unstable_by_key(|(distance, _, _)| *distance);

    // Part 1: Use only the first 1000 shortest edges.
    let mut num_circuits = num_points;
    let mut union_find = QuickFindUf::<UnionBySize>::new(num_points);
    for &(_, i, j) in &edges[..edge_limit] {
        if union_find.union(i, j) {
            num_circuits -= 1;
        }
    }

    let mut circuits = vec![0usize; num_points];
    for i in 0..num_points {
        circuits[union_find.find(i)] += 1;
    }
    circuits.sort_unstable();

    let part_one = circuits
        .iter()
        .rev()
        .take(3)
        .product::<usize>() as i64;

    // Part 2: Continue until we have a fully connected graph (1 circuit).
    let mut part_two = 0;
    for &(_, i, j) in &edges[edge_limit..] {
        if union_find.union(i, j) {
            num_circuits -= 1;
            if num_circuits == 1 {
                part_two = points[i].x * points[j].x;
                break;
            }
        }
    }

    (part_one, part_two)
}

#[inline(always)]
fn main() {
    let input = get_input_as_str();
    let (p1, p2) = day8(input, 1000);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_day8() {
        const SAMPLE_INPUT: &str = indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};
        const SAMPLE_PART1_ANSWER: i64 = 40;
        const SAMPLE_PART2_ANSWER: i64 = 25272;

        let (part1_answer, part2_answer) = day8(SAMPLE_INPUT, 10);
        assert_eq!(part1_answer, SAMPLE_PART1_ANSWER);
        assert_eq!(part2_answer, SAMPLE_PART2_ANSWER);
    }

    #[test]
    fn test_day8_actual() {
        const ACTUAL_INPUT: &str = include_str!("../../data/inputs/day08.txt");
        const ACTUAL_ANSWERS: &str = include_str!("../../data/answers/day08.txt");
        let answers = ACTUAL_ANSWERS.split_once("\n").unwrap();
        let answers = (
            str::parse::<i64>(answers.0).unwrap(),
            str::parse::<i64>(answers.1).unwrap(),
        );
        assert_eq!(day8(ACTUAL_INPUT, 1000), answers);
    }
}
