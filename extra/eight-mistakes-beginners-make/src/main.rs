//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-09T22:44:03AEDT when are enums Copy?
//  Ongoing: 2023-02-09T22:44:43AEDT match 'or' uses single '|'?
//  Ongoing: 2023-02-09T23:33:38AEDT 'array_window()' <of/over> N elements?
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//  Allow 'array_windows()'
#![feature(array_windows)]

//  macro: get_func_name
//  {{{
macro_rules! get_func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}
//  }}}

//  Continue: 2023-02-09T23:33:13AEDT complete article
//  Continue: 2023-02-09T23:07:32AEDT complete 'Error' implementation for custom error (manually / "This Error" crate)

//  8 deadly mistakes beginner Rust developers make
//  LINK: https://www.youtube.com/watch?v=PbR4ECFIckg 

fn one_unneccessary_indirection()
{
    //  Use '&str' instead of 'String' (the latter coerces to the former)
    //  This is more flexible, as we can now accept both String and &str arguments
    fn fancy_print_i(s: String) {
        println!("s=({})", s);
    }
    fn fancy_print_ii(s: &str) {
        println!("s=({})", s);
    }

    println!("{}, DONE", get_func_name!());
}

fn two_overuse_slice_indexing()
{
    let points = vec![5,9,7,4,2,6];
    println!("points=({:?})", points);

    //  Using C-style indexing
    let mut deltas = vec![];
    for i in 1..points.len() {
        let current = points[i];
        let previous = points[i-1];
        let delta = current - previous;
        deltas.push(delta);
    }
    println!("deltas=({:?})", deltas);

    //  slice.array_windows()
    //  (unstable) Returns an iterator over overlapping windows of N elements
    //  <(use 'copied()' or else it consumes values?)>

    //  Using 'array_windows()' (unstable)
    let mut deltas = vec![];
    for [previous, current] in points.array_windows().copied() {
        let delta = current - previous;
        deltas.push(delta);
    }
    println!("deltas=({:?})", deltas);

    //  Replacing loop with map
    let mut deltas = points.array_windows().copied()
        .map(|[previous,current]| current - previous)
        .collect::<Vec<_>>();
    println!("deltas=({:?})", deltas);

    println!("{}, DONE", get_func_name!());
}

fn three_sentinel_values()
{
    //  Sentinel: value with a special meaning

    //  Eg: returning -1 / "" / Null to indicate failure
    fn find_i(arr: &[i32], val: i32) -> i32 {
        for i in 0..arr.len() {
            if arr[i] == val {
                return i as i32;
            }
        }
        -1
    }

    //  Instead use Option<T>
    fn find_ii(arr: &[i32], val: i32) -> Option<usize> {
        for i in 0..arr.len() {
            if arr[i] == val {
                return Some(i);
            }
        }
        None
    }

    println!("{}, DONE", get_func_name!());
}

fn four_enums_and_pattern_matching()
{
    //  Bad: using String values to indicate options
    fn can_publish_blog_i(role: &str) -> bool {
        if role == "Admin" || role == "Writer" {
            return true;
        } 
        false
    }
    //  Instead use an Enum to encode options
    enum Role {
        Admin, Reader, Writer,
    }
    fn can_publish_blog_ii(r: Role) -> bool {
        match r {
            Role::Admin | Role::Writer => true,
            _ => false,
        }
    }


    //  if statement vs let pattern matching:
    let opt = Some(32);
    if opt.is_some() {
        let value = opt.unwrap();
        //  { ... }
    }
    if let Some(value) = opt {
        //  { ... }
    }


    //  if statement vs let pattern matching:
    let list = vec![1,2,3];
    if !list.is_empty() {
        let first = list[0];
        //  { ... }
    }
    if let [first, ..] = list.as_slice() {
        //  { ... }
    }


    println!("{}, DONE", get_func_name!());
}


fn five_error_handling()
{
    //  Error propagation operator: '?'
    //          let a: i32 = a.parse()?;
    //  is equivalent to
    //          let a = a.parse()
    //          if let Err(e) = a { return Err(e); }
    //          let a = a.unwrap()
    //  (containing function must return the relevant Result type)

    //  Bad: not using '?' operator for error propagation
    fn parse_then_add_i(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let a = a.parse::<i32>();
        if let Err(e) = a { 
            return Err(e);
        }
        let b = b.parse::<i32>();
        if let Err(e) = b {
            return Err(e);
        }
        Ok(a.unwrap() + b.unwrap())
    }
    //  Instead:
    fn parse_then_add_ii(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let a: i32 = a.parse()?;
        let b: i32 = b.parse()?;
        Ok(a + b)
    }


    //  Bad: custom Error types that do not implement 'std::error:Error'
    //  (However) Implementing Error ourselves is tedious:
    #[derive(Debug)]
    pub enum DataStoreError {
        Disconnect(std::io::Error),
        Redaction(String),
        InvalidHeader { expected: String, found: String },
        Unknown,
    }
    impl std::fmt::Display for DataStoreError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
//  impl std::error::Error for DataStoreError {
//      fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//          //  ...
//      }
//      fn cause(&self) -> Option<&dyn std::error::Error> {
//          //  ...
//      }
//  }


    //  recommendation: use "This Error" library
    //  <>

    println!("{}, DONE", get_func_name!());
}

fn sixth_standard_library_traits()
{
    //  'std::default::Default' allows sensible default values to be <specified/created>
    pub trait Eg_Default: Sized {
        fn default() -> Self;
    }

    //  'std::convert::From' define a conversion between types
    //  (also provides 'std::convert::Into')
    pub trait Eg_From<T>: Sized {
        fn from(value: T) -> Self;
    }

    //  'std::convert::TryFrom' defines a conversion between types that can fail
    //  (also provides 'std::convert::TryInto')
    pub trait Eg_TryFrom<T>: Sized {
        type Error;
        fn try_from(value: T) -> Result<Self, Self::Error>;
    }
    

    enum CliError {
        IoError(std::io::Error),
        ParseError(std::num::ParseIntError),
    }


    println!("{}, DONE", get_func_name!());
}


fn seven()
{
}


fn eight()
{
}


fn bonus()
{
}


fn main() 
{
    one_unneccessary_indirection();
    two_overuse_slice_indexing();
    three_sentinel_values();
    four_enums_and_pattern_matching();
    five_error_handling();
    sixth_standard_library_traits();
    seven();
    eight();
    bonus();
}

