fn cons(a: usize, b: usize) -> usize {
    if a == 0 {
        if b == 0 {
            1
        } else {
            3 * cons(0, b - 1)
        }
    } else {
        2 * cons(a - 1, b)
    }
}

fn car(n: usize) -> usize {
    if n % 2 != 0 {
        0
    } else {
        car(n / 2) + 1
    }
}
fn cdr(n: usize) -> usize {
    if n % 3 != 0 {
        0
    } else {
        cdr(n / 3) + 1
    }
}

fn main() {
    let n = cons(3, 5);
    assert_eq!(car(n), 3);
    assert_eq!(cdr(n), 5);
}
#[test]
fn test() {
    main();
}
