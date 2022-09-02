type Tree = Vec<N>;

#[derive(Clone, Debug)]
enum N {
    V(usize),
    T(Tree),
}

fn tree_map(p: &impl Fn(usize) -> usize, t: &Tree) -> Tree {
    if t.is_empty() {
        vec![]
    } else {
        let first = match &t[0] {
            N::V(v) => N::V(p(*v)),
            N::T(tree) => N::T(tree_map(p, tree)),
        };
        let mut rest = tree_map(p, &t[1..].to_vec());
        rest.insert(0, first);
        rest
    }
}

fn square_tree(t: &Tree) -> Tree {
    tree_map(&square, t)
}
fn square(x: usize) -> usize {
    x * x
}

fn main() {
    let tree = vec![
        N::V(1),
        N::T(vec![N::V(2), N::T(vec![N::V(3), N::V(4)]), N::V(5)]),
        N::T(vec![N::V(6), N::V(7)]),
    ];
    println!("{:#?}", square_tree(&tree));
}
#[test]
fn test() {
    main();
}
