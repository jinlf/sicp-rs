fn cube(x: f64) -> f64 {
    x * x * x
}
fn p(x: (f64, usize)) -> (f64, usize) {
    (3.0 * x.0 - 4.0 * cube(x.0), x.1 + 1)
}
fn sine(angle: f64) -> (f64, usize) {
    if !(angle.abs() > 0.1) {
        (angle, 0)
    } else {
        p(sine(angle / 3.0))
    }
}

fn main() {
    /*
    12.15 -> 1
    12.15/3.0 = 4.05 -> 1
    4.05/3.0 = 1.35 -> 1
    1.35/3.0 = 0.45 -> 1
    0.45/3.0 = 0.15 -> 1
    0.15/3.0 = 0.05 -> 0
    共5次
     */
    assert_eq!(sine(12.15).1, 5);
    /*
    空间 n 步数 n
     */
}
#[test]
fn test() {
    main();
}
