/* ---------------------Generics--------------------------
These are placeholders for cocrete types.
It enables writing more reusable and flexible code.
It avoids having duplicate code for different types.
Zero cost abstraction, Rust compiler will at compile time find out generics
with concrete types.

const generics
---------------
Type parameter that represents a compile-time constant value.
It allows to write generic code that operates on values that are known at
compile time.
Used for array sizes, bit widths and other constants.
*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 3.5 };
    let diff = Point2 { x: 1.5, y: 2 };
    println!(
        "First point is :{:?}, Second point is: {:?} and the third
        point is : {:?}",
        integer, float, diff
    );
}
