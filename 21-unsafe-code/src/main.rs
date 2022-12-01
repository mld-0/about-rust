//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-12-01T00:32:04AEDT 'unsafe' allows Rust to be written in Rust ... it allows efficent buffer management of Vec (I follow), std::io to talk to the OS (I'm ok-<ish>), provide std::sync/std::thread concurrency primatives (I follow less<?>) ... ((now that you mention it) please explain the first claim too)
//  Ongoing: 2022-12-01T22:38:09AEDT How is newtype 'Ascii' enforcing behaviour at runtime - (won't any error due to non-ascii bytes be at runtime?) [...] (or does 'protected at compiletime' mean that if the program compiles, any such errors that arise will be caught?)
//  Ongoing: 2022-12-01T22:50:02AEDT A struct-tuple (tuple-struct) with a single variable has the same layout in memory as that variable would have(?)
//  Ongoing: 2022-12-01T22:54:29AEDT (recall), implementing 'From' also implements 'To'(?)
//  Ongoing: 2022-12-01T23:00:57AEDT Is invalid utf8 data in a String considered undefined behaviour? 
//  Ongoing: 2022-12-01T23:15:17AEDT meaning of the term 'contact' in the context of Rust
//  Ongoing: 2022-12-01T23:22:01AEDT <(definition: aliasing - where two references point to <the-same/overlapping> memory)>
//  Ongoing: 2022-12-02T00:29:09AEDT detectable-ness of Rust UB (is Rust like C in that a body of code cannot reasonably be judged to be UB free?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//  Use pointers only were references are inadequate (memory management / FFI [foreign-function-interfaces] / <>)
//  References: shared '&' / mutable '&mut'
//  Pointers: '*const'  / '*mut*

//  Shared references in long form: "&'a T", parameters: 'a / T
//  pointers have no lifetime parameter (compiler cannot check validity of use)
//  Pointers: stored in 'usize', can be null

//  Pointers can be cast to different types, different constness/mutablity using 'as'
//  References can be cast to pointers using 'as'
//  Pointers can be converted to references by dereferencing them and borrowing a reference from the result
//  Pointers are not automatically dereferenced by '.'

//  C++ smart pointer equivalents:
//      std::unique_ptr         Box
//      std::shared_ptr         Arc


//  Rust provides the 'unsafe' block for certain operations
//  <(Rust's normal level of safety is not available (in unsafe blocks)/(for unsafe operations))>
//  Unsafe features can result in undefined behaviour if not used <carefully/correctly>
//  (Undefined behaviour is bad(TM))


//  <(The declaration of an unsafe block is an assurance to Rust that the code within does not break its safety rules)>


fn example_unsafe_blocks()
{
    //  An unsafe block is an ordinary block preceded by the keyword 'unsafe'
    unsafe {
        String::from_utf8_unchecked(vec![20,30,40,50]);
    }

    //  Avoid unsafe blocks except where they are strictly necessary

    //  Unsafe blocks permit 4 additional operations:
    //      unsafe functions can be called
    //      raw pointers can be dereferenced
    //      mutable static variables can be accessed
    //      FFI functions/variables can be accessed

    //  Safe code can create, pass, and compare raw pointers

    //  <(what makes a function unsafe? (consider 'String::from_utf_unchecked' <- function is unsafe because it violates String's <guarantee/invariant> to always be valid utf8(?)))>

    fn eg_ascii() 
    {
        //  Custom type 'Ascii', buffer guaranteed to hold valid ASCII
        //  Use unsafe block to allow efficent conversion to String, without requiring unsafe interface
        //  Since Vec<u8> Ascii.0 is a private variable, it can only be accessed by associated functions - this allows us to validate any data placed in it
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Ascii(Vec<u8>);
        #[derive(Debug, Eq, PartialEq)]
        pub struct NotAsciiError(pub Vec<u8>);
        impl Ascii {
            pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
                if bytes.iter().any(|&b| !b.is_ascii()) {
                    return Err(NotAsciiError(bytes))
                }
                Ok(Ascii(bytes))
            }
        }
        impl From<Ascii> for String {
            fn from(ascii: Ascii) -> String {
                unsafe { String::from_utf8_unchecked(ascii.0) }
            }
        }

        //  'Ascii' is an example of a newtype - a wrapper around an existing type that enforces certain behaviours (and type safety) <(at compile-time)>

        let b: Vec<u8> = b"ASCII and ye shall recieve".to_vec();
        let a1: Ascii = Ascii::from_bytes(b).unwrap();
        let a2: Ascii = a1.clone();
        let s1: String = String::from(a1);
        let s2: String = Ascii::into(a2);
        assert_eq!(s1, "ASCII and ye shall recieve");
    }
    eg_ascii();

    println!("example_unsafe_blocks, DONE");
}


fn example_unsafe_functions()
{
    //  An unsafe function is a function where 'fn' is preceded by the keyword 'unsafe'
    //  Unsafe functions can only be called from inside unsafe blocks
    //  The body of an unsafe function is an unsafe block

    //  Declaring a function 'unsafe' is a warning that it has a contact that must be satisfied to avoid undefined behaviour
    //  In this case, we are not validating that 'bytes' is valid ASCII data
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Ascii(Vec<u8>);
    impl Ascii {
        //  We declare this function unsafe because the caller must follow rules Rust cannot enforce to avoid undefined behaviour
        //  Note that it does not return a Result
        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    //  <(The presence of unsafe functions implies invalid inputs can cause undefined behaviour)>
    //  <(The consiquences of violating the contract of an unsafe block may not occur until after leaving that block)>
    //  To avoid UB, not just the unsafe function must be correct, so must all code that calls it

    //  A function that can be misused in a way that compiles, but causes UB, is should be marked unsafe 
    //  (and a function that cannot should not be market unsafe, even if it contains unsafe blocks)

    println!("example_unsafe_functions, DONE");
}


fn example_undefined_behaviour()
{
    //  In Rust, mutable references never alias. Breaking this rule in any way results in undefined behavior.
    //  There is no safe way to create two &mut references to the same value, and creating two mutable references via unsafe code is UB.

    //  UB is a powerful tool for compiler writers to permit optimizations

    //  Rust optimises code in accordance with its rules for well-behaved programs
    //          must not read uninitalized memory
    //          must not create invalid primatives 
    //                  null references/boxes
    //                  bools that are not 0/1
    //                  invalid enum values
    //                  non unicode char / non utf8 str/String
    //          no reference may outlive its referent
    //          shared references are read-only
    //          mutable references are exclusive
    //          must not derefence null / incorrectly-aligned / dangling pointers
    //          must not use pointer to access memory outside allocation associated with pointer
    //          must not contain data races (two threads writing/reading memory without synchronization)
    //          <(must not unwind <accross> a call made from another language)>
    //          must not violate contracts of standard library functions
    //  Violating any of these rules is UB
    //  Rust code that does not use unsafe features is guaranteed to follow all these rules if it compiles

    println!("example_undefined_behaviour, DONE");
}


fn example_unsafe_traits()
{
    println!("example_unsafe_traits, DONE");
}


fn main() 
{
    example_unsafe_blocks();
    example_unsafe_functions();
    example_undefined_behaviour();
    example_unsafe_traits();
}

