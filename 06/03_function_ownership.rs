fn main() {
    let s = String::from("Hello");
    take(s);
    // println!("{}", s); // ERROR!

    // Using Return to get ownership back
    let mut s2 = String::from("Hello");
    s2 = take_and_give(s2);
    println!("{}", s2); //No Error

    // Using different variable
    let s3 = String::from("Hello");
    let s4 = take_and_give(s3);
    println!("{}", s4); //No Error
}

fn take(st: String) {
    println!("{}", st);
}

fn take_and_give(st: String) -> String {
    println!("{}", st);
    st
}
