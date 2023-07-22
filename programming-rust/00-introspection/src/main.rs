//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-11-11T03:00:22AEDT stackoverflow link (only simplest (new-ish) answer given - link contains (other) more involved solutions)
//  Ongoing: 2022-11-11T03:01:30AEDT <perfect-forwarding> (get actual type (not differing by '&') of value parameter type passed, by-value, without stealing value?)
//  }}}

fn example_get_type_name()
{
    //  LINK: https://doc.rust-lang.org/std/any/fn.type_name.html
    //  LINK: https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable

    fn print_type_of<T>(_: &T) -> String {
        String::from(std::any::type_name::<T>())
    }
    fn print_type_of_byval<T>(x: T) -> String {
        String::from(std::any::type_name::<T>())
    }

    let v1: Vec<i32> = (0..=9).collect();

    println!("type_name(Vec<i32>)=({})", std::any::type_name::<Vec<i32>>());
    println!("print_type_of(v1)=({})", print_type_of(&v1));
    println!("print_type_of_byval(v1)=({})", print_type_of_byval(v1));

    println!("example_get_type_name, DONE");
}


fn example_alteratives_to_branching_by_getType()
{
    //  LINK: https://www.reddit.com/r/rust/comments/uw0hxg/how_to_get_type_of_a_variable/
}


fn main() 
{
    example_get_type_name();
}

