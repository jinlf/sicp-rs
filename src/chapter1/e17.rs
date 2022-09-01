fn mul(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else if is_even(b) {
        mul(double(a), halve(b))
    } else {
        a + mul(a, b - 1)
    }
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
    assert_eq!(mul(3, 7), 3 * 7);
}
#[test]
fn test() {
    main();
}
