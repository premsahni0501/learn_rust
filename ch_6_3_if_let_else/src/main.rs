#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Querter(UsStates),
}
#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
}
fn main() {
    let config = Some(3u8);

    match config {
        Some(i) => println!("config: {i}"),
        _ => (),
    }

    let config2 = Some(32);

    if let Some(max) = config2 {
        println!("config 2: {max}");
    }
    let mut count: u8 = 34;
    let v = value_in_cents(Coin::Querter(UsStates::Alabama), count);
    count = value_in_cents(Coin::Dime, v);
    println!("v: {v}, count: {count}");
}

fn value_in_cents(coin: Coin, count: u8) -> u8 {
    match coin {
        Coin::Querter(state) => {
            println!("State quarter from {state:?}");
            25
        }
        _ => count + 1,
    }
}
