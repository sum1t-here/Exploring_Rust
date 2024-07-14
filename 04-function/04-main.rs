// return value

fn sum_diff(num1: i32, num2: i32) -> (i32, i32) {
    // (num1 + num2, num1 - num2)
    // early return;
    return (num1 + num2, num1 - num2);
}

fn main() {
    let x = sum_diff(5, 7);
    println!("The value of x is, {:?}", x);
    // {:?} is used to print tuple
}

// The value of x is, (12, -2)
