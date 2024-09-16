fn main() {
    // create a vector
    let mut vec1: Vec<i32> = Vec::new();
    println!("{:?}", vec1);

    // create a vector with initial values
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec2);

    // update a vector
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("{:?}", vec1);

    vec1.pop();
    println!("{:?}", vec1);

    // let third: &i32 = &vec1[2];
    // println!("The third element is {third}");

    let third: Option<&i32> = vec1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
