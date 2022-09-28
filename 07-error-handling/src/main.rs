//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
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
    //  When rust panics, it prints an error message, as well as 
    //  'note: Run with `RUST_BACKTRACE=1` for a backtrace'
    //  (this variable must be exported)

    //  <(Stack unwinding is like C++ exception handling)>
    //  All temporary/local/argument values are dropped, in the reverse order they were created
    //  The current function is cleaned up, then its caller, and so on, then the thread exits 
    //  If the panic occurs in the main thread, the whole process exists with a nonzero error code

    //  Panic behaviour is safe and well defined. It will never leave a dangling pointer or half-initialized memory

    //  Stack unwinding can be caught with 'std::panic::catch_unwind()'
    //  (note that not all panics result in stack unwinding)

    //  Unwinding across non-Rust code is undefined behaviour

    //  <(Stack unwinding is default panic behavior)>

    //  If '.drop()' triggers a second panic during stack unwinding, the process is aborted

    //  The '-C panic=abort' compile option aborts the program immediately on panic
    //  (this reduces code size)

    println!("example_unwinding, DONE");
}




fn main() 
{
    example_unwinding();
}

