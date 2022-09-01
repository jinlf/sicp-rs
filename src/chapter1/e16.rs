fn fast_expt(b: u32, n: u32) -> u32 {
    fn iter(b: u32, cur: u32, result: u32) -> u32 {
        if cur == 0 {
            result
        } else if is_even(cur) {
            iter(square(b), cur / 2, result)
        } else {
            iter(b, cur - 1, b * result)
        }
    }
    iter(b, n, 1)
}
fn is_even(n: u32) -> bool {
    n % 2 == 0
}
fn square(n: u32) -> u32 {
    n * n
}

fn main() {
    assert_eq!(fast_expt(3, 5), 3u32.pow(5))
}
#[test]
fn test() {
    main();
}
