use koriel::ast::*;

fn main() {
    let x = Exp {
        kind: ExpKind::Atom(Atom {
            kind: AtomKind::Var(String::from("hello")),
        }),
    };
    println!("{:?}", x);
}
