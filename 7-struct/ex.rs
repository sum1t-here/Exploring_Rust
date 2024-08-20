// fn main() {
//     let width = 30;
//     let height = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width, height));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // The area of the rectangle is 1500 square pixels.

// with tuples ....

// fn main() {
//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels.", area(rect1))
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// The area of the rectangle is 1500 square pixels.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    // print the instance
    println!("rect1 is {:?}", rect1);

    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1)

    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// The area of the rectangle is 1500 square pixels.
