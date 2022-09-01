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

fn main() {
    println!("{}", fixed_point(|x| 1.0 + 1.0 / x, 1.0));
}
#[test]
fn test() {
    main();
}
