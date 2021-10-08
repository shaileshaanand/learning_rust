#[derive(Debug)]
struct User {
    age: i32,
}
fn build(age: i32) -> User {
    // User { age: age }
    User { age } // Field init Shorthand
}
fn main() {
    println!("{:?}", build(32));
}
