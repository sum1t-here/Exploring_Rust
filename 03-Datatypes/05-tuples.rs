fn main(){
    let tup : (i32, f64, char) = (500, 6.4, 'x' );
    
    // destructuring

    let (x, y, z) = tup;
    println!("y is {}", y);
    // y is 6.4

    // accessing by index

    let a = tup.0;
    println!("a is {}", a);
    // a is 500
}