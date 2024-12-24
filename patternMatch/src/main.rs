/* -------------Pattern Match------------------
It is a powerful construct that allows to compare a value against a set
of patterns, then execute different code based on which patterns matches.
The patterns can be made up of literal values, variable names, wildcards, etc..
I match, all possible cases must be handled, enforced by the compiler.

To reduce the boilerplate caused by mathc, we use if-let to unwrap a value of an
Option type.
*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("value is {:?}", value);

    let config_max = Some(3u8);

    // match statement
    match config_max {
        Some(max) => println!("The max is configured to be {:?}", max),
        _ => (),
    }

    // if-let statement
    if let Some(max) = config_max {
        println!("The max is configured to be {:?}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
