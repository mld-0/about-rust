//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-21T22:52:06AEDT add (all items) "func_name: DONE" messages?
//  }}}

//  Continue: 2023-02-21T22:52:40AEDT complete chapter.

//  Rust has a tiered error handling scheme:
//          Option when something might be absent
//          Result when something that can be handled goes wrong
//          panic! when something that cannot be handled goes wrong
//          abort! when something catastrophic happens

//  Option / Result can later be promoted into panic or abort at the callers discression

//  <(A panic can only be caught by the parent thread)>

//  If the main thread panics, all threads are terminated and the program returns rc=101


#[test]
fn panic()
{
    //  LINK: https://doc.rust-lang.org/std/macro.panic.html
    //  {{{
    //  }}}

    //  LINK: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
    //  {{{
    //  }}}
}


#[test]
fn unwinding()
{
    //  A panic can either unwind or abort

    //  Stack unwinding is the process of destroying local variables in the event of an error
    //  This continues, stack frame by stack frame, until exception handler code is reached

    //  <(Examination of stack frames by debugging tool? ()>
    //  (Set 'RUST_BACKTRACE=1' to display trace of stack unwinding)

    //  Enabling unwinding will generate extra code to handling unwinding and can inhibit optimisations

    //  LINK: https://lucumr.pocoo.org/2014/10/30/dont-panic/
    //  {{{
    //  panicking can be problematic to deal with

    //  First choice should be to write code that is guaranteed to not panic (unless there is no choice, i.e: out of memory)

    //  contention: Modern OS make it difficult to end up in a situation where an allocation fails

    //  One option is always to disable unwinding and just let the thread abort on a panic

    //  contention: the best way to isolate failures is on the OS level through seperate processes
    //  }}}
}


#[test]
fn catch_unwind()
{
    //  'catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R>'
    //  Invoke a closure, returning a Result containing the cause of the resulting panic if one occurs
    //  Only catches panics which unwind (not those that abort)
    //  Not meant to be used as a general try-catch mechanism

    //  The closure is required to be 'std::panic::UnwindSafe'
    //  Indicates type will not <easily?> cause broken invariants when used with 'catch_unwind()'
    //  (automatically implemented for many types)

    //  Example: 
    use std::panic;
    let result = panic::catch_unwind(|| { println!("hello"); });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| { panic!("oh no"); });
    assert!(result.is_err());

    //  Unwinding is slow, as it is not meant to be used under normal circumstances

    //  It is UB to unwind from Rust code into foreign code
    //  (use 'catch_unwind' to catch any panics <at/before> the FFI boundry)
}


#[test]
fn exception_safety()
{
}


#[test]
fn poisoning()
{
}
