
/**
 * function pointer in c/c++, anonymous function in js
 *
 * a function pointer is a variable that stores the address of a function
 * in rust, we can use a closure to create a function pointer
 */

fn function_with_closure<G>(f: G) where G: FnOnce(&str) {
    f("hello world");
}

fn main() {
    let s = "the content of the x is : ";
    let print_x_closure = |x: &str| {
        println!("{} {}", s, x);
    };
    function_with_closure(print_x_closure);
    function_with_closure(|x: &str| {
        println!("{} {}", s, x);
    });
}
