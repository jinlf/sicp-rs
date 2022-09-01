use rand::{thread_rng, Rng};
use std::ops::Sub;

const TOLERANCE: f64 = 0.00001;

fn fixed_point<T>(f: impl Fn(T) -> T, first_guess: T) -> T
where
    T: Sub + Sub<Output = T> + Into<f64> + PartialOrd + Copy,
{
    fn is_close_enough<T>(v1: T, v2: T) -> bool
    where
        T: Sub + Sub<Output = T> + Into<f64> + PartialOrd,
    {
        f64::abs((v1 - v2).into()) < TOLERANCE
    }
    fn try_it<T>(f: impl Fn(T) -> T, guess: T) -> T
    where
        T: Sub + Sub<Output = T> + Into<f64> + PartialOrd + Copy,
    {
        let next = f(guess);
        if is_close_enough(guess, next) {
            next
        } else {
            try_it(f, next)
        }
    }
    try_it(f, first_guess)
}

fn newton_method(g: impl Fn(f64) -> f64 + Copy, guess: f64) -> f64 {
    fixed_point(newton_transform(g), guess)
}
fn newton_transform(g: impl Fn(f64) -> f64 + Copy) -> impl Fn(f64) -> f64 {
    move |x| x - (g(x) / derive(g)(x))
}
const DX: f64 = 0.00001;
fn derive(g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |x| (g(x + DX) - g(x)) / DX
}

fn cubic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
    move |x| x * x * x + a * x * x + b * x + c
}
fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    let (a, b, c) = (thread_rng().gen(), thread_rng().gen(), thread_rng().gen());
    let x = newton_method(&cubic(a, b, c), 1.0);
    println!("{}", x);
    assert!(f64_eq(cubic(a, b, c)(x), 0.0))
}

#[test]
fn test() {
    main();
}
