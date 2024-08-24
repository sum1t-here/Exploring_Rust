// #[derive(Debug)]
// struct QuitMsg; // unit struct

// #[derive(Debug)]
// struct MovMsg {
//     x: i32,
//     y: i32,
// }

// #[derive(Debug)]
// struct writeMsg(String); // tuple struct

// #[derive(Debug)]
// struct ChangeColorMsg(i32, i32, i32); // tuple struct

// impl QuitMsg {
//     fn call(&self) {
//         println!("Self is {:?}", self);
//     }
// }

// impl MovMsg {
//     fn call(&self) {
//         println!("Self is {:?}", self);
//     }
// }

// impl writeMsg {
//     fn call(&self) {
//         println!("Self is {:?}", self);
//     }
// }

// impl ChangeColorMsg {
//     fn call(&self) {
//         println!("Self is {:?}", self);
//     }
// }

// fn main() {
//     // create instance of the above structs
//     let q = QuitMsg;
//     let m = MovMsg { x: 10, y: 20 };
//     let w = writeMsg(String::from("Hello World"));
//     let c = ChangeColorMsg(0, 255, 0);

//     println!("{:?}", q);
//     println!("{:?}", m);
//     println!("{:?}", w);
//     println!("{:?}", c);

//     // QuitMsg
//     // MovMsg { x: 10, y: 20 }
//     // writeMsg("Hello World")
//     // ChangeColorMsg(0, 255, 0)

//     q.call();
//     m.call();
//     w.call();
//     c.call();

//     // Self is QuitMsg
//     // Self is MovMsg { x: 10, y: 20 }
//     // Self is writeMsg("Hello World")
//     // Self is ChangeColorMsg(0, 255, 0)
// }

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("Hello World"));
    let x = Message::Move { x: 3, y: 4 };
    let y = Message::ChangeColor(0, 0, 0);
    let z = Message::Quit;

    m.call();
    x.call();
    y.call();
    z.call();

    // Write("Hello World")
    // Move { x: 3, y: 4 }
    // ChangeColor(0, 0, 0)
    // Quit
}
