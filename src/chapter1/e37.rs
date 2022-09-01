fn cont_frac(n: impl Fn(usize) -> f64, d: impl Fn(usize) -> f64, k: usize) -> f64 {
    fn iter(n: impl Fn(usize) -> f64, d: impl Fn(usize) -> f64, k: usize, result: f64) -> f64 {
        if k == 0 {
            result
        } else {
            let result = n(k) / (d(k) + result);
            iter(n, d, k - 1, result)
        }
    }
    iter(n, d, k, 0.0)
}
fn cont_frac1(n: impl Fn(usize) -> f64, d: impl Fn(usize) -> f64, k: usize) -> f64 {
    fn iter(n: impl Fn(usize) -> f64, d: impl Fn(usize) -> f64, k: usize, cur: usize) -> f64 {
        if cur > k {
            0.0
        } else {
            let n_cur = n(cur);
            1.0 / (d(cur) / n(cur) + iter(n, d, k, cur + 1) / n_cur)
        }
    }
    iter(n, d, k, 1)
}

fn main() {
    for i in 1..20 {
        println!("{}", cont_frac1(|_| 1.0, |_| 1.0, i));
    }
    println!();
    for i in 1..20 {
        println!("{}", cont_frac(|_| 1.0, |_| 1.0, i));
    }
}
#[test]
fn test() {
    main();
}
