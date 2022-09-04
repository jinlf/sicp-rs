fn quenes() -> Vec<[u8; 8]> {
    fn queen_cols(k: usize) -> Vec<[u8; 8]> {
        if k == 0 {
            empty_board()
        } else {
            queen_cols(k - 1)
                .iter()
                .flat_map(|rest_of_queens| {
                    (1..=8)
                        .collect::<Vec<u8>>()
                        .iter()
                        .map(|&new_row| adjoin_position(new_row, k, rest_of_queens))
                        .collect::<Vec<[u8; 8]>>()
                })
                .filter(|positions| is_safe(k, positions))
                .collect()
        }
    }
    queen_cols(8)
}
fn empty_board() -> Vec<[u8; 8]> {
    vec![[0; 8]]
}
fn is_safe(k: usize, positions: &[u8; 8]) -> bool {
    fn is_row_safe(row: u8, k: usize, positions: &[u8; 8]) -> bool {
        if k < 2 {
            true
        } else {
            positions[k - 2] != row && is_row_safe(row, k - 1, positions)
        }
    }
    fn is_left_up_safe(row: u8, k: usize, positions: &[u8; 8]) -> bool {
        if k < 2 || row < 1 {
            true
        } else {
            positions[k - 2] + 1 != row && is_left_up_safe(row - 1, k - 1, positions)
        }
    }
    fn is_left_down_safe(row: u8, k: usize, positions: &[u8; 8]) -> bool {
        if k < 2 || row > 8 {
            true
        } else {
            positions[k - 2] != row + 1 && is_left_down_safe(row + 1, k - 1, positions)
        }
    }
    if k < 2 {
        true
    } else {
        let row = positions[k - 1];
        is_row_safe(row, k, positions)
            && is_left_up_safe(row, k, positions)
            && is_left_down_safe(row, k, positions)
    }
}
fn adjoin_position(new_row: u8, k: usize, rest_of_queens: &[u8; 8]) -> [u8; 8] {
    let mut ret = *rest_of_queens;
    ret[k - 1] = new_row;
    ret
}

fn main() {
    let answers = quenes();
    assert_eq!(answers.len(), 92);
    assert!(answers.contains(&[3, 7, 2, 8, 5, 1, 4, 6]));
    println!("{:?}", answers);
}
#[test]
fn test() {
    main();
}
