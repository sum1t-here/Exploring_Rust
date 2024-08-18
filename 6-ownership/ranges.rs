fn main() {
    let s = String::from("Learning");

    // shortcut for initial index
    let slice = &s[0..3];
    println!("{}", slice);
    let slice = &s[..3];
    println!("{}", slice);

    // shortcut for final index
    let len = s.len();
    let slice = &s[4..len];
    println!("{}", slice);
    let slice = &s[4..];
    println!("{}", slice);

    // shortcut for both initial and final index
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}

// Lea
// Lea
// ning
// ning
// Learning
// Learning
