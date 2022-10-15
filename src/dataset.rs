use rand::{thread_rng, Rng};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance_squared(&self, other: &Point) -> f64 {
        let d1 = self.x - other.x;
        let d2 = self.y - other.y;
        d1 * d1 + d2 * d2
    }
}

pub fn string_to_points(contents: &String) -> Vec<Point> {
    // To do: Error handling: Unwrapping of line + expected # elements 
    let mut points: Vec<Point> = Vec::new();

    for line in contents.lines() {

        let values: Vec<f64> = line.split(',')
                                   .map(|val| f64::from_str(val.trim())
                                   .unwrap())
                                   .collect();
        
        let c = Point::new(values[1], values[2]);
        points.push(c);
    }
    points
}

// Probably useless
pub fn random_points(n: usize, mn: f64, mx: f64) -> Vec<Point> {
    let mut rng = thread_rng();
    let mut points: Vec<Point> = Vec::new();

    for _ in 0..n {
        let x: f64 = rng.gen_range(mn, mx);
        let y: f64 = rng.gen_range(mn, mx);
        let c = Point::new(x, y);
        points.push(c);
    }
    points
}