fn main() {
    // slice of an array of characters
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];
    println!("{:?}", slice);

    // slice of a vector of int
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice: &[i32] = &vec[3..4];
    println!("{:?}", slice);

    // slice of strings
    let s: String = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{:?}", hello);
    println!("{:?}", world);
}
