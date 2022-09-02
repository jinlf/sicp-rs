fn zero<F, X>() -> impl Fn(F) -> Box<dyn Fn(X) -> X> {
    |f| Box::new(|x| x)
}

fn add_1<'a, F, X>(
    n: &'a impl Fn(&F) -> Box<dyn Fn(X) -> X>,
) -> impl Fn(F) -> Box<dyn Fn(X) -> X + 'a>
where
    F: Fn(X) -> X + 'a,
{
    move |f| Box::new(move |x| f(n(&f)(x)))
}

fn add<'a, F, X>(
    a: &'a impl Fn(&F) -> Box<dyn Fn(X) -> X>,
    b: &'a impl Fn(&F) -> Box<dyn Fn(X) -> X>,
) -> impl Fn(F) -> Box<dyn Fn(X) -> X + 'a>
where
    F: Fn(X) -> X + 'a,
{
    move |f| Box::new(move |x| a(&f)(b(&f)(x)))
}

fn one<F, X>() -> impl Fn(F) -> Box<dyn Fn(X) -> X>
where
    F: Fn(X) -> X + 'static,
{
    |f| Box::new(move |x| f(x))
}

fn two<F, X>() -> impl Fn(F) -> Box<dyn Fn(X) -> X>
where
    F: Fn(X) -> X + 'static,
{
    |f| Box::new(move |x| f(f(x)))
}

fn main() {
    /*
    (define one (lambda (f) (lambda (x) f(x))))

    (define two (lambda (f) (lambda (x) f(f(x)))))

    (define (add a b) (lambda (f) (lambda (x) a(f)(b(f)(x))))
    */
}
