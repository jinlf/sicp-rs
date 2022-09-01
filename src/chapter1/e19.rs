fn fib(n: u32) -> u32 {
    fib_iter(1, 0, 0, 1, n)
}
/*
应用T变换n次 从Fib(1),Fib(0)生成Fib(n+1),F(n)，应用n+1次，生成Fib(n+2),F(n+1)
以此类推应用2n次，即应用Tn两次，生成Fib(2n+1),Fib(2n)

a1 = bq+aq+ap
b1 = bp+aq
带入
a2  = b1q+a1q+a1p
    = (bp+aq)q+(bq+aq+ap)q+(bq+aq+ap)p
    = bpq+aqq+bqq+aqq+apq+bpq+apq+app
    = b(2pq+qq)+a(2qq+2pq+pp)
    = b(2pq+qq)+a(pp+qq)+a(2pq+qq)
b2  = b1p+a1q
    = (bp+aq)p+(bq+aq+ap)q
    = bpp+apq+bqq+aqq+apq
    = b(pp+qq)+a(2pq+qq)

p'  = pp+qq
q'  = 2pq+qq
 */
fn fib_iter(a: u32, b: u32, p: u32, q: u32, count: u32) -> u32 {
    if count == 0 {
        b
    } else if is_even(count) {
        let p1 = p * p + q * q;
        let q1 = 2 * p * q + q * q;
        fib_iter(a, b, p1, q1, count / 2)
    } else {
        fib_iter(b * q + a * q + a * p, b * p + a * q, p, q, count - 1)
    }
}
fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn origin_fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        origin_fib(n - 1) + origin_fib(n - 2)
    }
}

fn main() {
    for i in 0..=10 {
        assert_eq!(fib(i), origin_fib(i));
    }
}
#[test]
fn test() {
    main();
}
