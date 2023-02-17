//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::mem::{self, MaybeUninit};
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-17T18:20:31AEDT 'Rust tracks whether a variable should be dropped at runtime with a flag on the stack' (does it *always*? Surely many such runtime checks could be optimized away?) 
//  Ongoing: 2023-02-17T18:29:00AEDT re-creating a variable (of the same name) with 'let' - when does the old one get dropped?
//  Ongoing: 2023-02-17T21:50:42AEDT (does Rust have syntax for) initializing an array by supplying the first N elements and setting every element after that to be a default/zero value?
//  Ongoing: 2023-02-17T21:52:05AEDT is it 'safe Rust' or 'Safe Rust'?
//  Ongoing: 2023-02-17T21:56:29AEDT role of 'const var' (in a language where 'let var' also declares a constant (... for one thing the former can't be re-declared)?) [...] (specifically for our MaybeUninit array example would 'let' work?) [...] ('const' is a compile time constant, and MaybeUninit does not work with 'let' declared value)
//  Ongoing: 2023-02-17T22:01:59AEDT 'Box cannot panic' - owned pointer on the heap (<surely> must panic when there's no memory left?)
//  Ongoing: 2023-02-17T22:04:38AEDT 'x[i] = var' being pointer assignment (that's a thing generally or just for MaybeUninit?)
//  Ongoing: 2023-02-17T22:45:09AEDT chapter 'initalize an array element by element' example places each array element in a Box (producing '[Box<u32>;SIZE]') - (presumedly taken from 'MaybeUninit' docs, where they do something similar but with a single-element vector?) [...] (swapping s/Box<u32>/u32/ produces an array which is valid as per assert_eq! and miri?)
//  Ongoing: 2023-02-17T23:14:24AEDT unsafe blocks should be as short as possible? (prefer multiple unsafe blocks to a single lengthy one?) (... or is 'MaybeUninit::<Demo>' example doing this because it's a neater way to assign the results of an unsafe function?)
//  }}}

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
    //  Assume doing virtually anything with uninitalized memory is UB

    //  Safe Rust doesn't allow partial initialization of an array.
    //          Either supply every value:          let x = [v1, v2, v3]
    //          Or a single value and length:       let x = [val; len];


    //  'mem::uninitialized()' is deprecated (use 'mem::MaybeUninit' instead)

    //  'std::mem::MaybeUninit<T>'
    //  A wrapper type to construct uninitialized instances of 'T'
    //  (Very unsafe option for when Safe Rust is to restrictive)

    //  (Must be not allow uninitalized values to be dropped in any code path (including panicking))
    //  <(Violating the invariants of any type is UB)>
    //  (Initialising an invalid (misaligned/null) reference is UB, whether that reference is used or not)

    //  Example: initialize an array element by element
    const SIZE: usize = 10;
    let x = {
        //  Create uninitialized array
        let mut x: [MaybeUninit<u32>; SIZE] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        //  Do the initialization
        //  <(Dropping 'MaybeUninit' does nothing (hence we can use raw pointer assignment instead of 'ptr::write') (Exception safety not an issue because 'Box' cannot panic))>
        //  (Use 'ptr::{write, copy, copy_nonoverlapping}' if 'MaybeUninit::new(x)' cannot be used <(which is where it creates something that is invalid to drop)>)
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(i as u32);
        }
        //  Transmute to initialized type
        unsafe { std::mem::transmute::<_, [u32; SIZE]>(x) }
    };
    assert_eq!(x, [0_u32,1,2,3,4,5,6,7,8,9]);

    //  From chapter example (which uses Box<u32> instead of u32
    //  (The expression '*x[i].as_mut_ptr() = Box::new(i as u32)' drops the previous value of '*x[i]', which is an uninitalized Box (invalid to drop) ... (but dropping an uninitalized i32 is ok?))
//      unsafe { *x[i].as_mut_ptr() = Box::new(i as u32); }         //  wrong as per Miri
//      unsafe { *x[i].as_mut_ptr() = i as u32; }                   //  valid as per Miri


    //  It is illegal to construct a reference to uninitialized data
    //  (another method must be used to get a pointer to said data)
    //          For an array of T, use 'base_ptr.add(idx)'
    //          For a struct, use the 'addr_of_mut!(struct.field)' macro
    struct Demo { field: bool, };
    let mut uninit: MaybeUninit<Demo> = MaybeUninit::<Demo>::uninit();
    let f1_ptr: *mut bool = unsafe { std::ptr::addr_of_mut!((*uninit.as_mut_ptr()).field) };
    unsafe { f1_ptr.write(true); }
    let init: Demo = unsafe { uninit.assume_init() };
}

