type Tree = Vec<N>;

#[derive(Clone, Debug)]
enum N {
    V(usize),
    T(Tree),
}

fn square_tree(t: &Tree) -> Tree {
    if t.is_empty() {
        vec![]
    } else {
        let first = match &t[0] {
            N::V(v) => N::V(square(*v)),
            N::T(tree) => N::T(square_tree(tree)),
        };
        let mut rest = square_tree(&t[1..].to_vec());
        rest.insert(0, first);
        rest
    }
}
fn square(x: usize) -> usize {
    x * x
}

fn square_tree1(t: &Tree) -> Tree {
    t.iter()
        .map(|x| match x {
            N::V(v) => N::V(square(*v)),
            N::T(tree) => N::T(square_tree1(tree)),
        })
        .collect()
}

fn main() {
    let tree = vec![
        N::V(1),
        N::T(vec![N::V(2), N::T(vec![N::V(3), N::V(4)]), N::V(5)]),
        N::T(vec![N::V(6), N::V(7)]),
    ];
    println!("{:#?}", square_tree(&tree));
    println!("{:#?}", square_tree1(&tree));
}
#[test]
fn test() {
    main();
}
