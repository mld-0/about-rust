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
//  Ongoing: 2023-02-22T23:42:31AEDT 'compile_error!()' example?
//  Ongoing: 2023-02-22T23:59:05AEDT Result vs Option: Result can contain an error type, (<what/anything> else?)
//  Ongoing: 2023-02-28T21:54:42AEDT (tool for) identifying which functions in a block may panic
//  Ongoing: 2023-02-28T22:00:03AEDT how to declare 'my_push_all()' example (without having to comment it out)
//  Ongoing: 2023-02-28T22:02:46AEDT meaning of the 'Maximal' exception safety (safe code should be exception safe to the point of doing the right thing)
//  Ongoing: 2023-02-28T22:13:25AEDT "Rust doesn't have a try-finally construct" -> try_unwind <doesn't count / isn't suitable> (or this was written before it was <added/stabilized>?)
//  Ongoing: 2023-02-28T23:00:28AEDT Types other than 'Mutex' which poison themselves?
//  }}}
use std::sync::{Arc, Mutex};
use std::thread;

//  Continue: 2023-02-28T23:07:48AEDT exception safety best practices
//  Continue: 2023-02-28T23:08:06AEDT identifying potential panics
//  Continue: 2023-02-28T23:08:16AEDT try-catch-finally alternatives

//  Rust has a tiered error handling scheme:
//          Option when something might be absent
//          Result when something that can be handled goes wrong
//          panic! when something that cannot be handled goes wrong
//          abort! when something catastrophic happens

//  Option / Result can later be promoted into panic or abort at the callers discretion

//  <(A panic can only be caught by the parent thread)>

//  If the main thread panics, all threads are terminated and the program returns rc=101

//  <(In Rust, 'Exception Safety' refers to always being memory-safe should unwinding occur)>


#[test]
fn panic()
{
    //  LINK: https://doc.rust-lang.org/std/macro.panic.html
    //
    //  'panic!()'
    //  Allows the program to terminate immediately with feedback to the user
    //  Takes a string argument using 'format!()' syntax
    //  Default behaviour is to print message to stderr with file/line/column of panic call
    //
    //  Set default behaviour with 'std::panic::set_hook()'
    //  Use 'std::panic::panic_any()' to panic with any other type as argument
    //  Use 'compile_error!()' for conditional errors at compile time
    //
    //  panic vs Result
    //  panic is used to construct errors representing a bug
    //  Result is used to wrap the result of an action that can fail
    //  Propagate any error in a Result with the '?' operator

    //  LINK: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
    //
    //  <('expect' is just 'unwrap' that takes an argument)>
    //
    //  A final program should have more robust error handling than 'unwrap' / 'expect' where failure is anticipated
    //  
    //  Use 'result.expect(msg)' to indicate that Err would be a bug
    //
    //  panic when something happens that could leave the program in a bad state
    //  panic when a function is given input it cannot handle
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
    //  (Rust optimises heavily for the "doesn't unwind" case)

    //  It is UB to unwind from Rust code into foreign code
    //  (use 'catch_unwind' to catch any panics <at/before> the FFI boundary)


    //  Example: destructors called
    //  <>
    struct Foo {};
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("drop Foo");
        }
    }
    panic::catch_unwind( || {
        let x = Foo {};
    });
}



#[test]
fn exception_safety()
{
    //  Rust provides a lot of mechanisms for code to panic
    //          Attempting to unwrap a None
    //          Index out-of-bounds
    //          Divide by zero
    //          Arithmetic overflow (debug builds)

    //  Exception Safety is the state of code working correctly when exceptions are thrown.
    //  (Being ready for unwinding wherever code might panic)

    //  In Rust, we must concern ourselves with two levels of exception safety
    //          Minimal: unsafe code must be exception safe to the point of not violating memory safety
    //          Maximal: <(safe code should be exception safe to the point of doing the right thing)>

    //  All Unsafe code must ensure it has minimal exception safety

    //  <(Unsafe code must be read to deal with bad Safe code when it comes to unwinding)>
    //  (Only non-panicking code should be run when the program is in an unsound state)
    //  (Alternatively, have a guard to clean up unsound states in case of panic)


    //  Example: Vec::push_all
    //  The function 'T::clone()' is free to panic
    //  If it does so, the function will exit early leaving the vector with an invalid length
    //  This will lead to uninitalized memory being dropped when the vector is dropped
//  impl<T: Clone> Vec<T> {
//      fn my_push_all(&mut self, to_push: &[T]) {
//          self.reserve(to_push.len());
//          unsafe {
//              self.set_len(self.len() + to_push.len());
//              for (i,x) in to_push.iter().enumerate() {
//                  self.ptr().add(i).write(x.clone());
//              }
//          }
//      }
//  }
    //  Fix: update the vector's length after each loop iteration


    //  Example: BinaryHeap::sift_up
    //  <(since Rust doesn't have a try-finally construct, we define a seperate struct to store the algorithms state, with its own destructor to handle the 'finally' logic)>
    //  <>
}


#[test]
fn poisoning()
{
    //  Types can often get away with not providing maximal exception safety, since anything that witnesses an exception is about to be destroyed

    //  Some types may chose to poison themselves if they witness a panic
    //  (prevent any further use to prevent errors propogating)

    //  'std::sync::Mutex' will poison itself if one of its MutexGuards is dropped during a panic
    //  Any further attempts to lock the Mutex will return Err or panic
    //  Preventing a Mutex that has witnessed a panic while locked prevents any data that was left in an inconsistent/incomplete state from being used.
    //  (Mutex provides 'into_inner()' to access the lock even though it is poisoned)


    //  Example: recovering a poisoned mutex
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = lock.clone();
    let _ = thread::spawn( move || -> () {
        let _guard = lock2.lock().unwrap();
        panic!();
    }).join();
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(posisoned) => posisoned.into_inner(),
    };
}


#[test]
fn best_practices()
{
    //  LINK: https://towardsdatascience.com/how-to-design-for-panic-resilience-in-rust-55d5fd2478b9
    //  {{{
    //  }}}

    //  LINK: https://blog.burntsushi.net/unwrap/
    //  {{{
    //  }}}
}


#[test]
fn identifying_potential_panics() 
{
    //  Rudimentary / <totally-inadequate> check:
    //          grep -rE 'panic!|.unwrap()|.expect()' 

    //  Tools:
    //          <(?)>

    //  <(User defined code = potential panic)>
}


#[test]
fn try_catch_finally_alternatives()
{
}

