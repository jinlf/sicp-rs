fn last_pair<T>(list: &[T]) -> Vec<T>
where
    T: Clone,
{
    if list.len() == 1 {
        list.to_vec()
    } else {
        last_pair(&list[1..])
    }
}

fn main() {
    let list = &[23, 72, 149, 34];
    assert_eq!(last_pair(list), vec![*list.iter().last().unwrap()]);
}
#[test]
fn test() {
    main();
}
