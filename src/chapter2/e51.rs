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

fn below(painter1: impl Fn(Frame), painter2: impl Fn(Frame)) -> impl Fn(Frame) {
    let split_point = Vect::new(0.0, 0.5);
    let painter_down = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        split_point,
    );
    let painter_up = transform_painter(
        painter2,
        split_point,
        Vect::new(1.0, 0.5),
        Vect::new(0.0, 1.0),
    );
    move |frame| {
        painter_down(frame);
        painter_up(frame);
    }
}
fn beside(painter1: impl Fn(Frame), painter2: impl Fn(Frame)) -> impl Fn(Frame) {
    let split_point = Vect::new(0.5, 0.0);
    let painter_left = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        split_point,
        Vect::new(0.0, 1.0),
    );
    let painter_right = transform_painter(
        painter2,
        split_point,
        Vect::new(1.0, 0.0),
        Vect::new(0.5, 1.0),
    );
    move |frame| {
        painter_left(frame);
        painter_right(frame);
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

fn rotate270(painter: impl Fn(Frame)) -> impl Fn(Frame) {
    transform_painter(
        painter,
        Vect::new(0.0, 1.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}

fn below1(painter1: impl Fn(Frame), painter2: impl Fn(Frame)) -> impl Fn(Frame) {
    rotate270(beside(rotate90(painter2), rotate90(painter1)))
}

fn main() {
    let painter1 = |frame| {
        let s = Segment::new(Vect::new(0.0, 0.0), Vect::new(1.0, 1.0));
        segment_to_painter(&[s])(frame)
    };
    let painter2 = |frame| {
        let s = Segment::new(Vect::new(0.0, 0.0), Vect::new(1.0, 0.5));
        segment_to_painter(&[s])(frame)
    };
    let f = Frame::new(
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 1.0),
    );
    painter1(f);
    painter2(f);
    below(painter1, painter2)(f);
    beside(painter1, painter2)(f);
    below1(painter1, painter2)(f);
}
#[test]
fn test() {
    main();
}
