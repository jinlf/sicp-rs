use dyn_clone::DynClone;
use std::fmt::Debug;

trait Painter: Debug + DynClone {}

#[derive(Clone, Debug)]
struct MyPainter {}
impl Painter for MyPainter {}

fn up_split<A>(painter: &A, n: usize) -> Box<dyn Painter>
where
    A: Painter + 'static,
{
    if n == 0 {
        dyn_clone::clone_box(painter)
    } else {
        let smaller = &*up_split(painter, n - 1);
        let up = &*beside(smaller, smaller);
        below(painter, up)
    }
}
fn beside<A, B>(painter1: &A, painter2: &B) -> Box<dyn Painter>
where
    A: Painter + ?Sized,
    B: Painter + ?Sized,
{
    Box::new(MyPainter {})
}
fn below<A, B>(painter1: &A, painter2: &B) -> Box<dyn Painter>
where
    A: Painter + ?Sized,
    B: Painter + ?Sized,
{
    Box::new(MyPainter {})
}

fn main() {
    let p = MyPainter {};
    println!("{:?}", up_split(&p, 3));
}
#[test]
fn test() {
    main();
}
