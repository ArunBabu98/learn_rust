/* -----------------BORROWING-----------------
It is a way of temporarily accessing data without taking the
ownership of it. WHen borrowing, we take the reference (pointer)
to the data and not the data itself. It's role is to prevent
dangling pointers and data races

Data can be borrowed immutabily and mutably.

Borrowing Rules
----------------
1. At any given time, you can have either one mutable reference or any
number of immutable references.
2. References must always be valid.

*/

fn main() {
    let s1 = String::from("Hello"); // s1 is not deallocated since we are only passing a reference of s1

    let len = calculate_length(&s1);
    println!("The length of '{:?}' is {:?}", s1, len);

    // mutable reference
    let mut s = String::from("Hello");
    change(&mut s); // mutable reference[s can only have one mutable reference at a time]
    println!("s is {:?}", s);

    let x = 5;
    let p: &i32 = &x; // reference
    println!("The memory address of x is : {:p}", p);
    let y = *p; // dereference
    print!("y is {:?}", y);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}
