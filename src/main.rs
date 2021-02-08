use koriel::ast::Def;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let deserialized: Vec<Def> = serde_json::from_str(&buffer.as_str()).unwrap();
    // println!("{:?}", deserialized);
    let serialized = serde_json::to_string(&deserialized).unwrap();
    println!("{}", serialized);
    Ok(())
}
