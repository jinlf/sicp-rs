use std::{fmt::Display, ops::Sub};

fn iterative_improve<'a, T>(
    is_good_enough: &'a impl Fn(T, T) -> bool,
    improve: &'a impl Fn(T, T) -> T,
) -> impl Fn(T) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Sub + Sub<Output = T> + Into<f64> + Copy + 'a,
{
    fn is_close_enough<T>(v1: T, v2: T) -> bool
    where
        T: Sub + Sub<Output = T> + Into<f64>,
    {
        (v1 - v2).into().abs() < TOLERANCE
    }
    fn try_it<'a, T>(
        is_good_enough: &'a impl Fn(T, T) -> bool,
        improve: &'a impl Fn(T, T) -> T,
        guess: T,
    ) -> impl Fn(T) -> T + 'a
    where
        T: Sub + Sub<Output = T> + Into<f64> + Copy + 'a,
    {
        move |x| {
            let next = improve(guess, x);
            if is_close_enough(guess, next) {
                next
            } else {
                try_it(is_good_enough, improve, next)(x)
            }
        }
    }
    |guess| Box::new(try_it(is_good_enough, improve, guess))
}
fn sqrt(x: f64) -> f64 {
    iterative_improve(&is_good_enough, &improve)(1.0)(x)
}
fn improve(guess: f64, x: f64) -> f64 {
    average(guess, x / guess)
}
fn average(x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}
fn is_good_enough(guess: f64, x: f64) -> bool {
    (square(guess) - x).abs() < 0.0001
}
fn square(x: f64) -> f64 {
    x * x
}

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

const TOLERANCE: f64 = 0.00001;

fn fixed_point<'a, T>(f: &'a impl Fn(T) -> T, first_guess: T) -> T
where
    T: Sub + Sub<Output = T> + PartialOrd + Copy + Into<f64> + From<f64> + Display,
{
    fn is_close_enough<T>(v1: T, v2: T) -> bool
    where
        T: Sub + Sub<Output = T> + PartialOrd + Into<f64>,
    {
        T::into(v1 - v2).abs() < TOLERANCE
    }
    iterative_improve(&is_close_enough, &|guess, _| f(guess))(first_guess)(f64::MAX.into())
}

fn cos(x: f64) -> f64 {
    x.cos()
}

fn main() {
    let x = 9.0;
    println!("{},{}", sqrt(x), x.sqrt());
    assert!(f64_eq(sqrt(x), x.sqrt()));
    let y = fixed_point(&cos, 1.0);
    println!("{},{}", y, y.cos());
    assert!(f64_eq(y, y.cos()));
}
#[test]
fn test() {
    main();
}
