/* -------------------VECTORS-----------------------
Vectors are like arrays, but they are dynamically sized.
Vectors are allocated on the heap as contigous block of memory.
All the elements in a vector must have the same type.
special macro: vec!

Capacity of a vector is the amount of space allocated for any future 
elements that will be added onto the vector. This is not same as the length
of the vector.
*/

fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    let v: Vec<u8> = Vec::from(arr);
    println!("The vector is: {:?}", v);
}
