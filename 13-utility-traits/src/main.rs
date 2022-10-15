//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-16T01:50:56AEDT (cannot create an unsized type directly), takeaway -> we cannot create an instance of an unsized type? (unsized types are only ever pointers to sized types?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
//  Continue: 2022-10-16T02:09:53AEDT when to make a type copy

//  Utility traits:
//      <>Drop                  Destructors
//      <>Sized                 Mark type as fixed size known at compile time
//      <>Clone                 Cloning values <(deep copy)>
//      <>Copy                  bitwise copy <(shallow copy)>
//      <>Deref/DerefMut        dereference custom pointer type
//      <>Default               Types with a 'default' value
//      <>AsRef/AsMut           Borrowing references of one type from another
//      <>Borrow/BorrowMut      Like AsRef/AsMut, but guaranteeing consistent hashing/ordering/equality
//      <>From/Into             Converting one type to another
//      <>ToOwned               Converting a reference to an owned value


fn example_drop()
{
    //  Values are dropped when their owner goes away
    //  This is generally automatic when a variable goes out of scope
    //  It is usually only neccessary to implement Drop for resource management classes

    //  A type that implements 'Drop' cannot implement 'Copy'
    #[derive(Debug)]
    struct Appellation {
        name: String, nicknames: Vec<String>,
    }
    impl Appellation {
        fn new(name: String, nicknames: Vec<String>) -> Self {
            Appellation { name, nicknames }
        }
    }

    trait ExampleDrop {
        fn drop(&mut self);
    }

    //  Customise how a given value is dropped by implementing 'std::ops::Drop'
    //  (this is Rust's equivalent of a destructor)
    impl Drop for Appellation {
        fn drop(&mut self) {
            println!("dropping Appellation=({:?})", self);
        }
    }

    //  We can explicitly drop a variable with the function 'drop()'
    //  (this function is trivial)
    fn ExampleDrop<T>(_x: T) {}

    let x = Appellation::new("Zeus".to_string(), vec!["cloud collector".to_string(), "king of the gods".to_string()]);
    drop(x);

    println!("example_drop, DONE");
}


fn example_sized()
{
    use std::fmt::Display;

    //  A 'sized type' is a type whose values all have the same size in memory
    //  (almost all types are sized)

    //  Unsized values cannot be stored in variables or passed as arguments, 
    //  they can only be <accessed> through pointers (which are themselves sized types).
    //  Pointers to unsized values are always fat pointers - they must contain the value's size.
    //  (and in the case of a pointer to a trait object, a pointer to a vtable of method implementations)

    //  (Only sized types can be returned from functions)

    //  String and array slices (str / [T]) are unsized 

    //  The referent of a trait object is unsized
    //  &std::io::Write and Box<std::io::Write> are pointers to some value that implements Write
    //  (this value may be any size)

    //  All sized types implement 'std::marker::Sized' (this is done automatically for custom types)
    //  (When used as a bound, 'Sized' requires the type's size to be known at compile time)
    //  (this is an example of a 'marker trait')

    //  Generic types are sized by default
    //  Equivalent:
    //      S<T>
    //      S<T: Sized>

    //  To specify a type that is questionably (not required to be) sized:
    //      S<T: ?Sized>

    //  Only the last field of a struct can be questionably sized:
    struct ExampleRcBox<T: ?Sized> {
        ref_count: usize,
        value: T,
    }
    //  (the resulting struct may be sized/unsized depending on type T)

    fn display(boxed: &ExampleRcBox<dyn Display>) { 
        println!("For your enjoyment: {}", &boxed.value) 
    }

    //  <(We cannot create an unsized type directly. Instead, create a sized value that implements the unsized type, and convert the sized reference to an unsized reference)>
    let boxed_lunch: ExampleRcBox<String> = ExampleRcBox { ref_count: 1, value: "lunch".to_string() };
    let boxed_displayable: &ExampleRcBox<dyn Display> = &boxed_lunch;
    //  (this conversion is implicit)

    display(&boxed_lunch);
    display(&boxed_displayable);

    println!("example_sized, DONE");
}

fn example_clone()
{
    //  Clone should construct an independent <deep> copy of self and return it
    //  Only sized types can implement clone

    //  Cloning can be expensive (hence why it must be performed explicitly)

    //  'clone_from()' permits optimisations that may not be possible with 'clone'

    trait ExampleClone: Sized {
        fn clone(&self) -> Self;
        fn clone_from(&mut self, source: &Self) {
            *self = source.clone();
        }
    }

    //  Clone can be automatically implemented for a given type
    #[derive(Clone)]
    struct MyComplex<T> {
        re: T, im: T,
    }

    //  Types for which <deep copying> is meaningful should implement clone

    println!("example_clone, DONE");
}

fn example_copy()
{
    //  <(copy vs clone)>
    //  copy is a marker trait for types which can be shallow <bitwise> copied 
    //  A type that implements 'drop' cannot be copyable

    //  'std::marker::Copy'
    trait ExampleCopy: Clone { }

    //  Copy can be automatically implemented for a given type (if it is Cloneable)
    #[derive(Clone,Copy)]
    struct MyComplex<T> {
        re: T, im: T,
    }

    //  <(Later changing whether a type is copyable is problematic)>

    //  <(A type should be made copy if <>)>

    println!("example_copy, DONE");
}

fn main() 
{
    example_drop();
    example_sized();
    example_clone();
    example_copy();
}

