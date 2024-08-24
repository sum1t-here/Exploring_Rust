// // compare values against a series of patterns
// #[allow(dead_code)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Rarity),
// }

// #[allow(dead_code)]
// #[derive(Debug)]
// enum Rarity {
//     Common,
//     Uncommon,
//     Rare,
//     Epic,
//     Legendary,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(Rarity) => {
//             println!("Quarter: {:?}", Rarity);
//             25
//         }
//     }
// }

// fn main() {
//     let coin = Coin::Penny;
//     println!("The value of the coin is:{}", value_in_cents(coin));

//     let coin = Coin::Nickel;
//     println!("The value of the coin is:{}", value_in_cents(coin));

//     let coin = Coin::Quarter(Rarity::Epic);
//     println!("The value of the coin is:{}", value_in_cents(coin));

//     // Lucky penny!
//     // The value of the coin is:1
//     // The value of the coin is:5
//     // Quarter: Epic
//     // The value of the coin is:25
// }

// Match with Option<T>

// enum Option<T> {
//     Some(T),
//     None,
// }
fn main() {
    fn plus(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus(five);
    println!("{:?}", six);

    // Some(6)
}
