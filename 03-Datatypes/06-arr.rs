fn main() {
    let arr = [1, 2, 3, 4, 5]; // array of five elements

    let first = arr[0];

    println!("first: {}", first);
    // first: 1

    for elements in arr.iter() {
        println!("{}", elements);
    }
    // 1
    // 2
    // 3
    // 4
    // 5
}
