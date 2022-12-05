//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-12-06T00:48:10AEDT why 'Eg_assert_eq' has so many level of match statements(?) [...] (is it to turn our '$left' macro argument into 'left_val' rust variable(?))
//  Ongoing: 2022-12-06T00:57:01AEDT 'Eg_assert_eq' using 'let' instead of 'match' (which books says should be equivalent(?))
//  }}}
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

//  Macros offer functionality beyond what simple function calls can provide
//  During compilation, a macro call is expanded into Rust code.
//  Macro calls are denoted by an exclamation point, eg: 'println!()'


fn example_macro_basics()
{
    //  Macros cannot be called before they are defined:
    //Eg_assert_eq![5, 5];          //  error, macro not found


    //  Definition: 'assert_eq!()'
    //  The definition of a macro can be contained in brackets, braces, or parenthesis. A semicolon is optional after a macro defined inside braces '{}'
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

    //  By convention, 'assert_eq!' is called with parentheses '()', 'vec!' is called with brackets '[]', and macros are defined with braces '{}'


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
    }
    repetition();


    println!("{}, DONE", func_name!());
}


fn example_built_in_macros()
{
    println!("{}, DONE", func_name!());
}


fn example_debugging_macros()
{
    println!("{}, DONE", func_name!());
}


fn example_json_macro()
{
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

