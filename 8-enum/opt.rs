// in rust there is no null value, we have option enum

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let some_num = Some(5);
    let some_str = Some("a str");
    let absent_num: Option<i32> = None;

    println!("{:?}", some_num);
    println!("{:?}", some_str);
    println!("{:?}", absent_num);

    // Some(5)
    // Some("a str")
    // None

    let x: i8 = 5;
    let y: Option<i8> = Some(4);

    let sum = x + y.unwrap();
    // unwrap() is a method that returns the value inside the Option<T> if it is Some(T)

    println!("Sum : {:?}", sum);
    // Sum : 9
}
