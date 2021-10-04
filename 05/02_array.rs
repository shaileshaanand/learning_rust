fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let mut arr3: [i32; 5] = [0; 5];
    println!("{:?} {:?} {:?}", arr1, arr2, arr3);
    arr3[2] = 10;
    println!("{:?}", arr3);
    println!("at ind 2 {}", arr3[2]);
    println!("len {}", arr3.len());
    show(arr3)
}

fn show(arr: [i32; 5]) {
    for num in arr.iter() {
        println!("{}", num);
    }
}
