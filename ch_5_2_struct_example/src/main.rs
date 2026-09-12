#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 40;
    println!("Area of the rectangle is {}", area(width, height));

    let dimentions = (30, 50);
    println!("Area of the rectangle is {}", area_with_tuple(dimentions));

    let rect1 = Rectangle {
        width: 32,
        height: 43,
    };
    println!(
        "Area of the rectangle with dimentions {rect1:#?} is {}",
        area_with_struct(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };
    dbg!(
        "Area of the rectangle with dimentions",
        &rect2,
        area_with_struct(&rect2)
    );

    let rect3 = Rectangle {
        width: 45,
        height: 42,
    };
    println!(
        "Area: {0}, can hold other rect? {1}",
        rect3.area(),
        rect3.can_hold(&rect1)
    );

    let sq1 = Rectangle::square(49);
    println!("Area of square: {}", sq1.area());
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_with_tuple(dimensions: (i32, i32)) -> i32 {
    let (width, height) = dimensions;
    width * height
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}
