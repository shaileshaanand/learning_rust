fn main() {
    let a = String::from("HelloWorld");
    let r1 = &a[0..5];
    let r2 = &a[0..=5];
    let r3 = &a[..5];
    let r4 = &a[0..];
    let r5 = &a[..];
    println!("a = {}", a);
    println!("&a[0..5] = {}", r1);
    println!("&a[0..=5] = {}", r2);
    println!("&a[..5] = {}", r3);
    println!("&a[0..] = {}", r4);
    println!("&a[..] = {}", r5);

    println!("{}", "\n");

    let b = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let r1 = &b[0..5];
    let r2 = &b[0..=5];
    let r3 = &b[..5];
    let r4 = &b[0..];
    let r5 = &b[..];
    println!("b = {:?}", b);
    println!("&a[0..5] = {:?}", r1);
    println!("&a[0..=5] = {:?}", r2);
    println!("&a[..5] = {:?}", r3);
    println!("&a[0..] = {:?}", r4);
    println!("&a[..] = {:?}", r5);
}
