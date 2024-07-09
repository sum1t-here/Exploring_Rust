fn main(){
    let t = true; // implicit declaration
    let f:bool = false; // explicit declaration

    println!("t = {}, f = {}",t,f);

    // t = true, f = false

    // if
    if t{
        println!("t is true");
    }else{
        println!("t is false");
    }

    // t is true


    let not_t = !t;
    println!("not_t {}", not_t);
    // not_t false
}