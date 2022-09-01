fn fast_mul(a: u32, b: u32) -> u32 {
    fn iter(a: u32, cur: u32, result: u32) -> u32 {
        if cur == 0 {
            result
        } else if is_even(cur) {
            iter(double(a), halve(cur), result)
        } else {
            iter(a, cur - 1, a + result)
        }
    }
    iter(a, b, 0)
}
fn is_even(n: u32) -> bool {
    n % 2 == 0
}
fn double(n: u32) -> u32 {
    2 * n
}
fn halve(n: u32) -> u32 {
    n / 2
}

fn main() {
    assert_eq!(fast_mul(3, 7), 3 * 7);
}
#[test]
fn test() {
    main();
}
