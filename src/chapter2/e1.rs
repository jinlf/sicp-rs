#[derive(Debug)]
struct Rat {
    n: isize,
    d: isize,
}
impl Rat {
    fn new(n: isize, d: isize) -> Self {
        if d < 0 {
            Self { n: -n, d: -d }
        } else {
            Self { n, d }
        }
    }
}

fn main() {
    println!("{:?}", Rat::new(3, 5));
    println!("{:?}", Rat::new(-3, 5));
    println!("{:?}", Rat::new(3, -5));
    println!("{:?}", Rat::new(-3, -5));
}
#[test]
fn test() {
    main();
}
