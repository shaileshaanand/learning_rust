#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function
    fn build(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 70,
    };

    let area = rect1.area();
    println!("{}", area);
    println!("{}", rect2.can_hold(&rect1));
    println!("{}", rect1.can_hold(&rect3));
    println!("{:?}", Rectangle::build(12, 13)); // Calling an associated function
}
