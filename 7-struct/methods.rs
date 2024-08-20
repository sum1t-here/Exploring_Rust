struct Rectangle {
    width: u32,
    height: u32,
}

// impl block is usde to define methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // getter for height
    fn height(&self) -> u32 {
        self.height
    }

    // check if an rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }

    // associated function to define a square
    // fn square(size: u32) -> Self {
    //     Self {
    //         width: size,
    //         height: size,
    //     }
    // }
}

// multiple impl blocks
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // print the area
    println!("The area of the rectangle is {} square pixels", rect1.area());
    // The area of the rectangle is 1500 square pixels

    if rect1.width() {
        println!("The width of the rectangle is {}", rect1.width);
    } else {
        println!("The width of the rectangle is 0");
    }

    // The width of the rectangle is 30

    println!("The height of the rectangle is {}", rect1.height);
    // The height of the rectangle is 50

    let rec2 = Rectangle {
        width: 30,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    // can rect1 hold rect2
    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rec2));

    // can rect3 hold rect2
    println!("Can rect3 hold rect2 {}", rec3.can_hold(&rec2));

    // Can rect1 hold rect2 false
    // Can rect3 hold rect2 true

    // create a square
    let square = Rectangle::square(10);

    // calculate the area
    println!("The area of the square is : {}", square.area());
    // The area of the square is : 100
}
