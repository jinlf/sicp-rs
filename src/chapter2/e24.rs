use std::fmt::Display;

struct List(Vec<N>);
enum N {
    V(usize),
    L(List),
}
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::new(), |acc, x| if acc.is_empty() {
                    format!("{}", x)
                } else {
                    format!("{}, {}", acc, x)
                })
        )?;
        write!(f, ")")
    }
}
impl Display for N {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V(v) => write!(f, "{}", v),
            Self::L(list) => write!(f, "{}", list),
        }
    }
}

fn main() {
    let list = List(vec![
        N::V(1),
        N::L(List(vec![N::V(2), N::L(List(vec![N::V(3), N::V(4)]))])),
    ]);
    println!("{}", list);
}
#[test]
fn test() {
    main();
}
