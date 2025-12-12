use aoc2025::utils;
use itertools::Itertools;
use std::cmp::{max, min};
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

fn find_largest(rectangles: Vec<(Point, Point, i64)>, edges: Vec<(Point, Point)>) -> Option<usize> {
    'rects: for rectangle in rectangles {
        let (c1, c2, area) = rectangle;
        let (xmin, xmax) = (min(c1.x, c2.x), max(c1.x, c2.x));
        let (ymin, ymax) = (min(c1.y, c2.y), max(c1.y, c2.y));

        for &edge in edges.iter() {
            let (e1, e2) = edge;
            let (exmin, exmax) = (min(e1.x, e2.x), max(e1.x, e2.x));
            let (eymin, eymax) = (min(e1.y, e2.y), max(e1.y, e2.y));
            if xmin >= exmax || xmax <= exmin {
                continue;
            }
            if ymin >= eymax || ymax <= eymin {
                continue;
            }
            continue 'rects;
        }
        return Some(area as usize);
    }
    None
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            let split = l.split(",").collect::<Vec<&str>>();
            Point {
                x: split[0].parse::<i64>().unwrap(),
                y: split[1].parse::<i64>().unwrap(),
            }
        })
        .collect();

    let mut edges = points
        .iter()
        .cloned()
        .tuple_windows::<(Point, Point)>()
        .map(|(e, o)| if e.x < o.x { (e, o) } else { (o, e) })
        .sorted_by(|e, o| {
            let e_area = ((e.0.x - e.1.x).abs() + 1) * ((e.0.y - e.1.y).abs() + 1);
            let o_area = ((o.0.x - o.1.x).abs() + 1) * ((o.0.y - o.1.y).abs() + 1);
            e_area.cmp(&o_area)
        })
        .collect::<Vec<_>>();
    let (first, last) = (points[0], points[points.len() - 1]);
    edges.push((last, first));

    let part1 = points
        .iter()
        .combinations(2)
        .filter_map(|points| {
            let (p1, p2) = (points[0], points[1]);
            if p1.x == p2.x || p1.y == p2.y {
                None
            } else {
                Some(((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1))
            }
        })
        .max();

    let mut candidates: Vec<_> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (p1, p2) = (points[i], points[j]);
            candidates.push(if p1.x < p2.x { (p1, p2) } else { (p2, p1) });
        }
    }

    let rectangles = candidates
        .iter()
        .map(|&p| {
            let area = ((p.0.x - p.1.x).abs() + 1) * ((p.0.y - p.1.y).abs() + 1);
            (p.0, p.1, area)
        })
        .sorted_by_key(|i| i.2)
        .rev()
        .collect::<Vec<_>>();

    let solution = Solution {
        part1: part1.unwrap_or(0) as usize,
        part2: find_largest(rectangles, edges).unwrap(),
    };
    println!("{:?}", solution);
}
