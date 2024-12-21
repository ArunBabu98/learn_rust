/* STruct is a compound type allowing to group together values of different types
into a named datat structure.

It is similar to tuples, but each value has a name, so the values can be accessed
through this name

Struct needs to be instanciated with data.

TUPLE STRUCTS
--------------
1. Tuple like syntax for stucts
2. It is basically a named tuple
3. Accessed through point notation

Unit-like STRUCTS
------------------
1. Structs without any field
2. Used when working with traits
3. It doesn't store any data
*/

fn main() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("Username"),
        email: String::from("username@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("mutated@gmail.com");

    println!(
        "{:?},{:?},{:?}",
        user1.username, user1.active, user1.sign_in_count
    );

    // Struct update syntax
    let user2 = User {
        username: String::from("username2"),
        ..user1 // user1 values - > user2 values
    };

    println!(
        "{:?},{:?},{:?}",
        user2.username, user2.active, user2.sign_in_count
    );

    // Tuple Structs
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

    println!("Black: {:?}:{:?}:{:?}", black.0, black.1, black.2);

    // To make a struct printable, use #[derive(Debug)]
    println!("{:?}", user2);
}
