use rand::Rng;
use std::time::Instant;

fn square(n: usize) -> usize {
    n * n
}
fn remainder(a: usize, b: usize) -> usize {
    a % b
}

fn is_fast_prime(n: usize, times: usize) -> bool {
    if times == 0 {
        true
    } else if fermat_test(n) {
        is_fast_prime(n, times - 1)
    } else {
        false
    }
}
fn fermat_test(n: usize) -> bool {
    let try_it = |a, n| expmod(a, n, n) == a;
    try_it(rand::thread_rng().gen::<usize>() % (n - 1) + 1, n)
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

fn timed_prime_test(n: usize) -> bool {
    print!("\n{}", n);
    start_prime_test(n, runtime())
}
fn start_prime_test(n: usize, runtime: Instant) -> bool {
    if is_fast_prime(n, 1) {
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

fn search_for_prime(start: usize, count: usize) {
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
