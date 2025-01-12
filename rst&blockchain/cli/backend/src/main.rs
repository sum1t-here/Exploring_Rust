use tokio;
use std::{ time };

async fn print_odd() {
    let total = 10;
    let mut count = 0;
    let mut odd = 1;
    let three_sec = time::Duration::from_secs(3);

    while count < total {
        println!("{}", odd);
        count += 1;
        odd += 2;
        tokio::time::sleep(three_sec).await;
    }
}

async fn print_even() {
    let total = 10;
    let mut count = 0;
    let mut even = 2;
    let three_sec = time::Duration::from_secs(3);

    while count < total {
        println!("{}", even);
        count += 1;
        even += 2;
        tokio::time::sleep(three_sec).await;
    }
}

#[tokio::main]
async fn main() {
    let odd = print_odd();
    let even = print_even();
    println!("Begin counting ....");
    // join is a function that wait for the two tasks to finish
    tokio::join!(odd, even);
    println!("Done counting ....");
}
