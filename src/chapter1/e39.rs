use std::f64::consts::*;

fn tan_cf(x: f64, k: usize) -> f64 {
    fn iter(x: f64, k: usize, cur: usize) -> f64 {
        if cur > k {
            0.0
        } else {
            x * x / (2.0 * (cur as f64) + 1.0 - iter(x, k, cur + 1))
        }
    }
    x / (1.0 - iter(x, k, 1))
}

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    let k = 100;
    for x in [0.0, FRAC_PI_8, FRAC_PI_6, FRAC_PI_4, FRAC_PI_3, PI] {
        assert!(f64_eq(tan_cf(x, k), x.tan()));
    }
}
#[test]
fn test() {
    main();
}
