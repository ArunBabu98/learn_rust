/* ----------------TYPE COERCION--------------------
Type conversion also called type casting is coercing primitice types
that can be performed by 'as' keyword.

'as' conversions can be chained.

WHen casting to an unsigned type T, T::MAX+1 is added or subtracted until
the value fits into the new type.

Using unsafe methods can lead to undefined behaviour.

From/Into conversion
---------------------
These traits are used for type conversions between different types without
requiring explicit casts.
It can be implemented for custom types.
Implementing From for a type will give us Into implementing for the the given
type for free.
*/

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number { value: n }
    }
}
fn main() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;

    let c1: char = integer as char;
    println!("{}", c1);

    let num: Number = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);
}
