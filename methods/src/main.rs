/* -----------------METHODS-----------------------
Method are functions that are associated with a particular type or struct.
It takes paramters and return a value, but defined as a memeber of a struct or
enum.
It is called using dot notation.
It is implemented through an "impl" block.

Associated Functions
---------------------
These are functions that are associated with a struct or am enum, but does not
take an instance as its first parameter.
It is called using the name of the type, not an instance of it.
It is often used as constructors for a struct or enum.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(300, 500);
    println!("Area of the rectangle is {:?}", rect1.area());
}
