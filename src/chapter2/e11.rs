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
}
impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let l1 = self.lower_bound();
        let u1 = self.upper_bound();
        let l2 = rhs.lower_bound();
        let u2 = rhs.upper_bound();

        let (min, max) = if l1 < 0.0 {
            if l2 < 0.0 {
                (u1 * u2, l1 * l2)
            } else if l2.abs() < f64::EPSILON {
                (0.0, 0.0)
            } else {
                (l1 * u2, u1 * l2)
            }
        } else if l1.abs() < f64::EPSILON {
            (0.0, 0.0)
        } else {
            if l2 < 0.0 {
                (u1 * l2, l1 * u2)
            } else if l2.abs() < f64::EPSILON {
                (0.0, 0.0)
            } else {
                (l1 * l2, u1 * u2)
            }
        };

        Self::new(min, max)
    }
}

fn main() {
    // 只找到7种情况
}
#[test]
fn test() {
    main();
}
