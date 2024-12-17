/* A statement is an instruction that performs some action but do
not produce a value. On the other hand, an expression evaluates to
a resultant value.

*/

fn main() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x // the resultant of this expression is assigned to y.
    };

    let z: u32 = { 2 + x };

    println!("x:{:?}", x);
    println!("y:{:?}", y);
    println!("z:{:?}", z);

    let s: i32 = sum(2, 3);
    println!("s:{:?}", s);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // no semicolon if the function needs to have a return type
}
