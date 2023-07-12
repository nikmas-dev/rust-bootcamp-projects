use thiserror::Error;

pub type Coordinate = f64;

#[derive(Error, Debug)]
#[error("there should be at least one point in the set of points")]
pub struct ConstructEmptyCollectionError;

#[derive(Error, Debug)]
#[error("cannot pop the last point: there should be at least one left")]
pub struct PopLastElementError;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate,
}

#[derive(Debug, Clone)]
pub struct Polyline(Vec<Point>);

impl Polyline {
    pub fn new(points: Vec<Point>) -> Result<Self, ConstructEmptyCollectionError> {
        if points.is_empty() {
            return Err(ConstructEmptyCollectionError);
        }

        Ok(Self(points))
    }

    pub fn points(&self) -> &[Point] {
        &self.0
    }

    pub fn push(&mut self, point: Point) {
        self.0.push(point);
    }

    pub fn pop(&mut self) -> Result<Point, PopLastElementError> {
        if self.0.len() == 1 {
            return Err(PopLastElementError);
        }

        Ok(self.0.pop().unwrap())
    }
}

fn main() {
    let mut polyline = Polyline::new(vec![Default::default(), Default::default()]).unwrap();
    println!("{:?}", polyline);
    assert_eq!(polyline.points().len(), 2);

    polyline.push(Default::default());
    assert_eq!(polyline.points().len(), 3);
    println!("{:?}", polyline);

    polyline.pop().unwrap();
    assert_eq!(polyline.points().len(), 2);
    println!("{:?}", polyline);

    polyline.pop().unwrap();
    assert_eq!(polyline.points().len(), 1);
    println!("{:?}", polyline);

    assert!(polyline.pop().is_err());
    println!("{:?}", polyline);
}
