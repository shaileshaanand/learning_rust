use std::io;
fn main() {
    let mut num = String::new();
    println!("Enter a Number:");
    io::stdin().read_line(&mut num).expect("Fail");
    let num: i32 = num.trim().parse().expect("Fail");
    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
