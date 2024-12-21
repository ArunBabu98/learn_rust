/* Normal flow of a program: top to bottom, line by line
Flow control is a concept that refers to the ability to control the order in which
statements or instrictions are executed in a program.
Allows to specify which instructions should be executed under which conditions
and in what order.

1) Conditionals
    a) if/else
    b) match
2) Loops
    a) for/while/loop
    b) continue/break
*/

fn main() {
    let n: i32 = 5;
    if n < 0 {
        println!("{:?} is negative", n);
    } else if n > 0 {
        println!("{:?} is positive", n);
    } else {
        println!("{:?} is Zero", n);
    }

    // if/else expression can be used in assignments
    let n: i32 = 100;
    let big_n: i32 = if n < 10 && n > -10 {
        println!("increase ten-fold");
        10 * n
    } else {
        println!("halve the number");
        n / 2.0 as i32
    };

    println!("big_n is :{:?}", big_n);

    // for in construct can be used to iterate through an iterator
    for n in 1..100 {
        if n == 99 {
            println!("99 is found!");
        }
    }

    let numbers = [1, 2, 3];
    for num in numbers {
        println!("The number is: {:?}", num);
    }
    // iterating and indexing
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("The {:?}th element is {:?}", i + 1, v)
    }
    // loop is usually used together with break or continue.
    // It is a special loop in rust that goes till infinity and needs a break or continue
    let mut count: u32 = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Three found!");
            continue;
        }
        println!("{:?}", count);
        if count == 5 {
            println!("That's enough");
            break;
        }
    }

    // loop is an expression, so we can use it with break to return a value

    let mut cnt: u32 = 0;

    let result: u32 = loop {
        cnt += 1;
        if cnt == 20 {
            break cnt;
        }
    };
    println!("The result is: {:?}", result);

    // nested loop needs a labal

    let mut i_cnt: i32 = 0;

    'outer: loop {
        'inner: loop {
            if i_cnt >= 20 {
                break 'inner;
            }
            i_cnt += 2;
        }
        i_cnt += 5;
        break 'outer;
    }

    println!("i_cnt is: {:?}", i_cnt);
}
