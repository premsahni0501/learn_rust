#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Querter(UsStates),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Querter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let value = value_in_cents(Coin::Querter(UsStates::Alabama));
    println!("Coin value: {value}");

    let some_num = 43;
    println!(
        "Add 1 to some({some_num}): {0:#?}, add some to none: {1:#?}",
        plus_one(Some(some_num)),
        plus_one(None)
    );

    let dice_roll = 8;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(1),
    }
}

fn add_fancy_hat() {
    println!("Added fancy hat");
}
fn remove_fancy_hat() {
    println!("Added fancy hat");
}
fn move_player(spaces: u8) {
    println!("Move player by {spaces} spaces");
}
