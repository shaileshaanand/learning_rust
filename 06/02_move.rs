// Copying is cloned automatically for data on stack
// Use clone() for heap data

fn main() {
    let a = 20;
    let b = a;
    println!("{} {}", a, b); // No ERROR

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{} {}", s1, s2); // ERROR!
    println!("{}", s2);

    // Using clone
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("{} {}", s3, s4); // No ERROR
}
