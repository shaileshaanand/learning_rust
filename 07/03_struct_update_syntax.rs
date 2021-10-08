#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    // hobby: String,
}

fn main() {
    let u1 = User {
        name: String::from("Shailesh"),
        age: 25,
        // hobby: String::from("Cricket"),
    };
    let u2 = User {
        name: String::from("Satyam"),
        ..u1 // Struct update Syntax
    };
    println!("{:?} {:?}", u1, u2);
}
