fn cc(amount: isize, coins_values: Vec<usize>) -> usize {
    if amount == 0 {
        1
    } else if amount < 0 || coins_values.len() == 0 {
        0
    } else {
        cc(amount, except_first_denomination(&coins_values))
            + cc(
                amount - first_denomination(&coins_values) as isize,
                coins_values,
            )
    }
}
fn except_first_denomination(coins_values: &Vec<usize>) -> Vec<usize> {
    coins_values[1..].to_vec()
}
fn first_denomination(coins_values: &Vec<usize>) -> usize {
    coins_values[0]
}

fn main() {
    let us_coins = vec![50, 25, 10, 5, 1];
    assert_eq!(cc(100, us_coins), 292);
}
#[test]
fn test() {
    main();
}
