use std::ops::Mul;

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

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    /*
    l1 = (1-p1)c1
    u1 = (1+p1)c1
    l2 = (1-p2)c2
    u2 = (1+p2)c2

    l3 = l1l2 = (1-p1)c1(1-p2)c2 = (1-p1-p2+p1p2)c1c2
    u3 = u1u2 = (1+p1)c1(1+p2)c2 = (1+p1+p2+p1p2)c1c2
    c3 = (l3+u3)/2 = (1+p1p2)c1c2
    p3 = (u3-l3)/2/c3 = (p1+p2)/(1+p1p2)
     */

    let c1 = 10.0;
    let p1 = 0.2;
    let c2 = 20.0;
    let p2 = 0.1;
    let i1 = Interval::new_with_center_percent(c1, p1 * 100.0);
    let i2 = Interval::new_with_center_percent(c2, p2 * 100.0);
    let i3 = i1 * i2;
    assert!(f64_eq(i3.center(), (1.0 + p1 * p2) * (c1 * c2)));
    assert!(f64_eq(i3.percent(), 100.0 * (p1 + p2) / (1.0 + p1 * p2)));
}
#[test]
fn test() {
    main();
}
