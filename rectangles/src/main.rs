#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    fn square(size: usize) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("Area of rectangle is {}.", rect1.area());

    let rect2 = Rectangle {width: 10, height: 40};
    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {width: 60, height: 45};
    println!("can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(30);
    println!("square be like: {:?}", square);
}
