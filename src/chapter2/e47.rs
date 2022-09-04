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

trait Frame {
    fn new(origin: &Vect, edge1: &Vect, edge2: &Vect) -> Self
    where
        Self: Sized;
    fn origin(&self) -> &Vect;
    fn edge1(&self) -> &Vect;
    fn edge2(&self) -> &Vect;
}

#[derive(Debug)]
struct Frame1(Vec<Vect>);
impl Frame for Frame1 {
    fn new(origin: &Vect, edge1: &Vect, edge2: &Vect) -> Self
    where
        Self: Sized,
    {
        Self(vec![*origin, *edge1, *edge2])
    }

    fn origin(&self) -> &Vect {
        &self.0[0]
    }

    fn edge1(&self) -> &Vect {
        &self.0[1]
    }

    fn edge2(&self) -> &Vect {
        &self.0[2]
    }
}
#[derive(Debug)]
struct Frame2((Vect, Vect, Vect));
impl Frame for Frame2 {
    fn new(origin: &Vect, edge1: &Vect, edge2: &Vect) -> Self
    where
        Self: Sized,
    {
        Self((*origin, *edge1, *edge2))
    }

    fn origin(&self) -> &Vect {
        &self.0 .0
    }

    fn edge1(&self) -> &Vect {
        &self.0 .1
    }

    fn edge2(&self) -> &Vect {
        &self.0 .2
    }
}

fn main() {
    let origin = Vect::new(1.0, 2.0);
    let edge1 = Vect::new(3.0, 4.0);
    let edge2 = Vect::new(5.0, 6.0);
    let f1 = Frame1::new(&origin, &edge1, &edge2);
    let f2 = Frame2::new(&origin, &edge1, &edge2);
    println!("{:?}", f1);
    println!("{:?}, {:?}, {:?}", f1.origin(), f1.edge1(), f1.edge2());
    println!("{:?}", f2);
    println!("{:?}, {:?}, {:?}", f2.origin(), f2.edge1(), f2.edge2());
}
#[test]
fn test() {
    main();
}
