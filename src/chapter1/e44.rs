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

const DX: f64 = 0.00001;

fn smooth(f: impl Fn(f64) -> f64 + 'static) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| (f(x - DX) + f(x) + f(x + DX)) / 3.0)
}

fn smooth_n(f: impl Fn(f64) -> f64 + 'static, n: usize) -> Box<dyn Fn(f64) -> f64> {
    repeated(smooth, n)(Box::new(f))
}

fn square(n: f64) -> f64 {
    n * n
}

fn main() {
    println!("{}", repeated(square, 2)(3.0));
    println!("{}", smooth_n(square, 2)(9.0));
}
#[test]
fn test() {
    main();
}
