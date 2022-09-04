fn fold_left<A, B>(op: &impl Fn(B, &A) -> B, init: B, seq: &[A]) -> B {
    fn iter<A, B>(op: &impl Fn(B, &A) -> B, result: B, rest: &[A]) -> B {
        if rest.is_empty() {
            result
        } else {
            iter(op, op(result, &rest[0]), &rest[1..])
        }
    }
    iter(op, init, seq)
}
fn fold_right<A, B>(op: &impl Fn(B, &A) -> B, init: B, seq: &[A]) -> B {
    if seq.is_empty() {
        init
    } else {
        op(fold_right(op, init, &seq[1..]), &seq[0])
    }
}

fn main() {
    println!("{:?}", &[1.0, 2.0, 3.0].iter().rfold(1.0, |acc, x| acc / x));
    println!("{:?}", fold_right(&|acc, x| acc / x, 1.0, &[1.0, 2.0, 3.0]));

    println!("{:?}", &[1.0, 2.0, 3.0].iter().fold(1.0, |acc, x| acc / x));
    println!("{:?}", fold_left(&|acc, x| acc / x, 1.0, &[1.0, 2.0, 3.0]));

    println!(
        "{:?}",
        &[1, 2, 3].iter().rfold(vec![], |mut acc, x| {
            acc.push(x);
            acc
        })
    );
    println!(
        "{:?}",
        fold_right(
            &|mut acc: Vec<i32>, x: &i32| {
                acc.push(*x);
                acc
            },
            vec![],
            &[1, 2, 3]
        )
    );
    println!(
        "{:?}",
        &[1, 2, 3].iter().fold(vec![], |mut acc, x| {
            acc.push(x);
            acc
        })
    );
    println!(
        "{:?}",
        fold_left(
            &|mut acc: Vec<i32>, x: &i32| {
                acc.push(*x);
                acc
            },
            vec![],
            &[1, 2, 3]
        )
    );
}
#[test]
fn test() {
    main();
}
