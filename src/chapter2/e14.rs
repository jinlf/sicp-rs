use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct Interval {
    lower_bound: f64,
    upper_bound: f64,
}
impl Interval {
    fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
    fn lower_bound(&self) -> f64 {
        self.lower_bound
    }
    fn upper_bound(&self) -> f64 {
        self.upper_bound
    }
    fn width(&self) -> f64 {
        (self.upper_bound() - self.lower_bound()) / 2.0
    }
    fn new_with_center_percent(center: f64, percent: f64) -> Self {
        let lower = center * (100.0 - percent) / 100.0;
        let upper = center * (100.0 + percent) / 100.0;
        Self::new(lower, upper)
    }
    fn center(&self) -> f64 {
        (self.lower_bound() + self.upper_bound()) / 2.0
    }
    fn percent(&self) -> f64 {
        self.width() / self.center() * 100.0
    }
}
impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.lower_bound() + rhs.lower_bound(),
            self.upper_bound() + rhs.upper_bound(),
        )
    }
}
impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.lower_bound() - rhs.upper_bound(),
            self.upper_bound() - rhs.lower_bound(),
        )
    }
}
impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let p1 = self.lower_bound() * rhs.lower_bound();
        let p2 = self.lower_bound() * rhs.upper_bound();
        let p3 = self.upper_bound() * rhs.lower_bound();
        let p4 = self.upper_bound() * rhs.upper_bound();
        Self::new(min(vec![p1, p2, p3, p4]), max(vec![p1, p2, p3, p4]))
    }
}
impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Interval::new(1.0 / rhs.upper_bound(), 1.0 / rhs.lower_bound())
    }
}
fn min(v: Vec<f64>) -> f64 {
    let mut v = v.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[0]
}
fn max(v: Vec<f64>) -> f64 {
    let mut v = v.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() - 1]
}

fn par1(r1: Interval, r2: Interval) -> Interval {
    (r1 * r2) / (r1 + r2)
}
fn par2(r1: Interval, r2: Interval) -> Interval {
    let one = Interval::new(1.0, 1.0);
    one / ((one / r1) + (one / r2))
}

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    let a = Interval::new(100.0, 200.0);
    let b = Interval::new(100.0, 101.0);
    let a_a = a / a;
    let a_b = a / b;

    let p1 = par1(a_a, a_b);
    let p2 = par2(a_a, a_b);
    println!("{:?}", p1);
    println!("{:?}", p2);

    println!("{},{}%", a.center(), a.percent());
    println!("{},{}%", b.center(), b.percent());
    println!("{},{}%", a_a.center(), a_a.percent());
    println!("{},{}%", a_b.center(), a_b.percent());
}
#[test]
fn test() {
    main();
}
