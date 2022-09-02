fn same_parity(first: usize, rest: &[usize]) -> Vec<usize> {
    fn same_parity_of(first: usize, rest: &[usize]) -> Vec<usize> {
        if rest.len() == 0 {
            vec![]
        } else {
            let mut list = same_parity_of(first, &rest[1..]);
            if rest[0] % 2 == first % 2 {
                list.insert(0, rest[0]);
            }
            list
        }
    }
    let mut result = same_parity_of(first, rest);
    result.insert(0, first);
    result
}
fn same_parity1(first: usize, rest: &[usize]) -> Vec<usize> {
    [first]
        .iter()
        .chain(rest.iter())
        .filter(|&x| x % 2 == first % 2)
        .cloned()
        .collect()
}

fn main() {
    let list = &[2, 3, 4, 5, 6, 7];
    assert_eq!(same_parity(1, list), same_parity1(1, list));
}
#[test]
fn test() {
    main();
}
