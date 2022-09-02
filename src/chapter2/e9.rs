use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
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

fn main() {
    let i1 = Interval::new(3.0, 5.0);
    let i2 = Interval::new(4.0, 7.0);
    let w1 = i1.width();
    let w2 = i2.width();
    let i1_2 = i1 + i2;
    println!("{:?}", i1_2);
    assert_eq!(i1_2.width(), w1 + w2);

    let i1 = Interval::new(3.0, 5.0);
    let i2 = Interval::new(4.0, 7.0);
    let w1 = i1.width();
    let w2 = i2.width();
    let i2_1 = i2 - i1;
    println!("{:?}", i2_1);
    assert_eq!(i2_1.width(), w2 + w1);

    let i1 = Interval::new(3.0, 5.0);
    let i2 = Interval::new(4.0, 7.0);
    let w1 = i1.width();
    let w2 = i2.width();
    let i1_mul_2 = i1 * i2;
    println!("{:?}", i1_mul_2);
    assert_ne!(i1_mul_2.width(), w2 * w1);

    let i1 = Interval::new(3.0, 5.0);
    let i2 = Interval::new(4.0, 7.0);
    let w1 = i1.width();
    let w2 = i2.width();
    let i1_div_2 = i1 / i2;
    println!("{:?}", i1_div_2);
    assert_ne!(i1_div_2.width(), w2 / w1);
}
#[test]
fn test() {
    main();
}
