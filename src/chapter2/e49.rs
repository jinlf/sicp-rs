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
struct Frame {
    origin: Vect,
    edge1: Vect,
    edge2: Vect,
}
impl Frame {
    fn new(origin: Vect, edge1: Vect, edge2: Vect) -> Self {
        Self {
            origin,
            edge1,
            edge2,
        }
    }
    fn origin(&self) -> Vect {
        self.origin
    }
    fn edge1(&self) -> Vect {
        self.edge1
    }
    fn edge2(&self) -> Vect {
        self.edge2
    }
    fn coord_map(&self) -> impl Fn(Vect) -> Vect + '_ {
        |v| self.origin() + (self.edge1().scale(v.xcor()) + self.edge2().scale(v.ycor()))
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
    fn midpoint(&self) -> Vect {
        Vect::new(
            (self.start().xcor() + self.end().xcor()) / 2.0,
            (self.start().ycor() + self.end().ycor()) / 2.0,
        )
    }
}

fn segment_to_painter(segment_list: &[Segment]) -> impl Fn(&Frame) + '_ {
    |frame| {
        segment_list.iter().for_each(|segment| {
            draw_line(
                frame.coord_map()(segment.start()),
                frame.coord_map()(segment.end()),
            )
        })
    }
}
fn draw_line(start: Vect, end: Vect) {
    println!("draw from {:?} to {:?}", start, end);
}

fn edge_painter() -> impl Fn(&Frame) {
    |frame| {
        let p1 = frame.origin();
        let p2 = frame.edge1();
        let p3 = frame.edge1() + frame.edge2();
        let p4 = frame.edge2();

        let s1 = Segment::new(p1, p2);
        let s2 = Segment::new(p2, p3);
        let s3 = Segment::new(p3, p4);
        let s4 = Segment::new(p4, p1);

        segment_to_painter(&[s1, s2, s3, s4])(frame)
    }
}

fn cross_painter() -> impl Fn(&Frame) {
    |frame| {
        let p1 = frame.origin();
        let p2 = frame.edge1();
        let p3 = frame.edge1() + frame.edge2();
        let p4 = frame.edge2();

        let s1 = Segment::new(p1, p3);
        let s2 = Segment::new(p2, p4);

        segment_to_painter(&[s1, s2])(frame)
    }
}

fn diamond_painter() -> impl Fn(&Frame) {
    |frame| {
        let p1 = frame.origin();
        let p2 = frame.edge1();
        let p3 = frame.edge1() + frame.edge2();
        let p4 = frame.edge2();

        let p1_2 = Segment::new(p1, p2).midpoint();
        let p2_3 = Segment::new(p2, p3).midpoint();
        let p3_4 = Segment::new(p3, p4).midpoint();
        let p4_1 = Segment::new(p4, p1).midpoint();

        let s1 = Segment::new(p1_2, p2_3);
        let s2 = Segment::new(p2_3, p3_4);
        let s3 = Segment::new(p3_4, p4_1);
        let s4 = Segment::new(p4_1, p1_2);

        segment_to_painter(&[s1, s2, s3, s4])(frame)
    }
}

fn main() {
    let f = Frame::new(
        Vect::new(1.0, 2.0),
        Vect::new(3.0, 4.0),
        Vect::new(5.0, 6.0),
    );
    edge_painter()(&f);
    cross_painter()(&f);
    diamond_painter()(&f);
    // wave_painter()(&f); //TODO
}
#[test]
fn test() {
    main();
}
