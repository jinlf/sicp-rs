fn double<T>(g: impl Fn(T) -> T + 'static) -> Box<dyn Fn(T) -> T> {
    Box::new(move |x| g(g(x)))
}

fn inc(n: usize) -> usize {
    n + 1
}

fn main() {
    println!("{}", double(inc)(5));
    println!(
        "{}",
        double(Box::new(double(Box::new(double))))(Box::new(inc))(5)
    );
}
#[test]
fn test() {
    main();
}
