use std::ops::{Add, Div};

fn is_good_enough(guess: f64, new_guess: f64) -> bool {
    ((new_guess - guess) / guess).abs() < 0.00001
}
fn sqrt_iter(guess: f64, x: f64) -> f64 {
    let new_guess = improve(guess, x);
    if is_good_enough(guess, new_guess) {
        guess
    } else {
        sqrt_iter(new_guess, x)
    }
}
fn improve(guess: f64, x: f64) -> f64 {
    average(guess, x / guess)
}
fn average<T>(x: T, y: T) -> T
where
    T: Add + Add<Output = T> + Div + Div<Output = T> + From<f64>,
{
    (x + y) / 2_f64.into()
}
fn sqrt(x: f64) -> f64 {
    sqrt_iter(1.0, x)
}
fn f64_eq(x: f64, y: f64) -> bool {
    ((x - y) / y).abs() < 0.00001
}

fn main() {
    assert!(f64_eq(sqrt(9.0), 9_f64.sqrt()));
    assert!(f64_eq(sqrt(f64::MAX), f64::MAX.sqrt()));
    assert!(f64_eq(sqrt(f64::MIN_POSITIVE), f64::MIN_POSITIVE.sqrt()));
}
#[test]
fn test() {
    main();
}
