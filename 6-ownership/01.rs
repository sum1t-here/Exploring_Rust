// Ownership is Rust's most unique feature.
// It enables to make memory safety guarantees without needing a garbage collector

// Memory handling:
// Approach 1: Having a garbage collector that regularly looks for no longer used
// memory as program runs
// Approach 2: Explicitly allocate/free memory(C, C++, Assembly)
// In Rust, memory managed through a set of rules that the compiler checks. If any
// rule is violated, the program won't compile

// stack & heap

// Data size with an unknown size at compile time or size that might change can't
// be stored on the Stack. They use Heap instead

// Purpose:
// Primary purpose is to manage heap data
// Keep track of what code is using what data on the heap
// Minimizing the amt of duplicate data on the heap
// Cleaning up unused data on the heap

// Rules:
// Each value has owner
// There can only be one owner at a time
// when the owner goes out of scope, the value will be dropped

// variable scope

// fn main() {
//     // s is not valid here, it's not yet declared
//     let s = "hello";

//     // do .... // s is still valid
// } // the scope is over s is not longer valid

// fn main() {
//     // s is not valid
//     let mut s = String::from("hello"); // s is valid and on the heap

//     // s is valid
//     s.push_str(", world!");

//     println!("{}", s);
// } // s is not valid anymore, we call drop

// fn main() {
//     let s1 = String::from("hello"); // create a string
//     let s2 = s1; // assign s1 to s2
//     // print s2
//     println!("s2 = {}", s2);
//     // print s1
//     println!("s1 = {}", s1);
// }

fn main() {
    let s1 = String::from("hello"); // create a string
    let s2 = s1.clone(); // assign s1 to s2
    // print s2
    println!("s2 = {}", s2);
    // print s1
    println!("s1 = {}", s1);
}
