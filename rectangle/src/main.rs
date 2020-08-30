fn main() {
    rectangle_simple();
    rectangle_tupel();
    rectangle_struct();
}

// Define the dimensions of the rectangle using separate variable for width and height
fn rectangle_simple() {
    let width = 30;
    let height = 50;

    println!("The area of the rect_simple is {}", area_simple(width, height));
}

fn area_simple(width : u32, height : u32) -> u32 {
    width * height
}

// Define the dimension of the rectangle using a tupel
fn rectangle_tupel() {
    let rect1 = (30, 50);
    println!("the area of rect_tupel is {}", area_tupel(rect1))

}

fn area_tupel(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Define the dimensions of the rectangle using a struct
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
}
impl Rectangle {
    fn square(size : u32) -> Rectangle {
        Rectangle {
            width : size,
            height : size,
        }
    }
}
fn rectangle_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect_struct is {:?}", rect1);
    println!("The area of rect_struct is {}", area_struct(&rect1));
    println!("The area of rect_struct is {}", rect1.area());
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
    println!("A square of size 10 is {:?}", Rectangle::square(10));
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}