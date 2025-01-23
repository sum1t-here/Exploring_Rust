fn main() {
    /**
     * Pointer type has three formats :
     * 1. reference
     * (reference -> means right to use but nor right to own)
     */

    // Immutable variable
    let x = 10;
    println!("x = {}", x); 
    // x = 10

    // Attempting to modify `x` will result in a compilation error
    // x = 20; // Uncommenting this line will cause an error

    // Immutable reference to `x`
    let x1 = &x;
    println!("x1 (reference to x) = {}", x1);
    // x1 (reference to x) = 10

    // Mutable variable
    let mut z = 10;
    println!("z = {}", z);
    // z = 10

    // Modify `z`
    z = 20;
    println!("z after modification = {}", z);
    // z after modification = 20

    // Mutable reference to `z`

    // Mutable references allow you to modify the value of a variable. 
    // Only one mutable reference to a variable is allowed at a time, 
    // and no immutable references can coexist with a mutable reference.

    let z1 = &mut z;
    *z1 += 5; // Dereference `z1` to modify `z`
    println!("z1 after modifying through mutable reference = {}", z1);
    // z1 after modifying through mutable reference = 25


    let mut a = 1;
    // mutable reference
    let a1 = &mut a;
    *a1 += 5;
    println!("a1 after modification = {}", a1);
    // a1 after modification = 6

    // immutable and mutable references together
    let mut b = 10;

    // Immutable reference
    let b1 = &b;
    println!("b1 = {}", b1);
    // b1 = 10
    // Mutable reference (only allowed after `b1` is no longer used)
    let b2 = &mut b;
    *b2 += 5;
    println!("b2 = {}", b2);
    // b2 = 15

    // Immutable reference again (after `b2` is no longer used)
    let b3 = &b;
    println!("b3 = {}", b3);
    // b3 = 15

// Key Points to Remember
// Immutable Variables: Cannot be modified after assignment.

// Mutable Variables: Can be modified using the mut keyword.

// Immutable References: Allow read-only access; multiple immutable references are allowed.

// Mutable References: Allow modification; only one mutable reference is allowed at a time.

// Rust's Borrowing Rules:

// You cannot have a mutable reference while an immutable reference exists.

// You can have multiple immutable references simultaneously.

/**
 * Box<T> is a smart pointer that allows you to store data on the heap rather than the stack. It is useful when:
 * You have a type whose size is not known at compile time (e.g., recursive types).
 * You want to transfer ownership of a large amount of data without copying it.
 * You want to ensure a type has a fixed size (e.g., for trait objects).
 * 
 * Key Points About Box<T>
 * Heap Allocation: Box<T> stores data on the heap.
 * Ownership: Box<T> owns the data it points to.
 * Recursive Types: Useful for types with unknown sizes at compile time (e.g., recursive types).
 * Trait Objects: Enables dynamic dispatch for trait objects.
 * Automatic Cleanup: Memory is automatically deallocated when the box goes out of scope.
 */
    // Store an integer on the heap using Box
    let p = Box::new(42);
    println!("p = {}", p); // Dereference the box to access the value

    // transferring ownership
    let q = p; // Ownership of the box is moved to `q`
    println!("q = {}", q);

    // dereferencing 
   
        let r = Box::new(42);
        let s = *r; // Dereference the box to get the value
        println!("s = {}", s);
    
    // Nested box
    // let x = Box::new(Box::new(42)); // Nested box
    // println!("x = {}", **x); // Double dereference to access the value

    // 3 raw pointer, same as in c, cpp
}