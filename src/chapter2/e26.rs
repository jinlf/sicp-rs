type List = Vec<N>;

#[derive(Clone, Debug, PartialEq)]
enum N {
    V(usize),
    L(List),
}

fn append(list1: List, list2: List) -> List {
    if list1.is_empty() {
        list2
    } else {
        let mut first = vec![list1[0].clone()];
        let mut rest = append(list1[1..].to_vec(), list2);
        first.append(&mut rest);
        first
    }
}

fn main() {
    let x = vec![N::V(1), N::V(2), N::V(3)];
    let y = vec![N::V(4), N::V(5), N::V(6)];
    println!("{:?}", append(x.clone(), y.clone()));
    println!("{:?}", (x.clone(), y.clone()));
    println!("{:?}", vec![N::L(x), N::L(y)]);
}
#[test]
fn test() {
    main();
}
