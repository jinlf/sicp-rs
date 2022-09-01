use std::ops::Add;

fn integral(f: impl Fn(f64) -> f64, a: f64, b: f64, dx: f64) -> f64 {
    sum(f, a + dx / 2.0, |x| x + dx, b) * dx
}

fn integral1(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / (n as f64);
    let y_k = |k: usize| f(a + (k as f64) * h);
    let s = sum(
        |k: usize| {
            if k % 2 == 1 {
                4.0 * y_k(k)
            } else {
                2.0 * y_k(k)
            }
        },
        1,
        |k: usize| k + 1,
        n - 1,
    );
    h / 3.0 * (y_k(0) + s + y_k(n))
}
fn sum<B, C>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    B: Add + Add<Output = B> + From<i32>,
    C: PartialOrd + Copy,
{
    if a > b {
        0_i32.into()
    } else {
        term(a) + sum(term, next(a), next, b)
    }
}
fn cube(n: f64) -> f64 {
    n * n * n
}

fn main() {
    println!("{}", integral(cube, 0.0, 1.0, 0.01));
    println!("{}", integral(cube, 0.0, 1.0, 0.001));
    println!("{}", integral1(cube, 0.0, 1.0, 100));
    println!("{}", integral1(cube, 0.0, 1.0, 1000));
}
#[test]
fn test() {
    main();
}
