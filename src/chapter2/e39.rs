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

fn reverse<T>(sequence: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    fold_right(
        &|acc: Vec<T>, y: &T| {
            let mut new_acc = acc;
            new_acc.push(y.clone());
            new_acc
        },
        vec![],
        sequence,
    )
}
fn reverse1<T>(sequence: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    fold_left(
        &|acc: Vec<T>, y: &T| {
            let mut new_acc = acc;
            new_acc.insert(0, y.clone());
            new_acc
        },
        vec![],
        sequence,
    )
}

fn main() {
    let list = vec![1, 2, 3];
    assert_eq!(reverse(&list), list.into_iter().rev().collect::<Vec<i32>>());
    let list = vec![1, 2, 3];
    assert_eq!(
        reverse1(&list),
        list.into_iter().rev().collect::<Vec<i32>>()
    );
}
#[test]
fn test() {
    main();
}
