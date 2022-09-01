use std::ops::{Add, Sub};

fn add1<T>(a: T, b: T) -> T
where
    T: PartialEq + Add + Add<Output = T> + Sub + Sub<Output = T> + From<isize>,
{
    if a == 0.into() {
        b
    } else {
        inc(add1(dec(a), b))
    }
}
fn add2<T>(a: T, b: T) -> T
where
    T: PartialEq + Add + Add<Output = T> + Sub + Sub<Output = T> + From<isize>,
{
    if a == 0.into() {
        b
    } else {
        add2(dec(a), inc(b))
    }
}

fn inc<T>(a: T) -> T
where
    T: Add + Add<Output = T> + From<isize>,
{
    a + 1.into()
}
fn dec<T>(a: T) -> T
where
    T: Sub + Sub<Output = T> + From<isize>,
{
    a - 1.into()
}

fn main() {
    assert_eq!(add1(4, 5), 9);
    assert_eq!(add2(4, 5), 9);
}
#[test]
fn test() {
    main();
}
