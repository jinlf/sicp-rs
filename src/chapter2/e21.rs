fn square_list(list: &[usize]) -> Vec<usize> {
    if list.len() == 0 {
        vec![]
    } else {
        let mut first = vec![list[0] * list[0]];
        let mut rest = square_list(&list[1..]);
        first.append(&mut rest);
        first
    }
}

fn square_list1(list: &[usize]) -> Vec<usize> {
    list.iter().map(|x| x * x).collect()
}

fn main() {
    let list = &[1, 2, 3, 4];
    assert_eq!(square_list(list), square_list1(list));
}
#[test]
fn test() {
    main();
}
