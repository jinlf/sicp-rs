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

#[derive(Debug, Clone, Copy)]
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

fn segment_to_painter(segment_list: &[Segment]) -> impl Fn(Frame) + '_ {
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

fn flip_horiz(painter: impl Fn(Frame)) -> impl Fn(Frame) {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}
fn transform_painter(
    painter: impl Fn(Frame),
    origin: Vect,
    corner1: Vect,
    corner2: Vect,
) -> impl Fn(Frame) {
    move |frame| {
        let m = frame.coord_map();
        let new_origin = m(origin);
        painter(Frame::new(
            new_origin,
            m(corner1) - new_origin,
            m(corner2) - new_origin,
        ))
    }
}

fn rotate90(painter: impl Fn(Frame)) -> impl Fn(Frame) {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 0.0),
    )
}

fn rotate180(painter: impl Fn(Frame)) -> impl Fn(Frame) {
    transform_painter(
        painter,
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 1.0),
        Vect::new(1.0, 0.0),
    )
}
fn rotate270(painter: impl Fn(Frame)) -> impl Fn(Frame) {
    transform_painter(
        painter,
        Vect::new(0.0, 1.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}

fn main() {
    let painter = |frame| {
        let s = Segment::new(Vect::new(0.0, 0.5), Vect::new(1.0, 0.5));
        segment_to_painter(&[s])(frame)
    };

    let f = Frame::new(
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 1.0),
    );

    painter(f);
    flip_horiz(painter)(f);
    rotate90(painter)(f);
    rotate180(painter)(f);
    rotate270(painter)(f);
}
#[test]
fn test() {
    main();
}
