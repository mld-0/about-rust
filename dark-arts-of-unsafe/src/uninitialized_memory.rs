//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-17T18:20:31AEDT 'Rust tracks whether a variable should be dropped at runtime with a flag on the stack' (does it *always*? Surely many such runtime checks could be optimized away?) 
//  Ongoing: 2023-02-17T18:29:00AEDT re-creating a variable (of the same name) with 'let' - when does the old one get dropped?
//  }}}

//  Continue: 2023-02-17T18:31:37AEDT complete chapter

#[test]
fn checked_uninitialized_memory()
{
    //  Stack variables are uninitialized until a value is assigned to them
    //  Rust prevents uninitialized variables being read - every branch must assign it a value before it can be used

    //  A variable does not have to be mutable to be assigned a value on different branches
    let x: i32;
    if true {
        x = 5;
    } else {
        x = 4;
    }
    assert_eq!(x, 5);
    //  Note that all branches must assign the variable a value (even if they are unreachable)


    //  However, this is valid: the compiler understands that 'x' will only be assigned to once (and therefore does not need to be mutable
    let x: i32;
    loop {
        x = 0;
        break;
    }


    //  If a value is moved out of a variable, that variable becomes logically uninitialized if the type of the value isn't Copy
    let x = 0;
    let y = Box::new(0);
    let z1 = x;             //  x is still valid because i32 is Copy
    let z2 = y;             //  y is uninitialized because Box isn't Copy


    //  Cannot re-assign to a variable after moving out of it unless it is mutable
    let mut x = Box::new(0);
    let y = Box::new(0);
    let z1 = x;
    let z2 = y;
    x = Box::new(1);
//  y = Box::new(1);        //  error, 'y' is not mutable
}

#[test]
fn drop_flags()
{
    //  Rust <always?> tracks whether a type should be dropped at runtime using a 'drop flag' on the stack
    //  (previously drop flags were kept in a hidden field for types that implement 'Drop')

    //  Assigning through a dereference unconditionally drops (assume the referent is initialized)
    //  <(Assigning in a let unconditionally doesn't drop (because we are making a new variable))>

    {
        let mut x = Box::new(0);
        let y = &mut x;
        *y = Box::new(1);               //  deref assumes referent is initialized, so always drops
    }

    {
        let mut x = Box::new(0);        //  no drop
        let mut y = x;                  //  no drop, make x uninit
        x = Box::new(0);                //  no drop
        y = x;                          //  drop y, make x uninit
    }                                   //  drop y

    {
    let x;
        if true {
            x = Box::new(0);
        }
    }   //  need to check drop flag to determine whether 'x' needs to be dropped
}

#[test]
fn unchecked_uninitialized_memory()
{
}

