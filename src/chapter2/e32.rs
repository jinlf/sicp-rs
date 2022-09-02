fn subsets(s: &[usize]) -> Vec<Vec<usize>> {
    if s.is_empty() {
        vec![vec![]]
    } else {
        let first = s[0];
        let rest = subsets(&s[1..]);
        append(
            &rest,
            &map::<Vec<usize>, Vec<usize>>(
                &|x| {
                    let mut v = x.clone();
                    v.push(first);
                    v
                },
                &rest,
            ),
        )
    }
}
fn append<A>(seq1: &Vec<A>, seq2: &Vec<A>) -> Vec<A>
where
    A: Clone,
{
    if seq1.is_empty() {
        seq2.to_vec()
    } else {
        let first = seq1[0].clone();
        let mut rest = append(&seq1[1..].to_vec(), seq2);
        rest.insert(0, first);
        rest
    }
}

fn map<A, B>(p: &impl Fn(&A) -> B, seq: &Vec<A>) -> Vec<B>
where
    A: Clone,
{
    if seq.is_empty() {
        vec![]
    } else {
        let first = p(&seq[0]);
        let mut rest = map(p, &seq[1..].to_vec());
        rest.insert(0, first);
        rest
    }
}

fn subsets1(s: &[usize]) -> Vec<Vec<usize>> {
    if s.is_empty() {
        vec![vec![]]
    } else {
        let mut rest = subsets(&s[1..]);
        let mut new_rest = rest
            .iter()
            .map(|x| {
                let mut v = x.clone();
                v.push(s[0]);
                v
            })
            .collect();
        rest.append(&mut new_rest);
        rest
    }
}

fn main() {
    let s = &[1, 2, 3];
    println!("{:?}", subsets(s));
    println!("{:?}", subsets1(s));
}
#[test]
fn test() {
    main();
}
