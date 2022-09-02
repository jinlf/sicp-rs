type Tree = Vec<N>;

#[derive(Clone, Debug, PartialEq)]
enum N {
    V(usize),
    T(Tree),
}

fn fringe(t: &Tree) -> Vec<usize> {
    if t.is_empty() {
        vec![]
    } else {
        let mut first = match &t[0] {
            N::V(v) => vec![*v],
            N::T(tree) => fringe(tree),
        };
        let mut rest = fringe(&t[1..].to_vec());
        first.append(&mut rest);
        first
    }
}

fn main() {
    let x = vec![N::T(vec![N::V(1), N::V(2)]), N::T(vec![N::V(3), N::V(4)])];
    println!("{:#?}", x);
    println!("{:?}", fringe(&x));
    let y = vec![N::T(x.clone()), N::T(x.clone())];
    println!("{:?}", fringe(&y));
}
#[test]
fn test() {
    main();
}
