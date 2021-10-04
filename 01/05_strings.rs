fn main() {
    let s = "Hello";
    println!("{}", s);
    let mut h = String::new();
    h = String::from(s);
    println!("{}", h);
    let mut i = String::from("World");
    println!("{}", i);
    i.push_str(" is nice");
    // h = String::from(s);
    println!("{}", i);
}
