use std::str::FromStr;

fn main() {
    let num_str = "123";
    /**
     * result is a type of enumeration -> container -> contain some kind of object inside it
     * result is type of Result
     * it take two kind of value one Ok, Err
     * both of them are container
     */
    let result = i32::from_str(&num_str);
    println!("The result is: {:?}", result);
    // The result is: Ok(123)
    match result {
        Ok(val) => {
            println!("The value is: {}", val);
        }
        Err(e) => {
            println!("The error is: {}", e);
        }
    }
}
