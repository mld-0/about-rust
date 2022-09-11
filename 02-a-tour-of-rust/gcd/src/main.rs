//  The 'use' declaration brings traits into scope
//  A trait is a collection of methods that types can implement.
//  A trait must be in scope in order to use its methods.
use std::io::Write;
use std::str::FromStr;

//  Variables are immutable by default
//  Use 'mut' to declare mutable variables.

fn gcd(mut n: u64, mut m: u64) -> u64 {
    //  '!' denotes a macro invocation
    //  Is always checked, regardless of compiler settings
    //  Use 'debug_assert!' for a macro that is skipped in release builds
    assert!(n != 0 && m != 0);
    //  Rust does not place parenthesis around loop/if conditions
    while m != 0 {
        if m < n {
            //  'let' is used to declare a local variable
            //  The type is infered automatically if not given
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    //  Rust has a return statement, but it is not needed here,
    //  if a function ends in a value without a ';', it is used as the return value
    //  Any block can return a value in this manner
    //  It is best practice to use 'return' for explicit early returns, and to use the trailing
    //  semicolon-less value as the <default> returned value
    n
}

//  We can declare tests in our source with test attribute:
#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
//  Run these tests with 'cargo test'

//  References and borrowing:
//      &x      borrow a reference to x
//      *r      value that r refers to

//  Rust assumes that if main returns at all, the program was successful 
//  Hence we do not specify the return type or return 0
fn main() {
    //  Declare a vector
    //  Its type will be infered from our use of it
    let mut numbers = Vec::new();

    //  'std::env::args' returns an iterator
    //  Its 'skip' method returns another iterator that skips a given number of values.
    for arg in std::env::args().skip(1) {
        //  'u64::from_str' returns a 'result' type, indicating the success of the operation.
        //  (Functions that perform IO or interact with the OS all return result types).
        //  Rust does not use exceptions, errors are handled either with result types or panic.
        //  We extract the value of the result type with 'expect', which prints an error and exists
        //  if the result indicates an error.
        //  <(The '&' indicates a borrow)>
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        //  Use the 'writeln' macro to write our error to stderr. Returns a result type.
        //  Use 'unwrap' instead of 'expect' when it is not neccessary to provide a custom message.
        writeln!(std::io::stderr(), "(Error) Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    //  Ongoing: 2022-09-11T22:16:17AEST is 'm' a reference we have to dereference because we are borrowing 'numbers', or is it a reference because that's how looping over a container works?
    //  <(The '*' indicates a dereference)>
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    //  Ongoing: 2022-09-11T22:26:01AEST need to unwrap writeln, but don't need to unwrap println?
    //  The 'println' macro substitutes format specifiers for arguments given and writes to stdout
    println!("The gcd of {:?} is {}", numbers, d);
}

