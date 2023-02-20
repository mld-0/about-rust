//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-20T21:11:25AEDT <(from a previous item)> why not to implement 'From' / (alternatives?)
//  Ongoing: 2023-02-20T21:16:22AEDT Rust, meaning of 'concrete context'? [...] (non-generic?)
//  Ongoing: 2023-02-20T21:16:34AEDT "a type will provide a static new" -> (is this describing a convention, or some implicit factory function named 'new()'?) 
//  Ongoing: 2023-02-20T21:47:54AEDT "If a variable has only been partially initialized, only initialized fields are dropped" -> (when this can take place? ('Foo { a: f(), b: g(), }' and 'f()' panics?))
//  Ongoing: 2023-02-20T21:51:31AEDT 'mem::drop()' vs 'std::ptr::drop_in_place()'(?)
//  }}}

//  Continue: 2023-02-20T21:24:04AEDT complete chapter

//  Resource Acquisition Is Initialization (RAII): ownership based resource management
//  Resources are owned by a managing object, whose ctor initializes the resource and dtor cleans it up.

#[test]
fn constructors()
{
    struct Foo { a: u8, b: u32, c: bool, };
    enum Bar { X(u32), Y(bool), };
    struct Unit;

    //  Rust does not provide constructors
    //  To initialize an object, name it and initialize all its fields
    let foo = Foo { a: 0, b: 1, c: false, };
    let bar = Bar::X(0);
    let empty = Unit;

    //  There are no copy/move ctors either
    //  Every type must be ready to be blindly memcopy-ed elsewhere in memory
    //  (Safe Rust does not allow a simple on-the-stack moveable linked-list implementation)


    //  Implementing 'Clone' is Rust's equivalent of a copy-ctor (deep copy)
    //  It is never implicitly invoked, it must be explicitly called: 'let y = x.clone()'
    pub trait Eg_Clone {
        fn clone(&self) -> Self;
    }


    //  A trait implements 'Copy' to indicate that its 'Clone' implementation is a simple bitwise copy
    //  Assignment is a copy for 'Copy' types <(for which '.clone()' is implicitly called)>
    //  (Types with a dtor cannot be Copy)
    pub trait Eg_Copy: Clone {}


    //  <(Rust previously provided 'std::marker::NoCopy' as a field to make structs non-copyable)>
    //  <(deprecated, 'structs are by default not copyable'?)>
    //  <(just add a dtor?)>


    //  Assignment and Copy/Clone: (copys are moves and moves are copys)
    //  {{{
    //  LINK: https://stackoverflow.com/questions/31168589/how-to-force-a-move-of-a-type-which-implements-the-copy-trait
    //          1)  let a = b           (b is not Copy)
    //  ('b' is memcpy-ed into 'a')
    //          2)  let a = b           (b is Copy)
    //  ('b' is memcpy-ed into 'a')
    //          3)  let a = b.clone()   (b is Clone)
    //  ('b.clone()' is memcpy-ed into 'a')
    //
    //  (The difference between 1/2 is whether 'b' can still be used after the assignment)
    //
    //  Consider 1/2/3 for 'Vec { &mut data, length, capacity }'
    //
    //  <(Rust guarantees elision of certain trivial moves/copies)>
    //
    //  }}}


    //  Rust allow for factory functions in place of constructors:
    impl Foo {
        fn new(a: u8, b: u32, c: bool) -> Self {
            Foo { a, b, c, }
        }
    }
    let x = Foo::new(1,2,false);

    //  Implement 'From' to define conversion from another type
    impl From<(u8,u32,bool)> for Foo {
        fn from(item: (u8,u32,bool)) -> Self {
            Foo { a: item.0, b: item.1, c: item.2, }
        }
    }


    //  For types with sensible default values, Rust provides 'std::default::Default'
    impl Default for Foo {
        fn default() -> Self {
            Foo { a: u8::default(), b: u32::default(), c: bool::default(), }
        }
    }
    //  <(In concrete contexts, a type will provide a static 'new' method for any kind of default ctor)>
}


#[test]
fn destructors()
{
    //  Rust provides destructors through the 'std::ops::Drop' trait
    trait Eg_Drop {
        fn drop(&mut self);
    }
    //  This function 'drop()' is called whenever the value is dropped

    //  Type that implement 'Drop' cannot be 'Copy'

    //  When a value is dropped, the dtors of all its fields are called recursively
    //  (Regardless of whether 'value' implements 'Drop')
    //  (No way to prevent this in Rust 1.0)
    //  (Because of this, most types do not need a custom dtor)

    //  Dropping fields manually in a dtor will result in a double-free when they are dropped implicitly
    //  (If it is necessary to do so, make the field in question Option<T>, and set it to None)

    //  A value cannot call '.drop()' on itself
    //  (Use 'mem::drop(value)' instead) <(or 'std::ptr::drop_in_place()')>

    //  Drop order:
    //      struct/enum/tuple member variables are dropped in the order they are declared
    //      array/owned-slice elements are dropped first->last
    //      local variables are dropped in reverse order
    //      closure variables are dropped in an unspecified order

    //  Any panic in a drop implementation will likely abort

    //  <(If a variable has only been partially initialized, only initialized fields are dropped)>

    //  To prevent a dtor being run:
    //          std::mem::forget
    //          std::mem::ManuallyDrop
}

