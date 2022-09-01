fn triangle1(row: usize, col: usize) -> usize {
    if col == 1 || row == col {
        1
    } else {
        triangle1(row - 1, col - 1) + triangle1(row - 1, col)
    }
}

fn main() {
    for row in 1..10 {
        for col in 1..=row {
            print!("{:5}", triangle1(row, col));
        }
        println!();
    }
}

#[test]
fn test() {
    main();
}
