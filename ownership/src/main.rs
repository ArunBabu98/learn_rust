/* ------------------OWNERSHIP---------------------
It is a set of rules that govern memory management. These rules are enforced
at compile time. If any of these rules are violated, the program won't run.

THREE RULES OF OWNERSHIP
------------------------
1) Each value in Rust had an owner
2) There can onle be one owner at a time
3) When the owner goes out of scope, the value will be dropped.

The owner of a value is the variable or data structure that holds it and is
responsible for allocating and freeing memory used to store that data.

Ownership prevents memory safety issues:

1. Dangling pointers
2. Double-free: Trying to free the memory that has already been freed
3. Memory leaks: Not freeing memory that should have been freed
*/

fn main() {
    let s = String::from("Hello!");
    takes_ownership(s); // Here, Since String allocates to Heap memory, s is moved
                        // println!("{:?}", s); // will throw error
    let x = 5;
    makes_copy(x); // x, is in stack cause it's a fixed tye, hence it only copies and can be reused
    println!("{:?}", x);

    let s1 = gives_ownership();
    println!("s1:{}", s1);

    let s2 = takes_and_gives_back(s1);
    println!("s2:{}", s2);
}

fn takes_ownership(some_string: String) {
    // comes into scope
    println!("{:?}", some_string);
} // goes out of scope

fn makes_copy(some_integer: i32) {
    // comes into scope
    println!("{:?}", some_integer);
} // goes out of scope

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
