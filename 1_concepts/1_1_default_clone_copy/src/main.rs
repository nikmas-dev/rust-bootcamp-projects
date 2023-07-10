pub type Coordinate = f64;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate,
}

#[derive(Debug, Clone)]
pub struct Polyline(Vec<Point>);

impl Polyline {
    pub fn new(points: Vec<Point>) -> Self {
        if points.is_empty() {
            panic!("there should be at least one point in the set of points");
        }

        Self(points)
    }

    pub fn points(&self) -> &[Point] {
        &self.0
    }
}

fn main() {
    let polyline = Polyline::new(vec![Default::default(), Default::default()]);
    println!("{:?}", polyline);
}
