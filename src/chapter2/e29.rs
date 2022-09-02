trait Mobile {
    fn left(&self) -> &Box<dyn Branch>;
    fn right(&self) -> &Box<dyn Branch>;
    fn total_weight(&self) -> f64 {
        self.left().total_weight() + self.right().total_weight()
    }
    fn is_balanced(&self) -> bool {
        self.left().is_balanced()
            && self.right().is_balanced()
            && self.left().moment() == self.right().moment()
    }
}
trait Branch {
    fn length(&self) -> f64;
    fn structure(&self) -> &Structure;
    fn total_weight(&self) -> f64 {
        match self.structure() {
            Structure::Weigh(w) => *w,
            Structure::Mobile(mobile) => mobile.total_weight(),
        }
    }
    fn moment(&self) -> f64 {
        self.length() * self.total_weight()
    }
    fn is_balanced(&self) -> bool {
        match self.structure() {
            Structure::Weigh(_) => true,
            Structure::Mobile(mobile) => mobile.is_balanced(),
        }
    }
}

struct Mobile1(Vec<Box<dyn Branch>>);
impl Mobile for Mobile1 {
    fn left(&self) -> &Box<dyn Branch> {
        &self.0[0]
    }
    fn right(&self) -> &Box<dyn Branch> {
        &self.0[1]
    }
}
enum Node {
    Length(f64),
    Structure(Structure),
}

struct Branch1(Vec<Node>);
impl Branch for Branch1 {
    fn length(&self) -> f64 {
        match &self.0[0] {
            Node::Length(length) => *length,
            _ => todo!(),
        }
    }

    fn structure(&self) -> &Structure {
        match &self.0[1] {
            Node::Structure(s) => s,
            _ => todo!(),
        }
    }
}

struct Mobile2(Box<dyn Branch>, Box<dyn Branch>);
impl Mobile for Mobile2 {
    fn left(&self) -> &Box<dyn Branch> {
        &self.0
    }

    fn right(&self) -> &Box<dyn Branch> {
        &self.1
    }
}
struct Branch2(f64, Structure);
impl Branch for Branch2 {
    fn length(&self) -> f64 {
        self.0
    }

    fn structure(&self) -> &Structure {
        &self.1
    }
}

enum Structure {
    Weigh(f64),
    Mobile(Box<dyn Mobile>),
}

impl Mobile1 {
    fn new(left: Box<dyn Branch>, right: Box<dyn Branch>) -> Self {
        Self(vec![left, right])
    }
}
impl Branch1 {
    fn new(length: f64, structure: Structure) -> Self {
        Self(vec![Node::Length(length), Node::Structure(structure)])
    }
}
impl Mobile2 {
    fn new(left: Box<dyn Branch>, right: Box<dyn Branch>) -> Self {
        Self(left, right)
    }
}
impl Branch2 {
    fn new(length: f64, structure: Structure) -> Self {
        Self(length, structure)
    }
}

fn main() {
    let left1 = Branch1::new(1.0, Structure::Weigh(2.0));
    let right1 = Branch2::new(2.0, Structure::Weigh(1.0));
    let mobile_right = Mobile2::new(Box::new(left1), Box::new(right1));
    let left = Branch1::new(1.0, Structure::Weigh(3.0));
    let right = Branch2::new(1.0, Structure::Mobile(Box::new(mobile_right)));
    let mobile = Mobile1::new(Box::new(left), Box::new(right));
    println!("{}", mobile.total_weight());
    assert!(mobile.is_balanced());
}
#[test]
fn test() {
    main();
}
