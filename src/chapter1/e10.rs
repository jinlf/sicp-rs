fn a(x: u32, y: u32) -> u32 {
    if y == 0 {
        0
    } else if x == 0 {
        2 * y
    } else if y == 1 {
        2
    } else {
        a(x - 1, a(x, y - 1))
    }
}

fn f(n: u32) -> u32 {
    a(0, n)
}
fn g(n: u32) -> u32 {
    a(1, n)
}
fn h(n: u32) -> u32 {
    a(2, n)
}
fn k(n: u32) -> u32 {
    5 * n * n
}

/*
a(1, x) = 2^x
a(2, 1) = 2
a(2, 2) = a(1, a(2, 1))
        = a(1, 2)
        = 2^2
a(2, 3) = a(1, a(2, 2))
        = 2^(2^2)
a(2, 4) = a(1, a(2, 3))
        = 2^(2^(2^2))
a(2, x) = a(1, a(2, x-1))
        = a(1, 2^(x-1))
        = 2^(2^(...(2^2)))      n个
a(3, 1) = 2
a(3, 2) = a(2, a(3, 1))
        = a(2, 2)
        = 2^2
a(3, 3) = a(2, a(3, 2))
        = 2^(...(2^2))   2^2个
a(3, x) = a(2, a(3, x-1))
        = 2^(...(2^2))          2^(x-1)个
 */
fn main() {
    assert_eq!(a(1, 10), 2u32.pow(10));
    assert_eq!(a(2, 4), 2u32.pow(2u32.pow(2u32.pow(2u32))));
    assert_eq!(a(3, 3), 2u32.pow(2u32.pow(2u32.pow(2u32))));

    assert_eq!(f(10), 2 * 10);
    assert_eq!(g(10), 2u32.pow(10));
    assert_eq!(h(4), 2u32.pow(2u32.pow(2u32.pow(2u32))));
    assert_eq!(k(3), 5 * 3 * 3);
}
#[test]
fn test() {
    main();
}
