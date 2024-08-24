// #[derive(Debug)]
// enum IpAddrKind {
//     v4,
//     v6,
// }

// // enum and structs
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::v4;
//     let six = IpAddrKind::v6;

//     println!("four: {:?}", four);
//     println!("six: {:?}", six);

//     // four: v4
//     // six: v6

//     route(four);
//     route(six);

//     // route called with v4
//     // route called with v6

//     let home = IpAddr {
//         kind: IpAddrKind::v4,
//         address: String::from("127.0.0.1"),
//     };

//     println!("{:?}", home);

//     let loopback = IpAddr {
//         kind: IpAddrKind::v6,
//         address: String::from("::1"),
//     };

//     println!("{:?}", loopback);
//     // IpAddr { kind: v4, address: "127.0.0.1" }
//     // IpAddr { kind: v6, address: "::1" }
// }

// fn route(ip_type: IpAddrKind) {
//     println!("route called with {:?}", ip_type);
// }

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", home);

    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", loopback);

    // V4(127, 0, 0, 1)
    // V6("::1")
}
