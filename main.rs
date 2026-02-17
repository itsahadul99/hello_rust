use crate::{
    collatz_length_fn::collatz_length_fn::collatz_length,
    fib_fn::fib_fn::{get_fib_result},
    variables::variables::hello_rust,
};
pub mod collatz_length_fn;
pub mod fib_fn;
pub mod variables;
fn main() {
    hello_rust();
    get_fib_result();
    collatz_length(6);
}
