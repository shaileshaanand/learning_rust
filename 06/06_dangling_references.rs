fn main() {
    let s = dangle();
}

fn dangle() -> &String {
    let d = String::from("Hello");
    &d // ERROR
}
