fn square(n: usize) -> usize {
    n * n
}
fn remainder(a: usize, b: usize) -> usize {
    a % b
}

fn expmod(base: usize, exp: usize, m: usize) -> usize {
    if exp == 0 {
        1
    } else if is_even(exp) {
        remainder(square(expmod(base, exp / 2, m)), m)
    } else {
        remainder(base * expmod(base, exp - 1, m), m)
    }
}
fn is_even(n: usize) -> bool {
    n % 2 == 0
}

fn full_test(n: usize) -> bool {
    for a in 1..n {
        if expmod(a, n, n) != a {
            return false;
        }
    }
    true
}

fn main() {
    let carnichaels = &[561, 1105, 1729, 2465, 2821, 6601];
    for c in carnichaels {
        assert!(full_test(*c));
    }
}
#[test]
fn test() {
    main();
}
