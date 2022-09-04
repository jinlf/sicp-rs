use dyn_clone::DynClone;
use std::fmt::Debug;

trait Painter: Debug + DynClone {}

#[derive(Clone, Debug)]
struct MyPainter {}
impl Painter for MyPainter {}

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

fn split<'b, 'd: 'b, 'h>(
    f: &'h impl Fn(&(dyn Painter + 'b), &(dyn Painter + 'b)) -> Box<dyn Painter + 'd>,
    g: &'h impl Fn(&(dyn Painter + 'b), &(dyn Painter + 'b)) -> Box<dyn Painter + 'd>,
) -> impl Fn(&(dyn Painter + 'b), usize) -> Box<dyn Painter + 'b> + 'h {
    move |painter, n| {
        if n == 0 {
            dyn_clone::clone_box(painter)
        } else {
            let smaller = &*split(f, g)(painter, n - 1);
            f(painter, &*g(smaller, smaller))
        }
    }
}

fn right_split(painter: &(dyn Painter), n: usize) -> Box<dyn Painter + '_> {
    split(&beside, &below)(painter, n)
}
fn up_split(painter: &(dyn Painter), n: usize) -> Box<dyn Painter + '_> {
    split(&below, &beside)(painter, n)
}

fn main() {
    let p = MyPainter {};
    let p1 = up_split(&p, 3);
    let p2 = right_split(&p, 5);
    println!("{:?}", p1);
    println!("{:?}", p2);
}
#[test]
fn test() {
    main();
}
