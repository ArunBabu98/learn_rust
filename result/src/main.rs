/* ---------------------RESULT--------------------------
Result is an enum type that represents the outcome of an operation that
could potentially fail.

Two possible variants:
1) Ok(T): A value T was found
2) Err(e): An error was found with a value e

The expected outcome is Ok, the unexpected outcome is Err.
Match pattern can be used since it's an enum.


    UNWRAP
----------------
The unwrap() methos takes as input a value of type Result and takes out the
value which is wrapped inside Ok(T) in case of success or panics in case of
an error.

    ?
----------------

The '?' operatir is shorthan way to propogate errors or unwrap Ok() results.
Basically the same as Unwrap() but instead of panic, returns an error.
It replaces an entire match statement.
It can be used in the main() function

    Type Alias
-----------------
It is a way of giving a new name to an existing type.
*/

fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.00 {
        return Err("Division by Zero");
    }

    Ok(x / y)
}

type U64 = u64;

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }

    let number: U64 = 42; // calling alias
}
