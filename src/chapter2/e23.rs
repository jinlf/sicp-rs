fn for_each<X>(p: impl Fn(&X), list: &[X]) {
    if list.len() != 0 {
        p(&list[0]);
        for_each(p, &list[1..])
    }
}
fn for_each1<X>(p: impl Fn(&X), list: &[X]) {
    list.iter().for_each(p);
}

fn main() {
    let list = &[57, 321, 88];
    for_each(|&x| println!("{}", x), list);
    for_each1(|&x| println!("{}", x), list);
}
#[test]
fn test() {
    main();
}
