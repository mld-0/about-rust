//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-09T00:50:01AEDT (how much I hate) 'constructor' as the name for 'thing from an enum'
//  Ongoing: 2022-10-09T00:53:16AEDT (Unlike the book) we use 'OK' instead of 'Ok' for <an/our> enum value</constructor> [...] (which is Ok(?) (but the syntax highlighting picks up on it))
//  Ongoing: 2022-10-09T00:56:50AEDT book advises (in previous chapter) to not import <functions/methods> directly, but instead import one the module one level above them and use that, i.e: 'use std::mem; mem::size_of();', but then does not-that with (among other examples) size_of
//  Ongoing: 2022-10-09T01:02:24AEDT why 'TimeUnit' only implements PartialEq
//  Ongoing: 2022-10-09T01:15:15AEDT returning &'static str as opposed to String(?) (returning a string constant?)
//  Ongoing: 2022-10-09T01:16:13AEDT what's the disgusting hackey way to do what TimeUnit::singular (get the name of each enum as a string) does? (without the verbosity/tedium)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

//  <(Enums provide an either/or type)>

//  Enums are otherwise known as sum-types, discriminated-unions, or algebraic-datatypes
//  Traditional Enums provide named constants

//  Rust allows Enums of containing multiple types
//  (while remaining type safe)
//  These enums are useful whenever a value might be either one thing or another


fn example_C_style_enums()
{
    //  In a C-style enum, values are stored using the smallest integer type that will contain them
    //  We can override this type with a '[#repr]' attribute

    //  Each value is known as a variant or a constructor
    //enum Ordering {
    //    Less, 
    //    Equal, 
    //    Greater,
    //}
    //  This enum above is part of the standard library
    //use std::cmp::Ordering;
    //use std::cmp::Ordering::*;

    enum Pet {
        Orca,
        Giraffe,
        Camel, 
    }
    let p1 = Pet::Orca;

    //  Use a self-import to import the <constructors> values of an enum in the current module
    //use self::Pet::*;         //  <(need to be in namespace/crate/module/<?>)>
    use Pet::*;
    let p2 = Orca;

    //  We can supply custom values, otherwise Rust will assign values beginning at 0
    enum HttpStatus {
        OK = 200,
        NotModified = 304,
        NotFound = 404,
    }

    use std::mem::size_of;
    assert_eq!(size_of::<Pet>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    //  Casting a C-style enum value to an integer is allowed
    //  (even where the cast results in an underflow) <(in which case we don't even get a warning?)>
    assert_eq!(HttpStatus::OK as i32, 200);

    //  Casting an integer to an enum is not allowed
    //  (Rust guarantees that an enum is only ever one of the values in the declaration)
    //  (Write a checked conversion function if needed as an alternative) (or use the 'enum_primitive' crate)
    fn HttpStatus_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::OK),
            304 => Some(HttpStatus::NotModified),
            404 => Some(HttpStatus::NotFound),
            _   => None,
        }
    }

    //  As with structs, the compiler can implement features</traits> copy and compare if told to do so:
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years,
    }
    //  Enums can have methods:
    impl TimeUnit {
        fn singular(self) -> &'static str {
            match self { TimeUnit::Seconds => "second", TimeUnit::Minutes => "minute", TimeUnit::Hours => "hour", TimeUnit::Days => "day", TimeUnit::Months => "month", TimeUnit::Years => "year", }
        }
    }

    println!("example_C_style_enums, DONE");
}


fn example_enums_with_data()
{
    println!("example_enums_with_data, DONE");
}


fn main() 
{
    example_C_style_enums();
    example_enums_with_data();
}

