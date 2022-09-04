fn unique_pairs(n: u32) -> Vec<(u32, u32)> {
    (1..=n)
        .collect::<Vec<u32>>()
        .iter()
        .flat_map(|&i| {
            (1..i)
                .collect::<Vec<u32>>()
                .iter()
                .map(|&j| (i, j))
                .collect::<Vec<(u32, u32)>>()
        })
        .collect()
}
fn prime_sum_pairs(n: u32) -> Vec<(u32, u32, u32)> {
    unique_pairs(n)
        .into_iter()
        .filter(|&(first, second)| is_prime(first + second))
        .map(|(first, second)| (first, second, first + second))
        .collect()
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

fn main() {
    println!("{:#?}", prime_sum_pairs(8));
}
#[test]
fn test() {
    main();
}
