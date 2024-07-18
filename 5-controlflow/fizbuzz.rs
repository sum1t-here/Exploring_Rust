/*
Print number from 1 to 100 but for multiples of 3 print fizz
for multiple of 5 print buzz
and for both fizzbuzz
*/

fn main(){
    for number in 1..=100{
        if number % 3 == 0 && number % 5 == 0{
            println!("fizzbuzz");
        }else if number % 3 == 0{
            println!("fizz");
        }else if number % 5 == 0{
            println!("buzz");
        }else {
            println!("{number}")
        }
    }
}