fn interproduct(a: i16, b: i16, c: i16) -> i16 {
    return a * b * c;
}
fn main() {
    print!("Hello, world!\n");
    let x: i32 = 10;
    let n: u32 = 30;
    println!("x: {x}");
    print!("interproduct: {}\n", interproduct(-1, 2, 3));
    takes_i8(127); // -128 to 127
    takes_u32(4294967295); // 0 to 4,294,967,295
    print!("Fib: {}\n", fib(n));

    let size = if x >= 10 { "Big" } else { "Small" };
    print!("x: {x}\n");
    print!("Size: {size}\n");
    print!("-----------------------------\n");
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
    // short hand match
    let flag = true;
    let flag_val = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {flag_val}");
    let mut i = 0;
    loop {
        i += 1;
        print!("Hello {i}\n");
        if i % 2 == 0 {
            print!("Even number {i}\n");
        }
        if i % 10 == 0 {
            print!("Number multiple of 10: {i}\n");
        }
        if i >= 10 {
            break;
        }
    }
    println!("Length: {}", collatz_length(12));
}

fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n > 1 {
        print!("n: {n}\n");
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}
