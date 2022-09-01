use std::time::Instant;

fn smallest_divisor(n: u32) -> u32 {
    find_divisor(n, 2)
}
fn find_divisor(n: u32, test_divisor: u32) -> u32 {
    if square(test_divisor) > n {
        n
    } else if is_divides(test_divisor, n) {
        test_divisor
    } else {
        find_divisor(n, next(test_divisor))
    }
}
fn next(n: u32) -> u32 {
    if n == 2 {
        3
    } else {
        n + 2
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

fn timed_prime_test(n: u32) -> bool {
    print!("\n{}", n);
    start_prime_test(n, runtime())
}
fn start_prime_test(n: u32, runtime: Instant) -> bool {
    if is_prime(n) {
        report_prime(runtime);
        true
    } else {
        false
    }
}
fn report_prime(runtime: Instant) {
    print!(" *** {}", runtime.elapsed().as_nanos());
}

fn runtime() -> Instant {
    Instant::now()
}

fn search_for_prime(start: u32, count: u32) {
    let mut n = start;
    let mut c = 0;
    loop {
        if timed_prime_test(n) {
            c += 1
        }
        if c == count {
            return;
        }
        n += 1
    }
}

fn main() {
    search_for_prime(1_000, 3);
    search_for_prime(10_000, 3);
    search_for_prime(100_000, 3);
    search_for_prime(1_000_000, 3);
    println!();
}
#[test]
fn test() {
    main();
}
