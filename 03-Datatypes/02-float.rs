fn main(){
    let x = 2.0; // f64 by defaut
    let y:f32 = 3.0; // f32
    println!("x = {}, y = {}", x,y);

    // x = 2, y = 3

    // NUMERIC OPERATIONS
    let sum = x + y;
    let diff = x - y;
    let product = x * y;
    let remainder = 45 % 3;
    let quotient = x / y;

    println!("Sum : {}",sum);
    println!("Difference : {}",diff);
    println!("Product : {}",product);
    println!("Remainder : {}",remainder);
    println!("Quotient : {}",quotient);

    // Sum : 5
    // Difference : -1
    // Product : 6
    // Remainder : 0
    // Quotient : 0.6666667
}