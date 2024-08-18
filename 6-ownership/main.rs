// Excercise: to get the first index of a string (slices)
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The s is {}", s);
    println!("The first word is = {}", word);

    // string literals
    let s2 = "second world";
    let word2 = first_word(s2);
    println!("The s2 is {}", s2);
    println!("The first word of s2 is = {}", word2);
}

// function to get the first word of a string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert the string to bytes

    // iterate through the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if the byte is a space
            return &s[0..i]; // return the slice of the string
        }
    }

    &s[..] // return the whole string if there is no space
}
