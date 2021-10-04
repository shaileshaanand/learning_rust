fn main() {
    // simple loop
    let mut a = 1;
    loop {
        a += 1;
        println!("{}", 10);
        if a > 5 {
            break;
        }
    }

    // while
    a = 1;
    while a <= 5 {
        println!("{}", a);
        a += 1;
    }
    // for loop
    println!("-for loop-",);
    for n in 1..11 {
        println!("{}", n);
    }
    /*
     * Rust for loop is faster than while loop
     * because while adds additional checks
     */
}
