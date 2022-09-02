fn cons<T>(x: T, y: T) -> impl Fn(Box<dyn Fn(T, T) -> T>) -> T
where
    T: Copy,
{
    move |m| m(x, y)
}
fn car<T>(z: impl Fn(Box<dyn Fn(T, T) -> T>) -> T) -> T {
    z(Box::new(|p, _| p))
}
fn cdr<T>(z: impl Fn(Box<dyn Fn(T, T) -> T>) -> T) -> T {
    z(Box::new(|_, q| q))
}

fn main() {
    let pair = &cons(3, 5);
    assert_eq!(car(pair), 3);
    assert_eq!(cdr(pair), 5);
}
#[test]
fn test() {
    main();
}
