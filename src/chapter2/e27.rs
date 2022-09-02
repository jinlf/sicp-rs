type List = Vec<N>;

#[derive(Clone, Debug, PartialEq)]
enum N {
    V(usize),
    L(List),
}

fn reverse(list: &List) -> List {
    if list.is_empty() {
        vec![]
    } else {
        let first = list[0].clone();
        let mut rest = reverse(&list[1..].to_vec());
        rest.push(first);
        rest
    }
}
fn deep_reverse(list: &List) -> List {
    if list.is_empty() {
        vec![]
    } else {
        let first = match &list[0] {
            N::V(v) => N::V(*v),
            N::L(list) => N::L(deep_reverse(list)),
        };
        let mut rest = deep_reverse(&list[1..].to_vec());
        rest.push(first);
        rest
    }
}

fn main() {
    let x = vec![N::L(vec![N::V(1), N::V(2)]), N::L(vec![N::V(3), N::V(4)])];
    println!("{:?}", x);
    println!("{:?}", reverse(&x));
    println!("{:?}", deep_reverse(&x));
}
#[test]
fn test() {
    main()
}
