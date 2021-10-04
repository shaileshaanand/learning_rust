/*
* Tuple is compound type
* Fixed Size
*/
fn main() {
    let mytup = (10, 3.2, false);
    let mytup2: (i32, f32, bool) = (10, 3.2, false);
    println!("{:?}", mytup);
    println!("{:?}", mytup2);
    println!("{}", mytup.0);
    println!("{}", mytup.1);
    println!("{}", mytup.2);
    show(mytup);
}

fn show(tup: (i32, f32, bool)) {
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    let (a, b, c) = tup; // Destructuring
    println!("{} {} {}", a, b, c);
}
