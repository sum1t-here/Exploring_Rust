struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("JOHN"),
        age: 25,
    };

    println!("Person name is {}, age is {}", person.name, person.age);

    // Person name is JOHN, age is 25
}
