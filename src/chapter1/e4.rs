use std::ops::{Add, Sub};

fn a_plus_abs_b<T>(a: T, b: T) -> T
where
    T: PartialOrd + From<isize> + Add + Add<Output = T> + Sub + Sub<Output = T>,
{
    let op = if b > 0.into() {
        std::ops::Add::add
    } else {
        std::ops::Sub::sub
    };
    op(a, b)
}

fn main() {
    assert_eq!(a_plus_abs_b(1, 2), 3);
    assert_eq!(a_plus_abs_b(1, -2), 3);
}

#[test]
fn test() {
    main();
}
