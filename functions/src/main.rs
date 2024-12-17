/* A divergent function is a function that never return to the caller.
Functions always needs to have type annotated for their parameters.
*/

fn main() {
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y);
    println!("s:{}", s);

    div_function();

    println!("This line will not be printed!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn div_function() -> ! {
    panic!("Function doesn't return anything") // Immediately exist with an eror
                                               // unimplemented!(), if the function is not implemented
                                               // todo!(),
}
