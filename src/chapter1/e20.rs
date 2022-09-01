fn gcd(a: u32, b: u32) -> (u32, usize) {
    if b == 0 {
        (a, 0)
    } else {
        let (x, count) = gcd(b, remainder(a, b));
        (x, count + 1)
    }
}
fn remainder(a: u32, b: u32) -> u32 {
    a % b
}

fn main() {
    /*
    正则序
        有点复杂
     */
    /*
        应用序
       206, 40 => 1
       40, 6   => 1
       6, 4    => 1
       4, 2    => 1
       2, 0    => 0
    */
    assert_eq!(gcd(206, 40).1, 4);
}
#[test]
fn test() {
    main();
}
