#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 20,
        width: 50,
    };

    println!("rect1 is {rect1:?}");

    println!(
        "The area of the rectangle is {} square pixels,",
        rect1.area(),
    )
}
