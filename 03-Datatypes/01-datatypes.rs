fn main(){
    // scalar types:
    //  - integers
    //  - floating point numbers
    //  - boolean
    //  - charcters
    // Compound types
    //  - tuples
    //  - array
    // Custom types
    //  - struct
    //  - enum


    // integers
    // length    signed        unsigned
    // 8-bit      i8             u8
    // 16-bit     i16            u16
    // 32-bit     i32            u32
    // 64-bit     i64            u64
    // 128-bit    i128           u128
    // arch       isize          usize

    let small_num:u8 = 255;
    let big_num:u128 = 123456023456789087;
    let small_num2:i8 = -1;
    let big_num2:i128 = - 123456023456789087;

    println!("Small number is {}", small_num);
    println!("Big number is {}", big_num);
    println!("Small number is {}", small_num2);
    println!("Big number is {}", big_num2);

    // Small number is 255
    // Big number is 123456023456789087
    // Small number is -1
    // Big number is -123456023456789087



    // Numeral system          Description                      Example
    // Decimal     Base-10     commom from                        98
    // Hexadecimal Base-16     prefixed with 0x                   0xff
    // Octal       Base-8      prefixed with 0o                   0o77
    // Binary      Base-2      prefixed with 0b                   0b1111111_000000
    // Byte        (u8 only)   ASCII characters, prefixed with b  b'A'

    let decimal = 98_222;
    let hex  = 0xfff;
    let oct = 0o777;
    let binary = 0b1111111_000000;
    let byte = b'A';

    println!("decimal {}",decimal);
    println!("hex {}",hex);
    println!("oct {}",oct);
    println!("binary {}",binary);
    println!("byte {}",byte);

    // decimal 98222
    // hex 4095
    // oct 511
    // binary 8128
    // byte 65

}