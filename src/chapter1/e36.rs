use std::{fmt::Display, ops::Sub};

const TOLERANCE: f64 = 0.00001;

fn fixed_point<T>(f: impl Fn(T) -> T, first_guess: T) -> T
where
    T: Sub + Sub<Output = T> + Into<f64> + PartialOrd + Copy + Display,
{
    fn is_close_enough<T>(v1: T, v2: T) -> bool
    where
        T: Sub + Sub<Output = T> + Into<f64> + PartialOrd,
    {
        f64::abs((v1 - v2).into()) < TOLERANCE
    }
    fn try_it<T>(f: impl Fn(T) -> T, guess: T) -> T
    where
        T: Sub + Sub<Output = T> + Into<f64> + PartialOrd + Copy + Display,
    {
        println!("{}", guess);
        let next = f(guess);
        if is_close_enough(guess, next) {
            next
        } else {
            try_it(f, next)
        }
    }
    try_it(f, first_guess)
}

fn f64_eq(a: f64, b: f64) -> bool {
    ((a - b) / a).abs() < 0.00001
}

fn main() {
    let x = fixed_point(|x: f64| 1000_f64.ln() / x.ln(), 2.0);
    println!();
    assert!(f64_eq(x.powf(x), 1000.0));
    let x = fixed_point(|x: f64| 0.5 * (x + 1000_f64.ln() / x.ln()), 2.0);
    println!();
    assert!(f64_eq(x.powf(x), 1000.0));
}
#[test]
fn test() {
    main();
}
