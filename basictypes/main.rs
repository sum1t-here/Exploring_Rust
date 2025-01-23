fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn build_vector_i16() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    print_type_of(&v[0]);   // i16
    print_type_of(&v);      // alloc::vec::Vec<i16>
    v
}

fn build_vector_u16() -> Vec<u16> {
    let mut v = Vec::new();
    v.push(10);
    print_type_of(&v[0]);   // u16
    print_type_of(&v);      // alloc::vec::Vec<u16>
    v
}

fn main() {
    build_vector_i16();

    build_vector_u16();

    // in rust char != u8
    let c = 'a';
    print_type_of(&c);          // char

    // add prefix b to make char to u8
    let d: u8 = b'a';
    print_type_of(&d);          // u8

    // type conversion using as
    let d1 = 64i32 as u32;
    print_type_of(&d1); // u32

    // conversion overflow
    let e = 1000i16 as u8;
    println!("value of e : {}", e);
    // value of e : 232

    println!("2 power 4 is {}", 2_i32.pow(4));
    // 2 power 4 is 16 -> not using types give error
}
