#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: i32,
}

fn main() {
    let user1 = User {
        email: String::from("anaandshailu@gmail.com"),
        username: String::from("shailesh"),
        age: 23,
    };
    println!("{:?}", user1);
    println!("{}", user1.age);
    println!("{}", user1.username);
    println!("{}", user1.email);

    let mut user2 = User {
        email: String::from("anaandshailu@gmail.com"),
        username: String::from("shailesh"),
        age: 23,
    };
    user2.age = 30;
    println!("{:?}", user2);
    println!("{}", user2.age);
    println!("{}", user2.username);
    println!("{}", user2.email);
}
