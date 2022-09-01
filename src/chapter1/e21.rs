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

fn main() {
    println!("{}", smallest_divisor(199));
    println!("{}", smallest_divisor(1999));
    println!("{}", smallest_divisor(19999));
}

#[test]
fn test() {
    main();
}
