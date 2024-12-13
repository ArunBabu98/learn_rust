/*

LENGTH               SIGNED                 UNISIGNED
8-bit                i8                     u8
16-bit               i16                    u16
32-bit               i32                    u32
64-bit               i64                    u64
128-bit              i128                   u128
arch                 isize                  usize

Defaults

Integer: i32
Floats:  f64

*/

fn main() {
    let x: i32 = 5;
    let mut _y = 5;

    _y = x;

    let z = 10;

    println!("x:{} y:{} z:{}", x, _y, z);

    // 38 which has been declared as u8 is converted u16 to variable why using 'as' keyword
    let h: u16 = 38_u8 as u16;

    println!("h:{}", h);

    let g: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&g));
    println!("g: {}", g);

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    // unsafe way to call
    let v1 = 251_u16 + 8;
    // safe way to call
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("v:{}", v);

    let p = 1_000.000_1;
    let _j: f32 = 0.12;
    let _n = 0.01_f64;

    assert_eq!(type_of(&p), "f64".to_string());

    // floating point prescision
    //1
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    //2
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
    printrange();
}

// Get the type of a given variable, return a string representation of the type
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn printrange() {
    let mut sum = 0;

    for i in -3..2 {
        sum += i;
    }

    println!("sum: {}", sum);

    for c in 'a'..='z' {
        println!("char: {}", c as u8)
    }
}
