// concise control flow with if let

// fn main() {
//     let config_max: Option<i32> = Some(100);
//     // match config_max {
//     //     Some(max) => println!("The max is configured to be {}", max),
//     //     _ => (), // ignore the case where config_max is None
//     // }

//     if let Some(max) = config_max {
//         println!("Max is: {}", max);
//     }
// }

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
fn main() {
    let coin = Coin::Quarter(Rarity::Epic);

    if let Coin::Quarter(rarity) = coin {
        println!("This quarter is a {:?}", rarity);
    } else {
        println!("This is not a quarter");
    }
}

// This quarter is a Epic
