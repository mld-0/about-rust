//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-09T19:57:47AEDT how similar is the resulting assembly when replacing references with raw pointers?
//  Ongoing: 2023-02-09T20:00:00AEDT Rust, compiler explorer doesn't support hiding library functions for Rust?
//  Ongoing: 2023-02-09T20:55:37AEDT YCM picks '_ = &mut x;' as invalid (but it compiles and runs?) [...] (change it to 'let _' it it stops complaining)
//  Ongoing: 2023-02-09T21:24:04AEDT YCM cannot determine the pointer type of a pointer declared as 'let pfoo = &mut foo as *mut _' -> it requires s/_/Foo/
//  Ongoing: 2023-02-09T21:29:30AEDT contention: Rust rules which permit pointer_aliasing_different_pointer_types / pointer_aliasing_partial_overlapping C++ UB behaviour might change (and make said behaviour invalid in Rust) in the future(?)
//  Ongoing: 2023-02-09T21:32:17AEDT differences between 'array.as_mut_ptr()' and '&mut array[0] as *mut _'?
//  Ongoing: 2023-02-09T21:46:21AEDT contention: there are many more rules for Unsafe Rust than covered here
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
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

//	Unsafe Rust is not C (Rust examples):
//  LINK: https://www.youtube.com/watch?v=DG-VLezRkYQ

fn first()
{
    //  Rejected: cannot call mutable method on 'v' while 'pv' exists
//  let mut v = vec![1,2,3,4];
//  let pv = &v[0];
//  v.push(5);
//  println!("{}", *pv);

    //  Detecting UB in Compiler Explorer:
    //          -Zsanitizer=address
    //  Detecting UB with Cargo:
    //          RUSTFLAGS="-Z sanitizer=address" cargo +nightly run

    //  Using unsafe to force Rust to compile above
    //  Produces heap-use-after-free with address sanitiser
    //  (silently gives wrong result when compile without)
    let mut v = vec![1,2,3,4];
    let pv = &v[0] as *const _;
    v.push(5);
    println!("pv=({})", unsafe { *pv });
}

fn second()
{
    //  Build optimization:
    //          -C opt-level=3              (compiler explorer)
    //          cargo run --release

    //  Compiler assumes 'x' / 'y' will never alias 
    //  (and is free to optimise function to always return 42)
    pub fn foo_i(x: &mut i32, y: &mut i32) -> i32 {
        *x = 42;
        *y = 99;
        *x
    }

    //  No such restriction on raw pointers:
    pub fn foo_ii(x: *mut i32, y: *mut i32) -> i32 {
        unsafe {
            *x = 42;
            *y = 99;
            *x
        }
    }

    let mut n = 0;
    //println!("{}", foo_i(&mut n, &mut n));        //  invalid, multiple mutable borrows

    //  valid(?) (miri doesn't object)
    //  (produces correct result for both debug/release build)
    let mut n = 0;
    let pn = &mut n as *mut _;
    println!("{}", foo_ii(pn, pn));


    //  UB: (according to miri this counts as) multiple mutable borrows
    //  (produces correct result for both debug/release build)
    let mut n = 0;
    let pn1 = &mut n as *mut _;
    let pn2 = &mut n as *mut _;
    println!("{}", foo_ii(pn1, pn2));


    //  UB: using unsafe to produce multiple mutable references to the same value
    let mut n = 0;
    let pn = &mut n as *mut _;
    let n_mut1 = unsafe { &mut *pn };
    let n_mut2 = unsafe { &mut *pn };
    println!("{}", foo_i(n_mut1, n_mut2));
    //  (outputs '99' in debug builds / '42' in release builds)
}


fn third()
{
    let mut x = 42;
    let px = &x as *const _;
    let _ = &mut x;                     //  'px' has been invalidated
    println!("{}", unsafe { *px });
}


fn pointer_aliasing_different_pointer_types() 
{
    #[repr(C)]
    pub struct Foo { x: i32 }
    #[repr(C)]
    pub struct Bar { x: i32 }

    //  <(Unlike C++, Rust does not assume pointers of different types cannot alias)>
    pub fn f(foo: *mut Foo, bar: *mut Bar) -> i32 {
        unsafe {
            (*foo).x = 42;
            (*bar).x = 99;
            (*foo).x
        }
    }

    //  (not invalid as per Miri)
    let mut foo = Foo { x: 0 };
    let pfoo = &mut foo as *mut Foo;
    let output = f(pfoo, pfoo as *mut Bar);
    println!("{}: output=({})", get_func_name!(), output);
}


fn pointer_aliasing_partial_overlapping()
{
    #[repr(C)]
    pub struct Foo { x: i32, y: i32, }

    //  <(Unlike C++, Rust does not assume pointers cannot partially overlap)>
    pub fn f(foo1: *mut Foo, foo2: *mut Foo) -> i32 {
        unsafe {
            (*foo1).y = 42;
            (*foo2).x = 99;
            (*foo1).y
        }
    }

    //  (not invalid as per Miri)
    let mut a = [0_i32; 3];
    let pa = a.as_mut_ptr();
    let f1 = pa as *mut Foo;
    let f2 = pa.wrapping_add(1) as *mut Foo;
    let output = f(f1, f2);
    println!("{}: output=({})", get_func_name!(), output);
}


fn main() 
{
    first();
    second();
    third();
    pointer_aliasing_different_pointer_types();
    pointer_aliasing_partial_overlapping();
}

