//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-07T00:04:03AEDT 'print_error()' function -> changes to Rust since book(?) 1. need 'dyn' keyword, 2. use 'source()' instead of deprecated 'cause()'
//  Ongoing: 2022-10-07T00:18:39AEDT defining 'GenError' again (unlike the book) we need the 'dyn' keyword?
//  Ongoing: 2022-10-07T00:25:57AEDT 'error.downcast_ref::<ErrorType>()' example?
//  Ongoing: 2022-10-07T00:36:46AEDT 'example_results()' should be multiple functions, (here 'result' being shorthand for (method of) dealing with errors) -> a function for each error handling subtopic(?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

//  Rust panics for errors that cannot be handled
//  A panic indicates a bug in the program
//  Causes of panics include:
//      Out of bounds array access
//      Integer division by zero
//      Calling '.unwrap()' on a None Option
//      Assertion failures
//  A panic can be triggered manually with the 'panic!()' macro
//  Rust can handle panics either by unwinding the stack, or aborting the process


fn example_unwinding()
{
    //  <(Stack unwinding is Rust's default panic behaviour)>

    //  When rust panics, it prints an error message, as well as 
    //  'note: Run with `RUST_BACKTRACE=1` env variable for a backtrace'
    //  (this variable must be exported)

    //  <(Stack unwinding is like C++ exception handling)>
    //  All temporary/local/argument values are dropped, in the reverse order they were created
    //  The current function is cleaned up, then its caller, and so on, then the thread exits 
    //  If the panic occurs in the main thread, the whole process exists with a nonzero error code
    //  Panic is safe. It does not violate any of Rust's safety rules.

    //  Panic behaviour is safe and well defined. It will never leave a dangling pointer or half-initialized memory

    //  Stack unwinding can be caught with 'std::panic::catch_unwind()'
    //  (note that not all panics result in stack unwinding)

    //  Unwinding across non-Rust code is undefined behaviour

    //  There are <(only?)> two circumstances under which Rust does not try to unwind the stack:
    //  First: If '.drop()' triggers a second panic during stack unwinding, the process is aborted
    //  Second: The '-C panic=abort' compile option aborts the program immediately on panic (this reduces code size)

    //  Ordinary Rust code is under no obligation to handle panic

    println!("example_unwinding, DONE");
}


//  On the use of Results:
//  Rust requires a decision from the programmer anywhere an error can occur
//  The most common decision is to propogate an error, Rust makes this seemless with the '?' operator
//  The possibility of errors is part of a function's return type
//  Rust checks to ensure Result values are used (preventing silent passing of an error)
//  Success/failure results can be stored in a single container of Result type (allowing modeling of partial success)


//  Rather than use exceptions, Rust functions that can fail return a Result type
use std::io;
struct WeatherReport {}
fn display_weather(report: &WeatherReport) {}
fn get_weather(lat: f32, long: f32) -> Result<WeatherReport, i32> { 
    if lat == 0.0 && long == 0.0 { return Err(0); }
    return Ok(WeatherReport{}); 
}
//  This function either returns 'OK(WeatherReport)' or 'Err(i32)'

fn example_results_manage_errors()
{
    //  Result vs Option:
    //  Result is for functions that are expected to succeed
    //  Option is for functions that may/may-not return a value

    //  Calling this function requires us to write error handling.
    //  We cannot use the result without checking whether it is an error,
    //  and if we do not use the result, we get a warning.

    //  The most common way of dealing with a Result is a match expression
    match get_weather(125.73, 19.51) {
        Ok(report) => {
            display_weather(&report);
        }
        Err(err) => {
            println!("get_weather, err=({})", err);
        }
    };
    //  (this is Rust's equivalent to the try/catch)

    //  Result<T,E> Methods:
    //  Return success value, or panic on error 
    //      result.unwrap()
    //      result.expect(message)
    //  Success/error value as Option<T>
    //      result.ok()
    //      result.err()
    //  Returns success value, or 'fallback' (disregarding error value)
    //      result.unwrap_or(fallback)
    //  <(Returns success value or calls 'fallback_fn')>
    //      result.unwrap_or_else(fallback_fn)

    //  Methods that do not consume Result:
    //  Bool indicating success/failure:
    //      result.is_ok()
    //      result.is_err()
    //  Convert 'Result<T,E>' to 'Result<&T,&E>'
    //      result.as_ref()
    //  Convert 'Result<T,E>' to 'Result<&mut T, &mut E>'
    //      result.as_mut()

    //  <(Result<()> denotes a Result type alias)>

    println!("example_results_manage_errors, DONE");
}

fn example_results_printing() 
{
    //  Printing errors:
    //  The standard library defines several error types: std::io::Error, std::fmt::Error, std::str::Utf8Error, ect.
    //  These all implement the 'std::error::Error' trait'
    //  Use '{}' with 'println!()' for a brief error message, or '{:?}' for a detail error message
    //  Use 'err.description()' to get the error message as a '&str'
    //  Use 'err.cause()' to return 'Option<&Error>', to get the underlying error, if any
    //  <('err.cause()' is deprecated, use 'err.source()')>
    use std::error::Error;
    use std::io::{Write,stderr};
    //fn print_error(mut err: &Error) {
    //fn print_error(err: &mut Error) {
    fn print_error(mut err: &dyn Error) {
        //  (any error that occurs while writing initial error gets ignored)
        let _ = writeln!(stderr(), "error: {}", err);
        while let Some(cause) = err.source() {
            let _ = writeln!(stderr(), "caused by: {}", cause);
            err = cause;
        }
    }
    //  Standard library errors do not include a stack trace, but the error-chain/stacktrace crates makes it easy to create custom error type which do

    println!("example_results_printing, DONE");
}

fn example_results_propogating() 
{
    //  Propogating errors:
    //  Use the '?' operator to propogate an error - leave it to the caller to deal with
    //let weather = get_weather(53.9, 61.4)?;
    //  On success, it unwraps the Result
    //  On error, it immediately returns the error up the call chain
    //  (Can only be used in functions that have a Result return type)
    //  (Use '.unwrap()' instead if failure is not expected)

    //  The same thing can be achieved through a match expression
    //let weather = match get_weather(53.9, 61.4) {
    //    Ok(success) => success,
    //    Err(err) => return Err(err)
    //};

    //  Older code may use 'try!()' instead of '?'
    //let weather = try!(get_weather(53.9, 61.4));
    //  (expands to match expression above)

    //  Some functions may use the '?' operator on practically ever line:
    use std::fs;
    //use std::io;
    use std::path::Path;
    fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
        for entry_result in src.read_dir()? {
            let entry = entry_result?;
            let dst_file = dst.join(entry.file_name());
            fs::rename(entry.path(), dst_file)?;
        }
        Ok(())
    }

    println!("example_results_propogating, DONE");
}

fn example_results_multiple_types() 
{
    //  Working with multiple error types

    //  We can define a GenResult type:
    type GenError = Box<dyn std::error::Error>;
    type GenResult<T> = Result<T,GenError>;
    //  This allows us to return any error type
    //  However, the information about what kind of errors may be returned is not communicated this way

    //  To convert any error type into a GenError, use 'GenError::from()':
    let io_error = io::Error::new(io::ErrorKind::Other, "timed out");
    let gen_error = GenError::from(io_error);
    //  <(The '?' operator does this automatically)>

    //  <(To handle a specific type of error, and propogate any other, use 'error.downcast_ref::<ErrorType>()')>


    //  Dealing with errors that "Can't happen"
    //  The '?' operator tells the compiler that the error in question can be returned
    //  Use '.unwrap()' instead where an error is not expected (in which case an error will cause a panic)


    //  Ignoring errors:
    //  Use the 'let _ = ...' idiom to silence warnings about unused Result values 

    println!("example_results_multiple_types, DONE");
}

fn example_results_errors_in_main() 
{
    //  Handling errors in 'main()'
    //  main cannot use '?' because its return type is not a Result.
    //  The simplest alternative is to use '.unwrap()' or '.expect(msg)'

    //  Alternatively we may wish to print the error ourselves
    if let Err(err) = get_weather(5.3, 9.7) {
        println!("err=({})", err);
        std::process::exit(1);
    }

    println!("example_results_errors_in_main, DONE");
}

fn example_results_custom_errors() 
{
    //  Custom error types
    #[derive(Debug, Clone)]
    pub struct JsonError {
        pub message: String,
        pub line :usize,
        pub column: usize,
    }
    use std::fmt;
    //  Errors should be printable
    impl fmt::Display for JsonError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{} ({}:{})", self.message, self.line, self.column)
        }
    }
    //  Errors should implement 'std::error::Error'
    impl std::error::Error for JsonError {
        fn description(&self) -> &str {
            &self.message
        }
    }
    fn return_jsonerror() -> Result<(),JsonError> {
        return Err(JsonError { message: "expected <>".to_string(), line: 1, column: 2 } );
    }

    println!("example_results_custom_errors, DONE");
}


fn main() 
{
    example_unwinding();
    example_results_manage_errors();
    example_results_printing();
    example_results_propogating();
    example_results_multiple_types();
    example_results_errors_in_main();
    example_results_custom_errors();
}

