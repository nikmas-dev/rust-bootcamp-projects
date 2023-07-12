use anyhow::{bail, Result};

pub type Coordinate = f64;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate,
}

#[derive(Debug, Clone)]
pub struct Polyline(Vec<Point>);

impl Polyline {
    pub fn new(points: Vec<Point>) -> Result<Self> {
        if points.is_empty() {
            bail!("there should be at least one point in the set of points");
        }

        Ok(Self(points))
    }

    pub fn points(&self) -> &[Point] {
        &self.0
    }

    pub fn push(&mut self, point: Point) {
        self.0.push(point);
    }

    pub fn pop(&mut self) -> Result<()> {
        if self.0.len() == 1 {
            bail!("cannot pop the last point: there should be at least one left");
        }

        self.0.pop();

        Ok(())
    }
}

fn main() {
    let polyline = Polyline::new(vec![Default::default(), Default::default()]).unwrap();
    println!("{:?}", polyline);
}
