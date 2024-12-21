/* Tuple is a way to store related pieced of information in a single variable
It is a collection of different types grouped tof=gether as a single compound value.
It is stored as a fixed-size contigous block of memory on the stack.
SIgnature is (T,T,T,..)

NOTE
-------
1) long tuples cannot be printed [upto 12 elements can be printed on screen]
*/

fn main() {
    let t0: (u8, i16) = (0, -1);
    let t1: (u8, (i16, u32)) = (0, (-1, 1));
    println!(" t0: {:?}, t1: {:?}", t0, t1);
    println!("{:?}", t0.1);

    // destructuring tuple
    let t: (i32, f64, &str) = (1, 6.4, "hello");
    let (x, y, z) = t;
    println!("x:{:?},y:{:?},z:{:?}", x, y, z);
}
