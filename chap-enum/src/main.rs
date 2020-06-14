struct IpV4 {}

struct IpV6 {}

enum IP {
    V4(IpV4),
    V6(IpV6),
}

enum Message {
    Quit,
    Move,
    Write,
    Change,
}

impl Message {
    fn call(&self) {
        println!("A Message is called");
    }
}

//enum Option<T> {
//    Some(T),
//    Other,
//}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("You got this state: {:?}", UsState);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => Some(1),
    }
}

fn main() {
    let home = IP::V4;
    let mess = Message::Write;
    mess.call();

    let some1 = Option::Some(5);
    let some2 = Some("A string");

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //    let some_u8_value = 0u8;
    //    match some_u8_value {
    //        1 => println!("one"),
    //        3 => println!("three"),
    //        5 => println!("five"),
    //        7 => println!("seven"),
    //        _ => (),
    //    }
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("Three");
    }

    let coin1 = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Penny;

    let mut count = 0;

    if let Coin::Quarter(state) = coin2 {
        println!("State from {:?}", state);
    } else {
        count += 1
    }
}
