#[derive(Debug)]
struct Interval {
    lower_bound: f64,
    upper_bound: f64,
}
impl Interval {
    fn new(lower_bound: f64, upper_bound: f64) -> Result<Self, String> {
        if lower_bound < 0.0 && upper_bound > 0.0 {
            Err("over zero".to_owned())
        } else {
            Ok(Self {
                lower_bound,
                upper_bound,
            })
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
        Self::new(lower, upper).unwrap()
    }
    fn center(&self) -> f64 {
        (self.lower_bound() + self.upper_bound()) / 2.0
    }
    fn percent(&self) -> f64 {
        self.width() / self.center() * 100.0
    }
}

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    let center = 10.0;
    let percent = 20.0;
    let i = Interval::new_with_center_percent(10.0, 20.0);
    assert!(f64_eq(i.center(), center));
    assert!(f64_eq(i.percent(), percent));
}
#[test]
fn test() {
    main();
}
