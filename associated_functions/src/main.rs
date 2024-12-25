/* ------------------Associated Types---------------------
It allows to specify a type that is associated with the trait.
WHen implementing the trait for a specific type we have to specify the concrete type
Basically a type placeholder that the trait methods can use in signature
similar to generic types, but are more flexible because they allow a trait
to have different associated types for different implementing types.
*/

trait MyTrait {
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct {}

impl MyTrait for MyStruct{
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}

fn main() {
    println!("");
}
