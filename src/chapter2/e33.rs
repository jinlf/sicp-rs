
fn accumulate<A, B>(op: &impl Fn(A, B) -> B, initial: B, sequence: &[A]) -> B
where
    A: Clone,
{
    if sequence.is_empty() {
        initial
    } else {
        op(sequence[0].clone(), accumulate(op, initial, &sequence[1..]))
    }
}

fn map<A, B>(p: &impl Fn(A) -> B, sequence: &[A]) -> Vec<B>
where
    A: Clone,
{
    accumulate::<A, Vec<B>>(
        &|x, mut acc| {
            acc.insert(0, p(x));
            acc
        },
        vec![],
        sequence,
    )
}
fn append<A>(seq1: &[A], seq2: &[A]) -> Vec<A>
where
    A: Clone,
{
    accumulate(
        &|x, mut acc: Vec<A>| {
            let mut list = vec![x];
            list.append(&mut acc);
            list
        },
        seq2.to_vec(),
        seq1,
    )
}
fn length<A>(sequence: &[A]) -> usize
where
    A: Clone,
{
    accumulate(&|x, acc| acc + 1, 0_usize, sequence)
}

fn map1<A, B>(p: &impl Fn(&A) -> B, sequence: &[A]) -> Vec<B>
where
    A: Clone,
{
    sequence.iter().rfold(vec![], |mut acc, x| {
        acc.insert(0, p(x));
        acc
    })
}

fn append1<A>(seq1: &[A], seq2: &[A]) -> Vec<A>
where
    A: Clone,
{
    seq1.iter().rfold(seq2.to_vec(), |mut acc, x| {
        acc.push(x.clone());
        acc
    })
}
fn length1<A>(sequence: &[A]) -> usize
where
    A: Clone,
{
    sequence.iter().rfold(0, |acc, x| acc + 1)
}

fn main() {
    println!("{:?}", map(&|x| 2 * x, &[1, 2, 3]));
    let v1 = &[1, 2, 3];
    let v2 = &[4, 5, 6];
    println!("{:?}", append(v1, v2));
    println!("{}", length(&[1, 2, 3, 4, 5]));

    println!("{:?}", map1(&|x| 2 * x, &[1, 2, 3]));
    let v1 = &[1, 2, 3];
    let v2 = &[4, 5, 6];
    println!("{:?}", append1(v1, v2));
    println!("{}", length1(&[1, 2, 3, 4, 5]));
}
#[test]
fn test() {
    main();
}
