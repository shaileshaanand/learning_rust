/*
* Using refrences we can refer values without taking ownership
* & represents a reference
*/

fn main() {
    let mut s1 = String::from("Hello");
    show(&s1); // Imutable Ref
    println!("{}", s1);
    add_world(&mut s1); // Mut Ref
    println!("{}", s1);
}

fn show(s1: &String) {
    println!("{}", s1);
    // s1.push_str("World"); // ERROR!
}

fn add_world(st: &mut String) {
    st.push_str("World");
}
