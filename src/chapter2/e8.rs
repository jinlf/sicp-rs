use std::ops::Sub;

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

fn main() {
    let i1 = Interval::new(3.0, 5.0);
    let i2 = Interval::new(4.0, 7.0);
    println!("{:?}", i2 - i1);
}
#[test]
fn test() {
    main();
}
