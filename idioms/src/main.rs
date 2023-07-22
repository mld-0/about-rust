//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  }}}
//#![allow(unused)]
#![allow(non_snake_case)]


fn about_match() 
{
}


fn if_let()
{
}


fn while_let()
{
}


fn about_enums()
{
}


//  Option<T>
fn Option_T()
{
    //  LINK: Module std::option (see below)
    //  https://doc.rust-lang.org/std/option/

    //  is_some()           
    //  is_none()

    //  Adapters for working with references:
    //  as_ref():               &Option<T> -> Option<&T>
    //  as_mut()                &mut Option<T> -> Option<&mut T>
    //
    //  as_deref():             &Option<T> -> Option<&T::Target>
    //  as_deref_mut():         &mut Option<T> -> Option<&mut T::Target>


    //  Extract contained value:
    //  expect():               <>
    //  unwrap():               <>
    //  unwrap_or():            <>
    //  unwrap_or_default()     <>
    //  unwrap_or_else()        <>

    //  Transform contained value:
    //  <>

}


//  Box<T>
fn Box_T()
{
}

//  Option<Box<T>>
fn Option_Box_T()
{
}


fn main() 
{
    about_match();
    about_enums();
    if_let();
    while_let();
    Option_T();
    Box_T();
    Option_Box_T();
    println!("main(), DONE");
}

