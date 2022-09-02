type List = Vec<N>;

#[derive(Clone, Debug, PartialEq)]
enum N {
    V(usize),
    L(List),
}

fn car(list: List) -> List {
    if list.len() == 0 {
        vec![]
    } else {
        match &list[0] {
            N::V(v) => vec![N::V(*v)],
            N::L(list) => list.clone(),
        }
    }
}
fn cdr(list: List) -> List {
    if list.len() < 2 {
        todo!()
    } else if list.len() == 2 {
        match &list[1] {
            N::V(v) => vec![N::V(*v)],
            N::L(list) => list.clone(),
        }
    } else {
        list[1..].to_vec()
    }
}

fn main() {
    let list1 = vec![N::V(1), N::V(3), N::L(vec![N::V(5), N::V(7)]), N::V(9)];
    let list2 = vec![N::L(vec![N::V(7)])];
    let list3 = vec![
        N::V(1),
        N::L(vec![
            N::V(2),
            N::L(vec![
                N::V(3),
                N::L(vec![
                    N::V(4),
                    N::L(vec![N::V(5), N::L(vec![N::V(6), N::V(7)])]),
                ]),
            ]),
        ]),
    ];

    assert_eq!(cdr(car(cdr(cdr(list1)))), vec![N::V(7)]);
    assert_eq!(car(car(list2)), vec![N::V(7)]);
    assert_eq!(cdr(cdr(cdr(cdr(cdr(cdr(list3)))))), vec![N::V(7)]);
}
#[test]
fn test() {
    main();
}
