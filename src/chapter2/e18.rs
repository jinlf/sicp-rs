fn reverse<T>(mut list: Vec<T>) -> Vec<T> {
    if list.len() == 0 {
        vec![]
    } else {
        let v = list.remove(0);
        let mut list = reverse(list);
        list.push(v);
        list
    }
}
fn main() {
    let list = vec![1, 4, 9, 16, 25];
    assert_eq!(
        reverse(list.clone()),
        list.into_iter().rev().collect::<Vec<i32>>()
    );
}
#[test]
fn test() {
    main();
}
