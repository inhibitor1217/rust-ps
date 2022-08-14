use std::{
    fmt::Display,
    io,
    ops::{Add, Neg},
};

fn parse_u32_stdin() -> u32 {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<u32>().unwrap();
    t
}

fn parse_point_stdin() -> Point {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let mut it = t.split_whitespace().map(|w| w.parse::<i32>().unwrap());
    Point {
        x: it.next().unwrap(),
        y: it.next().unwrap(),
    }
}

fn main() {
    let t = parse_u32_stdin();

    for _ in 0..t {
        let n = parse_u32_stdin();
        let mut points = vec![];
        for _ in 0..n {
            points.push(parse_point_stdin());
        }
        println!("{}", solve(points));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    fn len(&self) -> f64 {
        ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, PartialOrd)]
struct Answer(f64);

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= 1e-6
    }
}

fn solve(points: Vec<Point>) -> Answer {
    assert!(points.len() % 2 == 0);
    assert!(points.len() > 0);

    let can = candidates(&points, (points.len() / 2) as u32);

    can.iter()
        .map(Point::len)
        .map(Answer)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn candidates(points: &[Point], left_negations: u32) -> Vec<Point> {
    if points.len() == 0 {
        return vec![Point::zero()];
    }

    let mut next_candidates: Vec<Point> = vec![];

    if left_negations < points.len() as u32 {
        for pt in candidates(&points[1..], left_negations) {
            next_candidates.push(points[0] + pt);
        }
    }

    if left_negations > 0 {
        for pt in candidates(&points[1..], left_negations - 1) {
            next_candidates.push(-points[0] + pt);
        }
    }

    next_candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            solve(vec![
                Point { x: 5, y: 5 },
                Point { x: 5, y: -5 },
                Point { x: -5, y: 5 },
                Point { x: -5, y: -5 }
            ]),
            Answer(0.0),
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(
            solve(vec![
                Point {
                    x: -100_000,
                    y: -100_000
                },
                Point {
                    x: 100_000,
                    y: 100_000
                }
            ]),
            Answer(282842.712474619038)
        )
    }

    #[test]
    fn case_3() {
        assert_eq!(
            solve(vec![
                Point { x: 26, y: -76 },
                Point { x: 65, y: -83 },
                Point { x: 78, y: 38 },
                Point { x: 92, y: 22 },
                Point { x: -60, y: -42 },
                Point { x: -27, y: 85 },
                Point { x: 42, y: 46 },
                Point { x: -86, y: 98 },
                Point { x: 92, y: -47 },
                Point { x: -41, y: 38 },
            ]),
            Answer(13.341664064126334)
        )
    }
}
