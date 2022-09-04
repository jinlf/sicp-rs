fn accumulate_n<A, B>(op: &impl Fn(A, B) -> B, init: B, seqs: &Vec<Vec<A>>) -> Vec<B>
where
    A: Copy,
    B: Copy,
{
    if seqs.is_empty() || seqs[0].is_empty() {
        vec![]
    } else {
        let first: B = seqs
            .iter()
            .map(|seq| seq[0])
            .rfold(init, |acc, x| op(x, acc));
        let mut rest = accumulate_n(
            op,
            init,
            &seqs.iter().map(|seq| seq[1..].to_vec()).collect(),
        );
        rest.insert(0, first);
        rest
    }
}

fn main() {
    let s = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];
    println!("{:#?}", accumulate_n(&|x, acc| acc + x, 0, &s));
}
#[test]
fn test() {
    main();
}
