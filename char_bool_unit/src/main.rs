use std::mem::size_of_val;

fn main() {
    let c1: char = 'a'; // double quotes are for stings and single quotes are for charcters
    println!("{}", size_of_val(&c1)); // will take 4 bytes ini memory
    printchar(c1);

    let f: bool = false;
    if !f {
        println!("Success!")
    }

    // Unit type - It is a type that doesn't hold any value, and its size is 0 byes
    // Usually if a function doesn;t return a value, a unit type is returned

    let _v: () = (); // unit type
    implicitly_ret_unit();
}

fn printchar(c: char) {
    println!("{}", c);
}

// it will implicity return unit type
fn implicitly_ret_unit() {
    println!("I wil return a ()");
}
