use std::io;
fn main() {
    // taking a String
    let mut a = String::new();
    println!("Enter a String:");
    io::stdin().read_line(&mut a).expect("Failed");
    a = a.trim().to_string();
    println!("{}", a);
    let mut b = String::new();
    // taking a number
    println!("Enter a Number:");
    io::stdin().read_line(&mut b).expect("Failed");
    let b: i32 = b.trim().parse().expect("Failed");
    println!("{}", b);
    // taking a boolean
    let mut c = String::new();
    println!("Enter a Boolean:");
    io::stdin().read_line(&mut c).expect("Failed");
    let c: bool = c.trim().parse().expect("Failed");
    println!("{}", c);
}
