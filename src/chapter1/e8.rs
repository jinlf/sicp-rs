use std::ops::Mul;

fn is_good_enough(guess: f64, new_guess: f64) -> bool {
    ((new_guess - guess) / guess).abs() < 0.00001
}
fn cube_root_iter(guess: f64, x: f64) -> f64 {
    let new_guess = improve(guess, x);
    if is_good_enough(guess, new_guess) {
        guess
    } else {
        cube_root_iter(new_guess, x)
    }
}
fn improve(guess: f64, x: f64) -> f64 {
    (x / square(guess) + 2.0 * guess) / 3.0
}
fn square<T>(x: T) -> T
where
    T: Copy + Mul + Mul<Output = T>,
{
    x * x
}
fn cube_root(x: f64) -> f64 {
    cube_root_iter(1.0, x)
}
fn f64_eq(x: f64, y: f64) -> bool {
    ((x - y) / y).abs() < 0.00001
}

fn main() {
    assert!(f64_eq(cube_root(27.0), 27_f64.powf(1.0 / 3.0)));
    assert!(f64_eq(cube_root(f64::MAX), f64::MAX.powf(1.0 / 3.0)));
    assert!(f64_eq(
        cube_root(f64::MIN_POSITIVE),
        f64::MIN_POSITIVE.powf(1.0 / 3.0)
    ));
}
#[test]
fn test() {
    main();
}
