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
struct Rectangle {
    width: u32,
    height: u32,
}
fn rectangle_struct() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rect_struct is {}", area_struct(&rect));
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}