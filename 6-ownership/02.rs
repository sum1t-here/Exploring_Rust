// Reference & borrowing

// ownership and functions

// fn main() {
//     let i = 5;
//     call_int(i);
//     println!("After calling the function the value of i: {}", i);

//     let s = String::from("Hello");
//     call_str(s);
//     println!("After calling the function the value of s: {}", s);
// }

// fn call_int(i: i32) {
//     println!("call_int i: {}", i);
// }

// fn call_str(s: String) {
//     println!("call_str s: {}", s);
// }

// return value and scope

// fn main() {
//     let s1 = give_ownership();
//     println!("s1: {}", s1);

//     let s2 = String::from("Hello from main");

//     let s3 = take_and_give_ownership(s2);
//     println!("s3: {}", s3);
// }

// fn give_ownership() -> String {
//     let some_str = String::from("Hello from give_ownership");
//     some_str
// }

// fn take_and_give_ownership(some_str: String) -> String {
//     some_str
// }

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of `{}` is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// Borrowing

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // passing reference to the string
    println!("The length of `{}` is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// mutable reference
// multiple
// dangling
