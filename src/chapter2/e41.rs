fn tripple(n: u32, s: u32) -> Vec<(u32, u32, u32)> {
    (1..=n)
        .collect::<Vec<u32>>()
        .iter()
        .flat_map(|&i| {
            (1..i)
                .collect::<Vec<u32>>()
                .iter()
                .flat_map(|&j| {
                    (1..j)
                        .collect::<Vec<u32>>()
                        .iter()
                        .map(|&k| (i, j, k))
                        .collect::<Vec<(u32, u32, u32)>>()
                })
                .collect::<Vec<(u32, u32, u32)>>()
        })
        .filter(|&(i, j, k)| i + j + k == s && i != j && j != k && i != k)
        .collect()
}

fn main() {
    println!("{:#?}", tripple(10, 20));
}
#[test]
fn test() {
    main();
}
