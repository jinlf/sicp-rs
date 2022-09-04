fn accumulate_n<A, B>(op: &impl Fn(A, B) -> B, init: B, seqs: &Vec<Vec<A>>) -> Vec<B>
where
    A: Clone,
    B: Clone,
{
    if seqs.is_empty() || seqs[0].is_empty() {
        vec![]
    } else {
        let first: B = seqs
            .iter()
            .map(|seq| seq[0].clone())
            .rfold(init.clone(), |acc, x| op(x, acc));
        let mut rest = accumulate_n(
            op,
            init,
            &seqs.iter().map(|seq| seq[1..].to_vec()).collect(),
        );
        rest.insert(0, first);
        rest
    }
}

fn map<A, B>(proc: &impl Fn(&Vec<A>) -> B, seqs: &Vec<Vec<A>>) -> Vec<B>
where
    A: Copy,
{
    if seqs.is_empty() || seqs[0].is_empty() {
        vec![]
    } else {
        let first = proc(&seqs.iter().map(|seq| seq[0]).collect());
        let mut rest = map(proc, &seqs.iter().map(|seq| seq[1..].to_vec()).collect());
        rest.insert(0, first);
        rest
    }
}

fn dot_product(v: &Vec<usize>, w: &Vec<usize>) -> usize {
    map(&|x| multiply(x), &vec![v.to_vec(), w.to_vec()])
        .iter()
        .rfold(0, |acc, x| acc + x)
}
fn multiply(v: &Vec<usize>) -> usize {
    v.iter().rfold(1, |acc, &x| acc * x)
}
fn matrix_mul_vector(m: &Vec<Vec<usize>>, v: &Vec<usize>) -> Vec<usize> {
    m.iter().map(|x| dot_product(x, v)).collect()
}
fn transpose(m: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    accumulate_n(
        &|a, mut b: Vec<usize>| {
            b.insert(0, a);
            b
        },
        vec![],
        m,
    )
}
fn matrix_mul_matrix(m: &Vec<Vec<usize>>, n: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let cols = transpose(n);
    m.iter().map(|x| matrix_mul_vector(&cols, x)).collect()
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let w = vec![5, 6, 7, 8];
    let v1 = nalgebra::Vector4::from_vec(v.clone());
    let w1 = nalgebra::Vector4::from_vec(w.clone());
    assert_eq!(dot_product(&v, &w), v1.dot(&w1));

    let m = vec![vec![1, 2, 3, 4], vec![4, 5, 6, 6], vec![6, 7, 8, 9]];
    let m1 = nalgebra::Matrix3x4::from_fn(|r, c| m[r][c]);
    assert_eq!(
        nalgebra::Matrix3x1::from_vec(matrix_mul_vector(&m, &v)),
        m1 * v1
    );

    let y = transpose(&m);
    assert_eq!(nalgebra::Matrix4x3::from_fn(|r, c| y[r][c]), m1.transpose());

    let n = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];
    let n1 = nalgebra::Matrix4x3::from_fn(|r, c| n[r][c]);
    let a = matrix_mul_matrix(&m, &n);
    assert_eq!(nalgebra::Matrix3::from_fn(|r, c| a[r][c]), m1 * n1);
}
#[test]
fn test() {
    main();
}
