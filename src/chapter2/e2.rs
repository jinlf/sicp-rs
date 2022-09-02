use std::fmt::Display;

#[derive(Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Segment {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
    fn start(&self) -> Point {
        self.start
    }
    fn end(&self) -> Point {
        self.end
    }
    fn midpoint(&self) -> Point {
        Point::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let seg = Segment::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
    println!("{}", seg.midpoint());
}

#[test]
fn test() {
    main();
}
