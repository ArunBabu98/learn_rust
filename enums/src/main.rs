/* --------------------------ENUM-------------------------
It is a way of defining a type with only one of a possible set of values.
We can only access one variant of an enum at a time
It can hold additional information using tuple.
It is especially useful when using in match statements

OPTION ENUM
--------------

***THERE IS NO NULL IN RUST*****
To deal with this, enum Option<T> is used.

1) Option is an enum that represents a value that may or may not be present.
2) This is used to handle cases where a function or method might fail to return a value.

*/

fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    enum Number {
        Zero = 5, // setting descriminator as 5, default starts from 0
        One,      // 6, implicitly
        Two,      // 7, implicitly
    }

    println!("{:?}", Number::One as u8);

    // Each enum variants can hold its own data

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Move { x: 2, y: 3 };
    let msg2 = Message::Write(String::from("Hello"));

    println!("{:?},{:?}", msg1, msg2);

    // -------------Option Enum-------------------
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{:?}", n);
    } else {
        panic!("Never let this run!");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
