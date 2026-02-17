pub fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

pub fn get_fib_result() {
    let result = fib(10);
    println!("Fibonacci result: {result}");
}
