/* ------------------------TRAITS-------------------------
It is a set of methods that can be implemented for multiple types in
order to provide common functionality and behaviour between them.
Traits consists of method signature only, which then have to be implemented
by the target type.
Similar to "classes" in other languages but no the same.
It defines shared behaviour in an abstract way.

Derivable Traits
-----------------

A trait that can be automatically implemented for a struct or an enum by the
compiler.
Most common derivable traits are:
1) Debug - allowing to output content via {:?}
2) Clone - enables type to be duplicated.
3) Copy - enable type to be copied implicitly without clone()
4) PartialEq - enables comparison

Traits as Parameters
----------------------
Traits can be used a parameters for functions

Trait Objects
----------------------
Using 'impl Traait' doesn't work when returning multiple types.
Different implementations of a trait probably use different amounts of memory
but sizes of types must be known at compile time.
In this case, trait objects can be used.
A trait object is essentially a pointer to any type that implements that given
trat, where the precise type can only be known at runtime.

Static Dispatch
------------------
1. It resolves method calls at compile time
2. Compiler generates function code for each concrete type that implements trait
3. Calls appropriate function based on concrete types
4. Faster and more efficient than dynamic dispatch, but doesn't provide great
flexibility.

Dynamic Dispatch
------------------
1. Specific methods to be called is determined at runtime
2. it works by creating a reference or smart pointer to a trait object using '&dyn'
or 'Box<dyn>'
3. WHen trait object is created, compiler will build a vtable for that trait
4. vtable is a table that contains a pointer to the implementation of each method in the trait
for the specific type of the object that the reference points to.
5. Compiler will do a lookup in the vtable to determine which method should be called
for which type that implements a given trait.
6. This lookup will cause overhead but allows for more flexible code.

BOX
---------

A box is a smart pointer that allows to store data on the heap rather than the stack
Use box when you have a type whose size can't be known at compile time
returns a pointer to the data stored on the heap.

Difference between & and Box
------------------------------

1. Box allocated data on heap and owns it, it is als responsible for deallocation.
reference only points to a value already in memory.
2. Box can be passed across scopes, a reference has a limited lifetime.
3. Box can be cloned but reference not.
4. Box can be used in pattern matching

*/

use std::fmt::{Debug, Display};

pub trait Animal: Debug {
    fn sound(&self) -> String;
}

#[derive(Debug)]
struct Sheep;
#[derive(Debug)]
struct Cow;

#[derive(Debug)]
struct Dog;
#[derive(Debug)]
struct Cat;

impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("mahhh")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Moo")
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meow")
    }
}

impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("woof")
    }
}
fn main() {
    let cow = Cow;
    let sheep = Sheep;
    println!("1->Cow:{:?} and Sheep:{:?}", cow.sound(), sheep.sound());
    let cow_sound = get_sound(&cow);
    let sheep_sound = get_soundt(&cow);
    println!("2->Cow:{:?} and Sheep:{:?}", cow_sound, sheep_sound);

    let animal1 = return_animal("cat");
    let animal2 = return_animal("dog");

    println!("Cat: {:?} and Dog: {:?}", animal1, animal2);

    let random_number: f64 = 0.235;
    let animal = random_animal(random_number);
    println!("The random animal is: {:?}", animal.sound())
}

// traits as parameters
pub fn get_sound(item: &impl Animal) -> String {
    item.sound()
}

// trait bounds
pub fn get_soundt<T: Animal>(item: &T) -> String {
    item.sound()
}

// If you have a function that makes heavy use of trait bounds,
// we can use a where clause to make the code cleaner.

fn some_func<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // some code
}

// using reference
fn return_animal(s: &str) -> &dyn Animal {
    match s {
        "dog" => &Dog {},
        "cat" => &Cat {},
        "cow" => &Cow {},
        "sheep" => &Sheep {},
        _ => panic!(),
    }
}

// using box
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
