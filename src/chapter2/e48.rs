use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Vect {
    x: f64,
    y: f64,
}
impl Vect {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn xcor(&self) -> f64 {
        self.x
    }
    fn ycor(&self) -> f64 {
        self.y
    }
    fn scale(&self, s: f64) -> Self {
        Self {
            x: s * self.xcor(),
            y: s * self.ycor(),
        }
    }
}
impl Add for Vect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.xcor() + rhs.xcor(),
            y: self.ycor() + rhs.ycor(),
        }
    }
}
impl Sub for Vect {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.xcor() - rhs.xcor(),
            y: self.ycor() - rhs.ycor(),
        }
    }
}

#[derive(Debug)]
struct Segment {
    start: Vect,
    end: Vect,
}
impl Segment {
    fn new(start: Vect, end: Vect) -> Self {
        Self { start, end }
    }
    fn start(&self) -> Vect {
        self.start
    }
    fn end(&self) -> Vect {
        self.end
    }
}

fn main() {
    let start = Vect::new(1.0, 2.0);
    let end = Vect::new(3.0, 4.0);
    let seg = Segment::new(start, end);
    println!("{:?}", seg);
    println!("{:?}, {:?}", seg.start(), seg.end());
}
#[test]
fn test() {
    main();
}
