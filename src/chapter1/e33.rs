fn filtered_accumulate<B, C>(
    filter: impl Fn(B) -> bool,
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
        filter: impl Fn(B) -> bool,
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
            let v = term(a);
            let result = combiner(result, if filter(v) { v } else { null_value });
            iter(filter, combiner, null_value, term, next(a), next, b, result)
        }
    }
    iter(filter, combiner, null_value, term, a, next, b, null_value)
}

fn sum_of_prime(a: u32, b: u32) -> u32 {
    filtered_accumulate(is_prime, |x, y| x + y, 0, |x| x, a, |x| x + 1, b)
}
fn smallest_divisor(n: u32) -> u32 {
    find_divisor(n, 2)
}
fn find_divisor(n: u32, test_divisor: u32) -> u32 {
    if square(test_divisor) > n {
        n
    } else if is_divides(test_divisor, n) {
        test_divisor
    } else {
        find_divisor(n, test_divisor + 1)
    }
}
fn square(n: u32) -> u32 {
    n * n
}
fn is_divides(a: u32, b: u32) -> bool {
    remainder(b, a) == 0
}
fn remainder(a: u32, b: u32) -> u32 {
    a % b
}
fn is_prime(n: u32) -> bool {
    smallest_divisor(n) == n
}

fn product_of_mut_prime(n: u32) -> u32 {
    filtered_accumulate(
        |i| gcd(i, n) == 1,
        |acc, x| acc * x,
        1,
        |x| x,
        1,
        |x| x + 1,
        n - 1,
    )
}
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, remainder(a, b))
    }
}

fn main() {
    assert_eq!(sum_of_prime(2, 10), 2 + 3 + 5 + 7);
    assert_eq!(product_of_mut_prime(10), 3 * 7 * 9);
}
#[test]
fn test() {
    main();
}
