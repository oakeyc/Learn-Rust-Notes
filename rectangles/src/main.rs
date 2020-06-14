#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn square(size: i32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.height >= rect2.height && self.width >= rect2.width
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 30,
        width: 50,
    };

    let rect2 = Rectangle {
        height: 20,
        width: 10,
    };
    let rect3 = Rectangle {
        height: 90,
        width: 10,
    };

    let sq = Rectangle::square(60);
    println!("The Area is {}", area(&rectangle));
    println!("The Struct is {:?}", rectangle);
    println!("The Struct Area is {}", rectangle.area());
    println!("Can Rect1 Hold Rect2: {}", rectangle.can_hold(&rect2));
    println!("Can Rect1 Hold Rect3: {}", rectangle.can_hold(&rect3));
    println!("Do we have a square: {:?}", sq);
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.height * rectangle.width
}
