//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-12-06T00:48:10AEDT why 'Eg_assert_eq' has so many level of match statements(?) [...] (is it to turn our '$left' macro argument into 'left_val' rust variable(?))
//  Ongoing: 2022-12-06T00:57:01AEDT 'Eg_assert_eq' using 'let' instead of 'match' (which books says should be equivalent(?))
//  Ongoing: 2022-12-11T21:53:31AEDT book uses 'let' expression in vec! example defintion - we get an error trying to declare 'let mut v = Vec::new()' in our macro(?)
//  Ongoing: 2022-12-11T22:00:01AEDT what 'stringify!()' can/can't do? [...] (same for 'concat!()' / <other-macros?>)
//  Ongoing: 2022-12-12T21:27:40AEDT macro 'json!' and handling of trailing commas [...] (macro as taken from book does not support (is broken by) trailing commas)
//  Ongoing: 2022-12-12T21:44:11AEDT when is it necessary to provide a lifetime parameter (is it possible to implement 'From<&str>' for 'Json' without one?)
//  Ongoing: 2022-12-12T22:27:57AEDT prefixing names with '$crate' doesn't work (for macro defined in function?)
//  }}}
//#![feature(log_syntax)]
//#![feature(trace_macros)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  macro: func_name
//  {{{
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}
//  }}}

use std::collections::HashMap;

//  Macros offer functionality beyond what simple function calls can provide
//  During compilation, a macro call is expanded into Rust code.
//  Macro calls are denoted by an exclamation point, eg: 'println!()'

//  Macros have a default recursion limit of 64
//  Increase this with: #![recursion_limit = "256"]

fn example_macro_basics()
{
    //  Macros cannot be called before they are defined:
    //Eg_assert_eq![5, 5];          //  error, macro not found

    //  The definition of a macro can be contained in brackets, braces, or parenthesis. A semicolon is optional after a macro defined inside braces '{}'
    //  By convention, 'assert_eq!' is called with parentheses '()', 'vec!' is called with brackets '[]', and macros are defined with braces '{}'

    //  Definition: 'assert_eq!()'
    macro_rules! Eg_assert_eq {
        ($left: expr, $right: expr) => ({
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        panic!("Eg_assert_eq failed: (left: `{:?}`, right: `{:?}`)", 
                                    left_val, right_val)

                    }
                }
            }
        });
    }
    //  '$left' / '$right' are replaced with the arguments given during macro expansion
    //  <('expr' denotes an expression)>

    //  Equivalent: A macro can be called with brackets, braces, or parethesis
    Eg_assert_eq!(5, 5);
    Eg_assert_eq![5, 5];
    Eg_assert_eq!{5, 5};
    Eg_assert_eq!{5, 5}
    //  <(a semicolon is optional after a macro call with braces '{}')>


    //  The main way to define a macro is with 'macro_rules!' 
    //  These work by pattern matching. The body of a macro is just a series of rules:
    //      ( pattern1 ) => ( template1 );
    //      ( pattern2 ) => ( template2 );
    //      ...

    //  When the macro is expanded, the arguments given are matched against a pattern
    //  (In the case of Eg_assert_eq: '($left:expr, $right:expr)')


    //  It is important to consider that an argument might be an expression, ie: 'v.pop()'
    //  For that reason, the variables corresponding to the matched expression should only appear once
    //  Macros should use these variables by borrowing references to them, not by moving them


    //  Rust does the equivalent of placing parenthesis around pattern variables when expanding them
    //  (so that 'ADD_ONE(1 << 4)' becomes '(1 << 4) + 1' instead of '1 << 4 + 1', a classic C++ macro bug)


    fn repetition()
    {
        //      $(...)*         Match 0 or more times with no seperator
        //      $(...),*        Match 0 or more times, seperated by commas
        //      $(...);*        Match 0 or more times, seperated by semicolons
        //      $(...)+         Match 1 or more times with no seperator
        //      $(...),+        Match 1 or more times, seperated by commas
        //      $(...);+        Match 1 or more times, seperated by semicolons

        //  '<[_]>' denotes a slice whose type Rust must infer

        //  '$($x),*' iterates over whatever matches for 'x', inserting them into the macro as comma seperated values

        //  This method does not support trailing commas
        //  To handle these, use the rule: '$(...),+ ,' as a final case

        //  'vec!' comes in two forms:
        let buffer = vec![0_u8; 1000];
        let noodles = vec!["udon", "ramen", "soba"];

        //  This can be implemented as:
        macro_rules! Eg_vec {
            ($x: expr ; $n: expr) => {
                ::std::vec::from_elem($x, $n)
            };
            ( $( $x: expr ),* ) => {
                <[_]>::into_vec(Box::new([ $($x),* ]))
            };
            //  Trailing comma case
            ( $( $x: expr ),+ , ) => {
                Eg_vec![ $($x),* ]
            }
        }

        let x = Eg_vec![1,2,3];
        let y = Eg_vec![1;20];
    }
    repetition();


    println!("{}, DONE", func_name!());
}


fn example_built_in_macros()
{
    //  Rust supplies a number of macros (which do things that cannot be implemented by 'macro_rules!')

    //  file!()
    //  Expands to string literal containing current filename

    //  line!() / column!() 
    //  Expands to u32 literals containing line/column number
    //  (Line is 1-indexed, column is 0-indexed)

    //  stringify!(...tokens...)
    //  Expands to string literal containing given tokens
    //  <(what it can/can't do?)>

    //  concat!(str0, str1, ...)
    //  Expands to string literal made by concatenating its arguments

    //  cfg!(...)
    //  Expands to boolean constant, true if build config matches arguments

    //  env!("VAR_NAME")
    //  Expands to string literal, value of 'VAR_NAME' in environment at compile time

    //  option_env!("VAR_NAME")
    //  Same as 'env!()', except it returns 'Option<&'static str>'

    //  include!("file.rs")
    //  Expands to contents of specified Rust file

    //  include_str!("file.txt")
    //  Expands to '&static str' containing text of specified file (must be valid UTF8)

    //  include_bytes("file.dat")
    //  Expands to &'static [u8] containing contents of specified file

    println!("{}, DONE", func_name!());
}


fn example_debugging_macros()
{
    //  Rust provides various tools for debugging macros
    //  (requires Rust nightly as of stable-1.65.0)

    //  (Toggle rust nightly)
    //          rustup default nightly
    //          rustup default stable

    //  To view the source of a package with all macros expanded:
    //          cargo rustc --profile=check -- -Zunpretty=expanded
    //  To view the source of a file with all macros expanded:
    //          rustc -Zunpretty=expanded <file>

    //  Use the 'log_syntax!()' macro to print arguments at compile time
    //          #![feature(log_syntax)]
    //          log_syntax!("asdf\n");

    //  To log all macro calls at compile time:
    //          #![feature(trace_macros)]
    //          trace_macros!(true);
    //          <macros>
    //          trace_macros!(false);

    println!("{}, DONE", func_name!());
}


fn example_json_macro()
{
    //  Types of macro argument fragments
    //          Type:           Matches:                                        Can be followed by
    //          expr            An expression                                   => , ;
    //          stmt            An expression, not including trailing ;         => , ;
    //                          (use expr / block instead)
    //          ty              A type                                          => , ; = | { [ : > as where
    //          path            A path                                          => , ; = | { [ : > as where
    //          pat             A pattern                                       => , = | if in
    //          item            An item                                         Anything
    //          block           A block                                         Anything
    //          meta            Body of an attribute                            Anything
    //          ident           An identifier                                   Anything
    //          tt              A token tree                                    Anything

    //  A token tree is everything between a pair of brackets or quotes

    #[derive(Clone, PartialEq, Debug)]
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String,Json>>),
    }

    //  Example: macro for creating 'Json' values
    macro_rules! json {
        (null) => { 
            Json::Null 
        };
        ([ $($x:tt),* ]) => { 
            Json::Array( vec![ $(json!($x)),* ] ) 
        };
        ([ $($x:tt),+ , ]) => {             //  trailing comma case
            json![ [$($x),*] ] 
        };       
        ({ $($k:tt : $v:tt),* }) => { 
            //  Need to declare inner block in order to 'let' declare variables
            {
                let mut fields = Box::new(HashMap::new());
                $( fields.insert($k.to_string(), json!($v)); )*
                Json::Object(fields)
            }
        };
        //  or:
        //({ $($k:tt : $v:tt),* }) => { 
        //    Json::Object(Box::new(vec![ $( ($k.to_string(), json!($v)) ),* ].into_iter().collect()))
        //};
        ({ $($k:tt : $v:tt),+ , }) => {      //  trailing comma case
            json![ {$($k:$v),*} ] 
        };        
        ($x:tt) => { 
            Json::from($x) 
        };
    }

    //  Macros are not good at distinguishing types
    //  To support conversion of various types to 'Json', we implement the 'From' trait for it
    impl From<bool> for Json {
        fn from(b: bool) -> Json { Json::Boolean(b) }
    }
    impl From<String> for Json {
        fn from(s: String) -> Json { Json::String(s) }
    }
    impl From<&str> for Json {
        fn from(s: &str) -> Json { Json::String(s.to_string()) }
    }
    //impl<'a> From<&'a str> for Json {
    //    fn from(s: &'a str) -> Json { Json::String(s.to_string()) }
    //}

    //  Providing 'From' implementation for multiple types
    macro_rules! impl_from_num_for_json {
        ( $($t:ident)* ) => {
            $(
                impl From<$t> for Json {
                    fn from(n: $t) -> Json { Json::Number(n as f64) }
                }
            )*
        };
    }
    impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

    assert_eq!(json!(null), Json::Null);
    assert_eq!(json!("abc"), Json::String("abc".to_string()));
    assert_eq!(json!( {"name": "Larry"} ), Json::Object(Box::new(vec![ ("name".to_string(), Json::String("Larry".to_string())) ].into_iter().collect())));

    let students = json!([
                         {
                             "name": "Jim Blandy",
                             "class_of": 1926,
                             "major": "Tibetan throat singing",
                         },
                         {
                             "name": "Jason Orendorff",
                             "class_of": 1702,
                             "major": "Knots",
                         },
    ]);

    println!("students=({:?})", students);

    //  Our 'json!()' macro uses Box / HashMap / Json (and will not work if used in a scope where all of these are not visible)

    //  Rust macros are hygenic
    //  That is, variable names declared in the macro are renamed so as to not conflict with variables passed as argument
    //  This prevents macros from accessing variables from the callers scope - any variables needed by the macro should be passed to it as arguments


    //  Importing / exporting macros
    //  Within a crate:
    //      Macros that are visible in a module are visible in its child modules
    //      Use '#[macro_use]' attribute to export a macro from a module into its parent
    //  Between crates:
    //      Use '#[macro_use]' to import modules from another crate
    //      use '#[macro_export]' to export modules from a crate

    //  A macro should use absolute paths to any names it uses
    //  With a macro '$crate' expands to the absolute path of the root module of the crate where the macro is defined 
    //  <(Use '$crate' to prefix any names used in the macro to ensure the macro works in different scopes)>


    println!("{}, DONE", func_name!());
}


fn example_avoiding_matching_syntax_errors()
{
    println!("{}, DONE", func_name!());
}


fn example_procedural_macros()
{
    println!("{}, DONE", func_name!());
}


fn main() 
{
    example_macro_basics();
    example_built_in_macros();
    example_debugging_macros();
    example_json_macro();
    example_avoiding_matching_syntax_errors();
    example_procedural_macros();
}

