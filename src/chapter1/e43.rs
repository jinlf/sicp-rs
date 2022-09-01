fn compose<B, C, D>(
    f: impl Fn(D) -> B + 'static,
    g: impl Fn(C) -> D + 'static,
) -> Box<dyn Fn(C) -> B> {
    Box::new(move |x| f(g(x)))
}

fn repeated<A: 'static>(f: impl Fn(A) -> A + Copy + 'static, n: usize) -> Box<dyn Fn(A) -> A> {
    fn iter<A: 'static>(
        f: impl Fn(A) -> A + Copy + 'static,
        n: usize,
        result: impl Fn(A) -> A + 'static,
    ) -> Box<dyn Fn(A) -> A> {
        if n == 1 {
            Box::new(result)
        } else {
            iter(f, n - 1, compose(f, result))
        }
    }
    iter(f, n, f)
}

fn square(n: usize) -> usize {
    n * n
}

fn main() {
    assert_eq!(repeated(square, 2)(5), 625);
}
#[test]
fn test() {
    main();
}
