use std::ops::Add;

fn max<T>(a: T, b: T, c: T) -> T
where
    T: PartialOrd + Add + Add<Output = T>,
{
    if a > b {
        if b > c {
            a + b
        } else {
            a + c
        }
    } else if a > c {
        b + a
    } else {
        b + c
    }
}

fn main() {
    assert_eq!(max(1, 1, 1), 2);

    assert_eq!(max(1, 1, 2), 3);
    assert_eq!(max(1, 2, 1), 3);
    assert_eq!(max(2, 1, 1), 3);

    assert_eq!(max(1, 2, 2), 4);
    assert_eq!(max(2, 1, 2), 4);
    assert_eq!(max(2, 2, 1), 4);

    assert_eq!(max(1, 2, 3), 5);
    assert_eq!(max(1, 3, 2), 5);
    assert_eq!(max(2, 1, 3), 5);
    assert_eq!(max(2, 3, 1), 5);
    assert_eq!(max(3, 1, 2), 5);
    assert_eq!(max(3, 2, 1), 5);
}

#[test]
fn test() {
    main();
}
