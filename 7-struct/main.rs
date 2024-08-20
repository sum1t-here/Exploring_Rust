// stucts in rust: used to create custom data types
// similar to tuples, but with named fields
// used to create more complex data types
// structs are immutable by default

#[derive(Debug, Default)]
struct User {
    username: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    // use mut to make user mutable
    let mut user1 = User {
        username: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25,
    };

    // Name: John Doe
    // Email: doe@mail.com
    // Is Active: true
    // Age: 25

    println!(
        "Name: {} \nEmail: {} \nIs Active: {} \nAge: {}",
        user1.username,
        user1.email,
        user1.is_active,
        user1.age
    );

    // print all the values
    println!("User1 {:?}", user1);

    // User1 User { username: "John Doe", email: "doe@mail.com", is_active: true, age: 25 }

    // mutable structs
    user1.username = String::from("Batman");
    println!("User1 {:?}", user1);
    // User1 User { username: "Batman", email: "doe@mail.com", is_active: true, age: 25 }

    let user2 = build_user(String::from("John Doe"), String::from("doe@mail.com"));
    println!(
        "Name: {} \nEmail: {} \nIs Active: {} \nAge: {}",
        user2.username,
        user2.email,
        user2.is_active,
        user2.age
    );
    // Name: John Doe
    // Email: doe@mail.com
    // Is Active: true
    // Age: 25
}

// create a function to build a user
fn build_user(name: String, email: String) -> User {
    User {
        username: name,
        email,
        is_active: true,
        age: 25,
    }
}
