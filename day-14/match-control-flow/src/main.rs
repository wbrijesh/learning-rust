enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coins: Vec<Coin> = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let mut total_value_in_cents: u8 = 0;

    for coin in coins {
        total_value_in_cents += value_in_cents(coin);
    }

    println!("Total value in cents is {}", total_value_in_cents);
}
