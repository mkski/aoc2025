use aoc2025::utils;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,  
}

impl Line {
    fn intersects(&self, other: Line) -> bool {
        let d = (self.p1.x - other.p2.x) * (other.p1.y - other.p2.y) - (self.p1.y - self.p2.y) * (other.p1.x - other.p2.x);
        if d == 0 {
            return false
        }
        let tn = (self.p1.x - other.p1.x) * (other.p1.y - other.p2.y) - (self.p1.y - other.p1.y) * (other.p1.x - other.p2.x);
        let t = tn as f64 / d as f64;
        let un = (self.p1.x - self.p2.x) * (self.p1.y - other.p1.y) - (self.p1.y - self.p2.y) * (self.p1.x - other.p1.x);
        let u = (-un) as f64 / d as f64;

        (u >= 0.0 && u <= 1.0) && (t >= 0.0 && t <= 1.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct VerticalEdge {
    x: i64,
    y: (i64, i64),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct HorizontalEdge {
    x: (i64, i64),
    y: i64,
}

fn make_line(from: &Point, to: &Point) -> Vec<Point> {
    let mut line: Vec<Point> = Vec::new();
    if from.x == to.x {
        for y in min(from.y, to.y)..max(from.y, to.y) {
            line.push(Point { x: from.x, y })
        }
    } else {
        for x in min(from.x, to.x)..max(from.x, to.x) {
            line.push(Point { x, y: from.y })
        }
    }
    line.push(*to);
    line
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

    // let mut vertices: HashSet<Point> = HashSet::new();
    // for p in points.iter() {
    //     vertices.insert(*p);
    // }

    let mut edges: Vec<(&Point, &Point)> = Vec::new();
    let mut vedges: Vec<VerticalEdge> = Vec::new();
    let mut hedges: Vec<HorizontalEdge> = Vec::new();
    let mut boundary: HashSet<Point> = HashSet::new();
    for edge in points.iter().tuple_windows::<(&Point, &Point)>() {
        // let (from, to) = edge;
        edges.push(edge);
        // edges.push(Line { p1: *from, p2: *to });
        // edges.push(if from.x == to.x {
        //     let (ymin, ymax) = (min(from.y, to.y), max(from.y, to.y));
        //     Line { p1: Point { x: from.x, y: ymin + 1 }, p2: Point { x: from.x, y: ymax - 1 } }
        // } else {
        //     let (xmin, xmax) = (min(from.x, to.x), max(from.x, to.x));
        //     Line { p1: Point { x: xmin + 1, y:from.y }, p2: Point { x: xmax - 1, y: from.y } }
        // });

        // boundary.extend(make_line(from, to));
        // if edge.0.x == edge.1.x {
        //     vedges.push(VerticalEdge {
        //         x: edge.0.x,
        //         y: (min(edge.0.y, edge.1.y), max(edge.0.y, edge.1.y)),
        //     });
        // } else if edge.0.y == edge.1.y {
        //     hedges.push(HorizontalEdge {
        //         x: (min(edge.0.x, edge.1.x), max(edge.0.x, edge.1.x)),
        //         y: edge.0.y,
        //     });
        // }
    }
    // boundary.extend(make_line(&points[points.len() - 1], &points[0]));
    let (first, last) = (points[0], points[points.len() - 1]);
    edges.push((&last, &first));

    // edges.push(Line { p1: last, p2: first });
    // edges.push(if first.x == last.x {
    //     let (ymin, ymax) = (min(first.y, last.y), max(first.y, last.y));
    //     Line { p1: Point { x: first.x, y: ymin + 1 }, p2: Point { x: first.x, y: ymax - 1 } }
    // } else {
    //     let (xmin, xmax) = (min(first.x, last.x), max(first.x, last.x));
    //     Line { p1: Point { x: xmin + 1, y:first.y }, p2: Point { x: xmax - 1, y: first.y } }
    // });
    // if first.x == last.x {
    //     vedges.push(VerticalEdge {
    //         x: first.x,
    //         y: (min(first.y, last.y), max(first.y, last.y)),
    //     });
    // } else if first.y == last.y {
    //     hedges.push(HorizontalEdge {
    //         x: (min(first.x, last.x), max(first.x, last.x)),
    //         y: first.y,
    //     });
    // }

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

    let mut candidates: HashSet<(&Point, &Point)> = HashSet::new();
    for pair in points.iter().combinations(2) {
        let (p1, p2) = (pair[0], pair[1]);
        if p1.x == p2.x || p1.y == p2.y {
            continue;
        }
        candidates.insert(if p1.x < p2.x { (p1, p2) } else { (p2, p1) });
    }

    let part2 = candidates
        .iter()
        .filter_map(|p| {
            let (p1, p2) = (p.0, p.1);
            // println!();
            // println!("{p1:?} {p2:?}");
            // let line = Line { p1: *p1, p2: *p2 };
            // let line = if p1.x == p2.x {
            //     let (ymin, ymax) = (min(p1.y, p2.y), max(p1.y, p2.y));
            //     Line { p1: Point { x: p1.x, y: ymin }, p2: Point { x: p1.x, y: ymax - 1 } }
            // } else {
            //     let (xmin, xmax) = (min(p1.x, p2.x), max(p1.x, p2.x));
            //     Line { p1: Point { x: xmin, y:p1.y }, p2: Point { x: xmax - 1, y: p1.y } }
            // };

            let (ymin, ymax) = (min(p1.y, p2.y), max(p1.y, p2.y));
            let (xmin, xmax) = (min(p1.x, p2.x), max(p1.x, p2.x));
            let line = Line { p1: Point { x: xmin, y: ymin }, p2: Point { x: xmax, y: ymax } };
            let p1 = Point { x: xmin + 1, y: ymin + 1 };
            let p2 = Point { x: xmax - 1, y: ymax - 1 };

            // for edge in edges.iter() {
            //     if line.intersects(*edge) {
            //         println!("  x {edge:?}");
            //         return None;
            //     }
            //     println!("  > {edge:?}");
            // }
            // let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            // println!("  = {area}");
            // Some(area) 


            let (ymin, ymax) = (min(p1.y, p2.y), max(p1.y, p2.y));
            // will work for part 1 if we dont exclude any candidates
            println!();
            println!("{:?} <{:?} {:?}>", line, ymin, ymax);
            for edge in vedges.iter() {
                let (eminy, emaxy) = (min(edge.y.0, edge.y.1), max(edge.y.0, edge.y.1));
                let edge_line = Line { p1: Point { x: edge.x, y: eminy }, p2: Point { x: edge.x, y: emaxy } };

                if edge.x < p1.x || edge.x > p2.x || emaxy < ymin || eminy > ymax {
                    // println!("  o- edge {:?} is outside candidate box", edge);
                    continue; // edge is outside of candidate box
                }
                // if edge.x == p1.x && (eminy >= ymax || edge.y.1 <= ymin || (edge.y.0 >= ymin && edge.y.1 <= ymax))
                if edge.x == p1.x && (eminy < ymin && emaxy < ymax || eminy > ymin && emaxy > ymax)
                {
                    println!("  o- edge {:?} is within box edge", edge);
                    continue; // edge coincides with candidate box
                }
                if edge.y.0 >= p1.y && edge.y.1 <= p2.y {
                    println!("  x- edge {:?} is completely engulfed in candidate box", edge);
                    return None; // edge is completely engulfed in candidate box
                }
                if edge.y.0 <= ymin && edge.y.1 > ymax {
                    println!("  x- edge {:?} is completely engulfed in candidate box 2", edge);
                    return None; // edge is completely engulfed in candidate box
                }
                if edge.y.0 <= ymin && edge.y.1 > ymin && edge.y.1 < ymax {
                    println!("  x- edge {:?} starts outside above candidate box and ends inside", edge);
                    return None; // edge starts outside above candidate box and ends inside
                }
                if edge.y.0 > ymin && edge.y.1 > ymax {
                    println!("  x- edge {:?} starts inside candidate box and ends outside below", edge);
                    return None; // edge starts inside candidate box and ends outside below
                }
                if edge.y.0 < ymin && edge.y.1 > ymax {
                    println!("  x- edge {:?} starts outside below and ends outside above", edge);
                    return None; // edge starts outside below and ends outside above
                }
                // if line.intersects(edge_line) {
                //     println!("  x- {edge:?} intersects");
                //     return None;
                // }
                println!("  |- {:?}", edge);
            }

            let area = ((xmax-xmin).abs() + 1) * ((ymax-ymin).abs() + 1);
            println!("  = {area}");
            Some(area)
        })
        .max();

    let solution = Solution {
        part1: part1.unwrap_or(0) as usize,
        part2: part2.unwrap_or(0) as usize,
    };
    println!("{:?}", solution);
}
