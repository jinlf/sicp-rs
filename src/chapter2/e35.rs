type Tree = Vec<N>;

#[derive(Debug, Clone)]
enum N {
    V(usize),
    T(Tree),
}

fn count_leaves(x: &Tree) -> usize {
    x.iter()
        .map(|x| match x {
            N::V(_) => 1,
            N::T(tree) => count_leaves(tree),
        })
        .rfold(0, |acc, x| acc + x)
}

fn main() {
    let x = vec![N::T(vec![N::V(1), N::V(2)]), N::T(vec![N::V(3), N::V(4)])];
    assert_eq!(count_leaves(&x), 4);
    assert_eq!(count_leaves(&vec![N::T(x.clone()), N::T(x)]), 8);
}
#[test]
fn test() {
    main();
}
