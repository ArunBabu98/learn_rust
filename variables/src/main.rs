fn main() {
    // vriable x is 32 bit integer with a value of 5
    let x: i32 = 5;
    // For unused error removal, prepend a "_" to the variable
    // declared the type to mutable using "mut" keyword
    let mut y: i32 = 8;

    // Assertion, if not, quits else continues
    assert_eq!(x, 5);
    println!("Success");

    // adding to variable 'y' since it is mutable
    y += 2;

    assert_eq!(y, 10);
    println!("Success");

    // scope examples
    {
        let z: i32 = 4;
        println!("The value of x is {} and value of z is {}", x, z);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    define_s();

    // Shadowing
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("The value of x is now {}", x);

    // Destructuring
    let (mut a, b) = (1, 2); // this is destructuring
    a += 2;

    assert_eq!(a, 3);
    assert_eq!(b, 2);

    println!("value of a is {}, value of b is {}", a, b);

    // Destructuring assignments
    let (l, m);
    (l, ..) = (3, 4);
    [.., m] = [1, 2];

    assert_eq!([l, m], [3, 2]);

    println!("value of l is {} and value of m is {}", l, m);
}

fn define_s() {
    let s: &str = "This is another function!";

    println!("{}", s);
}
