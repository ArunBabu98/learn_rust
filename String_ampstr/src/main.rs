/* ----------------------String vs &str---------------------
A 'String' is a heap-allocated string type that owns its content and is mutable.
A '&str' is an immutable sequene of UTF-8 bytes in memory, it doesn't own the
underlying data and is immutable.

Use &str if you just want to a view of a string
&str is more lightweigth and efficient than String
Use String when you want to own the data and be able to mutate it.

String Literal is a sequence of characters enclosed in double quotes.
It is fixed size, compile time known sequence of UTF-8 bytes.

*/
fn main() {
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{:?}", hello);
    println!("{:?}", world);

    let s1: Box<str> = "hello world".into(); // into() converts to the target type. allocates to heam memory because of BOX.
    greetings(&s1); // &s1 is string slice

    let s2: String = String::from("I love dogs");
    // allocates new memory and store the modified string there
    let s3 = s2.replace("dogs", "cats");

    println!("s3:{:?}", s3);
}

fn greetings(s: &str) {
    println!("{:?}", s);
}
