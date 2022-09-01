fn f1(n: usize) -> usize {
    if n < 3 {
        n
    } else {
        f1(n - 1) + 2 * f1(n - 2) + 3 * f1(n - 3)
    }
}

fn f2(n: usize) -> usize {
    fn iter(cur: usize, n: usize, f_n_1: usize, f_n_2: usize, f_n_3: usize) -> usize {
        let result = f_n_1 + 2 * f_n_2 + 3 * f_n_3;
        if cur == n {
            result
        } else {
            iter(cur + 1, n, result, f_n_1, f_n_2)
        }
    }
    if n < 3 {
        n
    } else {
        iter(3, n, 2, 1, 0)
    }
}

fn main() {
    for i in 0..=10 {
        assert_eq!(f1(i), f2(i));
    }
}
#[test]
fn test() {
    main();
}
