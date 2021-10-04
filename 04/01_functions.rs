fn main() {
    println!("{}", is_even(24));
    println!("{:?}", fact(5));
}
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn fact(num: i32) -> (String, i32) {
    let mut fact = 1;
    for n in 1..num + 1 {
        fact *= n;
    }
    (String::from("Factorial is:"), fact)
}
