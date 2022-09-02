use std::fmt::Display;

trait Rect {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}

struct Rect1 {
    seg1: Segment,
    seg2: Segment,
    seg3: Segment,
    seg4: Segment,
}
impl Rect1 {
    fn new(seg1: Segment, seg2: Segment, seg3: Segment, seg4: Segment) -> Self {
        Self {
            seg1,
            seg2,
            seg3,
            seg4,
        }
    }
    fn seg1(&self) -> Segment {
        self.seg1
    }
    fn seg2(&self) -> Segment {
        self.seg2
    }
    fn seg3(&self) -> Segment {
        self.seg3
    }
    fn seg4(&self) -> Segment {
        self.seg4
    }
}
impl Rect for Rect1 {
    fn area(&self) -> f64 {
        self.seg1.len() * self.seg2.len()
    }

    fn circumference(&self) -> f64 {
        self.seg1.len() + self.seg2.len() + self.seg3.len() + self.seg4.len()
    }
}

struct Rect2 {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
}
impl Rect2 {
    fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        Self { p1, p2, p3, p4 }
    }
    fn p1(&self) -> Point {
        self.p1
    }
    fn p2(&self) -> Point {
        self.p2
    }
    fn p3(&self) -> Point {
        self.p3
    }
    fn p4(&self) -> Point {
        self.p4
    }
}
impl Rect for Rect2 {
    fn area(&self) -> f64 {
        let len1 = distance(&self.p1(), &self.p2());
        let len2 = distance(&self.p2(), &self.p3());
        len1 * len2
    }

    fn circumference(&self) -> f64 {
        let len1 = distance(&self.p1(), &self.p2());
        let len2 = distance(&self.p2(), &self.p3());
        let len3 = distance(&self.p3(), &self.p4());
        let len4 = distance(&self.p4(), &self.p1());
        len1 + len2 + len3 + len4
    }
}

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
    fn len(&self) -> f64 {
        distance(&self.start, &self.end)
    }
}
fn distance(p1: &Point, p2: &Point) -> f64 {
    let x1 = p1.x;
    let x2 = p2.x;
    let y1 = p1.y;
    let y2 = p2.y;
    (square(x1 - x2) + square(y1 - y2)).sqrt()
}
fn square(x: f64) -> f64 {
    x * x
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
    let rect1 = Rect1::new(
        Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
        Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0)),
        Segment::new(Point::new(1.0, 1.0), Point::new(0.0, 1.0)),
        Segment::new(Point::new(0.0, 1.0), Point::new(0.0, 0.0)),
    );
    println!("{},{}", rect1.circumference(), rect1.area());
    let rect2 = Rect2::new(
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 1.0),
    );
    println!("{},{}", rect2.circumference(), rect2.area());
}

#[test]
fn test() {
    main();
}
