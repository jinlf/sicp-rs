#![allow(clippy::eq_op)]

fn main() {
    assert_eq!(10, 10);
    assert_eq!(5 + 3 + 4, 12);
    assert_eq!(9 - 1, 8);
    assert_eq!(6 / 2, 3);
    assert_eq!((2 * 4) + (4 - 6), 6);
    let a = 3;
    let b = a + 1;
    assert_eq!(a + b + a * b, 19);
    assert_eq!(a == b, false);
    assert_eq!(if b > a && b < a * b { b } else { a }, 4);
    assert_eq!(
        if a == 4 {
            6
        } else if b == 4 {
            6 + 7 + a
        } else {
            25
        },
        16
    );
    assert_eq!(2 + if b > a { b } else { a }, 6);
    assert_eq!(
        if a > b {
            a
        } else if a < b {
            b
        } else {
            -1
        } + (a + 1),
        8
    );
}

#[test]
fn test() {
    main()
}
