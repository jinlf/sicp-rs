use std::ops::Mul;

fn product<C, B>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    C: PartialOrd + Copy,
    B: From<i32> + Mul + Mul<Output = B>,
{
    if a > b {
        B::from(1)
    } else {
        term(a) * product(term, next(a), next, b)
    }
}
fn factorial(n: i32) -> i32 {
    product(|x| x, 1, |x| x + 1, n)
}
fn pi() -> f64 {
    product(
        |i: usize| {
            let a = ((i / 2 + 1) * 2) as f64;
            let b = ((i + 1) / 2 * 2 + 1) as f64;
            a / b
        },
        1,
        |x: usize| x + 1,
        10000,
    ) * 4.0
}
fn product1<C, B>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    C: PartialOrd + Copy,
    B: From<i32> + Mul + Mul<Output = B>,
{
    fn iter<C, B>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C, result: B) -> B
    where
        C: PartialOrd + Copy,
        B: From<i32> + Mul + Mul<Output = B>,
    {
        if a > b {
            result
        } else {
            let result = term(a) * result;
            iter(term, next(a), next, b, result)
        }
    }
    iter(term, a, next, b, 1.into())
}
fn factorial1(n: i32) -> i32 {
    product1(|x| x, 1, |x| x + 1, n)
}
fn pi1() -> f64 {
    product1(
        |i: usize| {
            let a = ((i / 2 + 1) * 2) as f64;
            let b = ((i + 1) / 2 * 2 + 1) as f64;
            a / b
        },
        1,
        |x: usize| x + 1,
        10000,
    ) * 4.0
}

fn main() {
    for i in 1..10 {
        println!("{}", factorial(i))
    }
    println!("{}", pi());
    for i in 1..10 {
        println!("{}", factorial1(i))
    }
    println!("{}", pi1());
}
#[test]
fn test() {
    main();
}
