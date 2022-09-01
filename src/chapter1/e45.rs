use std::ops::Sub;

fn n_root(x: f64, n: usize) -> f64 {
    fixed_point(
        repeated(average_damp, n - 2)(Box::new(move |y: f64| x / y.powf((n - 1) as f64))),
        1.0,
    )
}

fn average_damp(f: impl Fn(f64) -> f64 + 'static) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| average(x, f(x)))
}
fn average(x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}

fn compose<B, C, D>(
    f: impl Fn(D) -> B + 'static,
    g: impl Fn(C) -> D + 'static,
) -> Box<dyn Fn(C) -> B> {
    Box::new(move |x| f(g(x)))
}

fn repeated<A: 'static>(f: impl Fn(A) -> A + Copy + 'static, n: usize) -> Box<dyn Fn(A) -> A> {
    fn iter<A: 'static>(
        f: impl Fn(A) -> A + Copy + 'static,
        n: usize,
        result: impl Fn(A) -> A + 'static,
    ) -> Box<dyn Fn(A) -> A> {
        if n == 1 {
            Box::new(result)
        } else {
            iter(f, n - 1, compose(f, result))
        }
    }
    iter(f, n, f)
}

const TOLERANCE: f64 = 0.0000001;

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

fn f64_eq(a: f64, b: f64) -> bool {
    if a.abs() < 0.00001 && b.abs() < 0.00001 {
        true
    } else {
        ((a - b) / a).abs() < 0.00001
    }
}

fn main() {
    let x = 10_f64.powf(10_f64);
    assert!(f64_eq(n_root(x, 10), 10.0));

    let x = 5_f64.powf(5_f64);
    assert!(f64_eq(n_root(x, 5), 5.0));
}
#[test]
fn test() {
    main();
}
