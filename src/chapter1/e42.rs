fn compose<B, C, D>(f: impl Fn(D) -> B, g: impl Fn(C) -> D) -> impl Fn(C) -> B {
    move |x| f(g(x))
}
fn square(n: usize) -> usize {
    n * n
}
fn inc(n: usize) -> usize {
    n + 1
}

fn main() {
    assert_eq!(compose(square, inc)(6), 49);
}
#[test]
fn test() {
    main();
}
