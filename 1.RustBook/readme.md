# Rust Basics

- Variables are immutable by default in Rust.
- On the other hand, constants `const` can never be mutated. Type must be annotated for constants. Nmaing convention for constants, is to use all upper case letter with underscore in between.
- Shadowing is another concept in Rust, where a variable name can be reused.
- Rust is a statically typed language.
- Rust has 4 major scalar types, Integers, Floating point numbers, Booleans and characters.
- Rust has two primitive compund types, Tuples and Arrays.
- main() function is the entry point of the Rust program.
- To return a value from a function, simple omit the semilcolon after writing the value that needs to be passed back.
- Ownership is a unique feature of Rust, it gauratees memory safety without needing a garbage collector.
- Ownership is a set of rules that govern how a Rust program manages memory.


### Ownership Rules

Ownership is a set of rules that govern how Rust manages memory. Memory is managed through a system of ownership with a set of rules that the compiler checks.

1) Each value in Rust has an owner
2) There can only be one owner at a time
3) When the owner goes out of scope, the value will be dropped.


- A crate is the samllest amount of code that the RUst compiler considers at a time. A crate can be binary crat.e or a library crate
- A package is a bundle of one or more crates that provides a set of functionality.

### Data Types

There are two types of data types : Scalar and Compound

A sacalar type represents a single value. 
1. Integers - i8 to i128 and isize
2. Floating - f32 and f64
3. Boolean
4. Characters

Compound types can gorup multiple values into one type.
1. Tuples - fixed lenth/heterogenous
2. Arrays - fixed length/homogenous