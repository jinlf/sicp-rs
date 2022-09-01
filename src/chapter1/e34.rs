fn f<A, B>(g: impl Fn(A) -> B) -> B
where
    A: From<usize>,
{
    g(2.into())
}
fn square(n: usize) -> usize {
    n * n
}

fn main() {
    assert_eq!(f(square), 4);
    assert_eq!(f(|z: usize| z * (z + 1)), 6);
    // f(f); 类型无法通过
}
#[test]
fn test() {
    main();
}
