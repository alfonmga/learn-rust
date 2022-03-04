#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool { self.width > 0 }
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, target: &Rectangle) -> bool { self.area() > target.area() }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 30 };
    let rect2 = Rectangle { width: 33, height: 33 };
    let rect3 = Rectangle { width: 11, height: 11 };

    println!("Can `rect1` hodl `rect2`? {}", rect1.can_hold(&rect2));
    println!("Can `rect1` hodl `rect3`? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("The area of the rectangle `rect1` is: {} square pixels", rect1.area());

    let sq = Rectangle::square(22);
    dbg!(sq);
}
