/* ------------------Array------------------
Fixed-size collection of elements of the same data
type stored as contigous block in stack memory.

Signature of array is [T,length] which indicates that the
length is fixed at compile time.

Arrays can neither grow not shrink, they must retain their
size.

        SLICES
-----------------------
A slice reference to contigous sequence of elements in a collection.
It provides a way to borrow part of a collection without taking ownership
of the entire collection.
It can be created from arrays, vectors, Strings and other collections
implementing the Deref trait.
*/

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array is {:?} and the length is: {:?}", arr, arr.len());
    // array shorthand to fill
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    // safer method for array indexing
    let element = list.get(0).unwrap();
    println!("element is: {:?}", element);

    //-----Slices -------
    let slice = &list[1..3]; // needs to use the reference of the slice
    println!("slice is: {:?}", slice);
}
