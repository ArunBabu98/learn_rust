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
*/

use std::fmt::{Debug, Display};

pub trait Animal {
    fn sound(&self) -> String;
}

#[derive(Debug)]
struct Sheep;
#[derive(Debug)]
struct Cow;

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

fn main() {
    let cow = Cow;
    let sheep = Sheep;
    println!("1->Cow:{:?} and Sheep:{:?}", cow.sound(), sheep.sound());
    let cow_sound = get_sound(&cow);
    let sheep_sound = get_soundt(&cow);
    println!("2->Cow:{:?} and Sheep:{:?}", cow_sound, sheep_sound);
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

fn some_func<T, U>(t: &T, u: &U) -> u32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // some code
}
