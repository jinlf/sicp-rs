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
}

fn main() {
    assert!(Interval::new(3.0, 5.0).is_ok());
    assert!(Interval::new(-3.0, 5.0).is_err());
}
#[test]
fn test() {
    main();
}
