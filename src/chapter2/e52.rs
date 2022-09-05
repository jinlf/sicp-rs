use std::fmt::Debug;
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
            x: self.x * s,
            y: self.y * s,
        }
    }
}
impl Add for Vect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.xcor() + rhs.xcor(), self.ycor() + rhs.ycor())
    }
}
impl Sub for Vect {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.xcor() - rhs.xcor(), self.ycor() - rhs.ycor())
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
    fn coord_map(&self) -> Box<dyn Fn(Vect) -> Vect + '_> {
        Box::new(|v| self.origin() + (self.edge1().scale(v.xcor()) + self.edge2().scale(v.ycor())))
    }
}

trait Painter: dyn_clone::DynClone + Debug {
    fn op(&self, frame: &Frame);
}
dyn_clone::clone_trait_object!(Painter);

fn flipped_pairs(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    let painter2 = beside(painter.clone(), flip_vert(painter));
    below(painter2.clone(), painter2)
}
fn right_split(painter: Box<dyn Painter>, n: usize) -> Box<dyn Painter> {
    if n == 0 {
        painter
    } else {
        let smaller = right_split(painter.clone(), n - 1);
        beside(painter, below(smaller.clone(), smaller))
    }
}
fn corner_split(painter: Box<dyn Painter>, n: usize) -> Box<dyn Painter> {
    if n == 0 {
        painter
    } else {
        let up = up_split(painter.clone(), n - 1);
        let right = right_split(painter.clone(), n - 1);
        let top_left = beside(up.clone(), up);
        let bottom_right = below(right.clone(), right);
        let corner = corner_split(painter.clone(), n - 1);
        beside(below(painter, top_left), below(bottom_right, corner))
    }
}
fn up_split(painter: Box<dyn Painter>, n: usize) -> Box<dyn Painter> {
    if n == 0 {
        painter
    } else {
        let smaller = up_split(painter.clone(), n - 1);
        below(painter, beside(smaller.clone(), smaller))
    }
}
fn square_of_four<'b, 'd>(
    tl: &'d dyn Fn(Box<dyn Painter + 'b>) -> Box<dyn Painter + 'static>,
    tr: &'d dyn Fn(Box<dyn Painter + 'b>) -> Box<dyn Painter + 'static>,
    bl: &'d dyn Fn(Box<dyn Painter + 'b>) -> Box<dyn Painter + 'static>,
    br: &'d dyn Fn(Box<dyn Painter + 'b>) -> Box<dyn Painter + 'static>,
) -> Box<dyn Fn(Box<dyn Painter + 'b>) -> Box<dyn Painter + 'static> + 'd> {
    Box::new(|painter: Box<dyn Painter + 'b>| {
        let top = beside(tl(painter.clone()), tr(painter.clone()));
        let bottom = beside(bl(painter.clone()), br(painter));
        below(bottom, top)
    })
}
fn flipped_pairs1(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    let combine4 = square_of_four(&identity, &flip_vert, &identity, &flip_vert);
    combine4(painter)
}
fn identity(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    painter
}
fn square_limit(painter: Box<dyn Painter>, n: usize) -> Box<dyn Painter> {
    let combine4 = square_of_four(&flip_horiz, &identity, &rotate180, &flip_vert);
    combine4(corner_split(painter, n))
}

fn segment_to_painter(segment_list: &[Segment]) -> Box<dyn Painter + '_> {
    #[derive(Debug, Clone)]
    struct SegementToPainter<'a> {
        segment_list: &'a [Segment],
    }
    impl<'a> Painter for SegementToPainter<'a> {
        fn op(&self, frame: &Frame) {
            println!("draw SegmentToPainter");
            self.segment_list.iter().for_each(|segment| {
                draw_line(
                    frame.coord_map()(segment.start()),
                    frame.coord_map()(segment.end()),
                )
            })
        }
    }
    Box::new(SegementToPainter { segment_list })
}
fn draw_line(start: Vect, end: Vect) {
    println!("draw from {:?} to {:?}", start, end);
}

fn transform_painter(
    painter: Box<dyn Painter>,
    origin: Vect,
    corner1: Vect,
    corner2: Vect,
) -> Box<dyn Painter> {
    #[derive(Debug, Clone)]
    struct TransformPainter {
        painter: Box<dyn Painter>,
        origin: Vect,
        corner1: Vect,
        corner2: Vect,
    }
    impl Painter for TransformPainter {
        fn op(&self, frame: &Frame) {
            println!("draw TransformPainter");
            let m = frame.coord_map();
            let new_origin = m(self.origin);
            self.painter.op(&Frame::new(
                new_origin,
                m(self.corner1) - new_origin,
                m(self.corner2) - new_origin,
            ))
        }
    }
    Box::new(TransformPainter {
        painter,
        origin,
        corner1,
        corner2,
    })
}
fn flip_vert(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(0.0, 1.0),
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 0.0),
    )
}
fn flip_horiz(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}

fn shirnk_to_upper_right(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(0.5, 0.5),
        Vect::new(1.0, 0.5),
        Vect::new(0.5, 1.0),
    )
}
fn rotate90(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(1.0, 0.0),
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 0.0),
    )
}
fn rotate180(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(1.0, 1.0),
        Vect::new(0.0, 1.0),
        Vect::new(1.0, 0.0),
    )
}
fn shirnk_inwards(painter: Box<dyn Painter>) -> Box<dyn Painter> {
    transform_painter(
        painter,
        Vect::new(0.0, 0.0),
        Vect::new(0.65, 0.35),
        Vect::new(0.35, 0.65),
    )
}
fn beside(painter1: Box<dyn Painter>, painter2: Box<dyn Painter>) -> Box<dyn Painter> {
    let split_point = Vect::new(0.5, 0.0);
    let paint_left = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        split_point,
        Vect::new(0.0, 1.0),
    );
    let paint_right = transform_painter(
        painter2,
        split_point,
        Vect::new(1.0, 0.0),
        Vect::new(0.5, 1.0),
    );

    #[derive(Debug, Clone)]
    struct BesidePainter {
        paint_left: Box<dyn Painter>,
        paint_right: Box<dyn Painter>,
    }
    impl Painter for BesidePainter {
        fn op(&self, frame: &Frame) {
            println!("draw BesidePainter");
            self.paint_left.op(&frame);
            self.paint_right.op(&frame);
        }
    }
    Box::new(BesidePainter {
        paint_left: dyn_clone::clone_box(&*paint_left),
        paint_right: dyn_clone::clone_box(&*paint_right),
    })
}

fn below(painter1: Box<dyn Painter>, painter2: Box<dyn Painter>) -> Box<dyn Painter> {
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

    #[derive(Debug, Clone)]
    struct BelowPainter {
        painter_down: Box<dyn Painter>,
        painter_up: Box<dyn Painter>,
    }
    impl Painter for BelowPainter {
        fn op(&self, frame: &Frame) {
            println!("draw BelowPainter");
            self.painter_down.op(frame);
            self.painter_up.op(frame);
        }
    }
    Box::new(BelowPainter {
        painter_down: dyn_clone::clone_box(&*painter_down),
        painter_up: dyn_clone::clone_box(&*painter_up),
    })
}
fn gen_wave() -> Box<dyn Fn(&Frame)> {
    Box::new(|frame| todo!())
}

#[derive(Debug, Clone)]
struct Wave {}
impl Painter for Wave {
    fn op(&self, frame: &Frame) {
        println!("draw wave");
    }
}

fn main() {
    let f = Frame::new(
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 1.0),
    );
    let wave = Box::new(Wave {});
    let wave2 = beside(wave.clone(), flip_vert(wave.clone()));
    let wave4 = below(wave2.clone(), wave2.clone());
    let new_wave4 = flipped_pairs(wave.clone());

    wave.op(&f);
    wave2.op(&f);
    wave4.op(&f);
    new_wave4.op(&f);
}
#[test]
fn test() {
    main();
}
