use std::ops::{Add, Mul};

fn sum<B, C>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    B: From<i32> + Add + Add<Output = B>,
    C: PartialOrd + Copy,
{
    accumulate(&|acc, x| acc + x, B::from(0), term, a, next, b)
}
fn product<B, C>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    B: From<i32> + Mul + Mul<Output = B>,
    C: PartialOrd + Copy,
{
    accumulate(&|acc, x| acc * x, B::from(1), term, a, next, b)
}
fn accumulate<B, C>(
    combiner: &impl Fn(B, B) -> B,
    null_value: B,
    term: impl Fn(C) -> B,
    a: C,
    next: impl Fn(C) -> C,
    b: C,
) -> B
where
    C: PartialOrd + Copy,
{
    if a > b {
        null_value
    } else {
        combiner(
            term(a),
            accumulate(combiner, null_value, term, next(a), next, b),
        )
    }
}
fn sum1<B, C>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    B: From<i32> + Add + Add<Output = B> + Copy,
    C: PartialOrd + Copy,
{
    accumulate1(|acc, x| acc + x, B::from(0), term, a, next, b)
}
fn product1<B, C>(term: impl Fn(C) -> B, a: C, next: impl Fn(C) -> C, b: C) -> B
where
    B: From<i32> + Mul + Mul<Output = B> + Copy,
    C: PartialOrd + Copy,
{
    accumulate1(|acc, x| acc * x, B::from(1), term, a, next, b)
}
fn accumulate1<B, C>(
    combiner: impl Fn(B, B) -> B,
    null_value: B,
    term: impl Fn(C) -> B,
    a: C,
    next: impl Fn(C) -> C,
    b: C,
) -> B
where
    C: PartialOrd + Copy,
    B: Copy,
{
    fn iter<B, C>(
        combiner: impl Fn(B, B) -> B,
        null_value: B,
        term: impl Fn(C) -> B,
        a: C,
        next: impl Fn(C) -> C,
        b: C,
        result: B,
    ) -> B
    where
        C: PartialOrd + Copy,
        B: Copy,
    {
        if a > b {
            result
        } else {
            let result = combiner(result, term(a));
            iter(combiner, null_value, term, next(a), next, b, result)
        }
    }
    iter(combiner, null_value, term, a, next, b, null_value)
}

fn main() {
    assert_eq!(sum(|x| x, 1, |x| x + 1, 10), 55);
    assert_eq!(product(|x| x, 1, |x| x + 1, 10), 3628800);
    assert_eq!(sum1(|x| x, 1, |x| x + 1, 10), 55);
    assert_eq!(product1(|x| x, 1, |x| x + 1, 10), 3628800);
}
#[test]
fn test() {
    main();
}
