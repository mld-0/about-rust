//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-12T22:44:54AEDT book examples do not use 'dyn' -> we have to for it to compile ((just) how out of date is the book?) [...] when did the book examples become invalid without 'dyn'? [...] (book is (C) 2018) [...] (what do we need to know about 'dyn' (other than 'put it where Ycm/compiler-errors tells us?)) [...] (taking this question to its conclusion is to ask, how non-trivial would it be to implement Box?)
//  Ongoing: 2022-10-12T22:59:17AEDT Using '.unwrap()' on 'File::create' result works (but using '?' doesn't) (another deviation from book example) [...] (same with result from our 'write_helloWorld' function (we can unwrap it, but not use '?')) [...] (and for that matter -> Rust is allowing us not to unwrap these results without warning/error?)
//  Ongoing: 2022-10-12T23:00:18AEDT 'File::create' happily (and without error/warning) overwrites an existing file
//  Ongoing: 2022-10-12T23:27:46AEDT when passing variable to function as '&mut dyn Write' -> how to get type of what we actually passed (or, what is going on vis-a-vis the rule: can't assign types that support Write to Write) [...] (what is actually going on is we pass a reference, which *doesn't* have to be the same type?)
//  Ongoing: 2022-10-12T23:40:48AEDT 'trait object' vs 'trait object reference' (as used by book) (they mean the later when the say the former?)
//  Ongoing: 2022-10-13T00:10:36AEDT 'where clauses' (were something introduced in the first part of the chapter) (we missed them? what are they?)
//  Ongoing: 2022-10-13T00:35:12AEDT writing a custom Result/Error <type>, (why/how)
//  Ongoing: 2022-10-13T00:39:08AEDT relationship between box/reference (putting something in a box means we are going to be accessing it through references) (but also Box<TraitObject> can be used to store anything that implements TraitObject(?) (just as a &TraitObject can) (what is going on here))
//  Ongoing: 2022-10-13T01:32:55AEDT presumedly (default methods are (meant to be) defined in terms of non-default methods?) ('write_all' is defined in terms of 'write') (what is the difference between them (both recieve an array of bytes?)) [...] (book gives example of Iterator, which has one required / dozens of default methods (and a promise to explain in ch15))
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
//  Continue: 2022-10-13T00:06:23AEDT de-comment (complete) examples which are commented (required traits not available)

use std::fs::File;
use std::env::temp_dir;
use std::io::Write;
use std::fmt::Debug;
use std::hash::Hash;

//  Polymorphism is the ability to write code that operates on many different types
//  Rust provides polymorphism through traits and generics.

//  Traits are Rust's implementation of abstract base classes (interfaces)
//  Rust also uses traits for operator overloading, and <(other features)>
fn example_traits()
{
    let mut path_output = temp_dir();
    path_output.push("programming-rust.11-traits-and-generics.hello-world-1.txt");
    println!("path_output=({})", path_output.as_path().display());

    //  The trait for writing bytes is 'std::io::Write'
    //  It provides, among others, the methods:
    //      fn write(&mut self, buf: &[u8]) -> Result<usize>
    //      fn flush(&mut self) -> Result<()>
    //      fn write_all(&mut self, buf: &[u8]) -> Result<()>

    //  'File', 'TcpStream', and 'Vec<u8>' all implement 'std::io::Write'

    //  The 'dyn' keyword is used whenever we are <creating/declaring> trait objects
    //  <>

    //  We can create a function that recieves any type that implements 'Write':
    fn write_helloWorld(out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(b"hello world\n")?;
        out.flush()
    }

    //  Usage with file:
    let mut f = File::create(path_output).unwrap();
    let mut buffer = Vec::<u8>::new();
    //  Rust automatically converts ordinary references into trait objects where needed:
    write_helloWorld(&mut f);
    write_helloWorld(&mut buffer);
    assert_eq!(buffer, b"hello world\n");

    //  Using traits:
    //  A trait is a feature that a given type may/may-not support (usually something the type can do).
    //      std::io::Write          can write bytes
    //      std::iter::Iterator     can produce a sequence of values
    //      std::clone::Clone       can make clones of itself in memory
    //      std::fmt::Debug         can be printed with the '{:?}' format specifier
    //  Examples of implementation:
    //      std::fs::File           implements Write
    //      Range<i32>              implements Iterator
    //      most std types          implement Clone, Debug

    //  To use trait methods on a given type, the trait itself must be in scope:
    buffer.write_all(b"asdf").unwrap();         //  requires 'std::io::Write' in scope
    //  (this is to prevent conflicts naming when adding new methods to types)
    //  (Clone/Iterator are always in scope)

    //  There is no virtual method overhead when calling trait methods on a type directly
    //  (there is however when calling through the trait type, eg: '&mut dyn Write')

    //  <(We cannot assign types that support a trait object to variables of that object)>
    //  (variable sizes must be known at compile time)
    //let writer: Write = buffer;               //  error

    //  We can however assign such an object to a reference of the trait type
    let writer: &mut dyn Write = &mut buffer;
    //  (any calls to this reference incur virtual method overheads)
    //  (we cannot convert back from a trait object reference to a concrete type)
    //  <(or even query the actual type)>

    //  Trait objects are fat pointers. They contain a pointer to the value itself, and another to a virtual table (vtable) (which in turn points to that object's implementations of the trait methods)
    //  (meaning a trait object reference is two machine words)
    //  This fat pointer, and the vtable are Rust implementation details (which should not be relied upon)

    //  <(These types, when used with types like 'Box' act like references)>
    let w: Box<dyn Write> = Box::new(f);
    //  <(Here 'Box<dyn Write' acts like '&mut dyn Write')>

    drop(w);
    println!("example_traits, DONE");
}

fn example_generics()
{
    let mut path_output = temp_dir();
    path_output.push("programming-rust.11-traits-and-generics.hello-world-2.txt");
    println!("path_output=({})", path_output.as_path().display());

    //  A generic function or type can be used with values of many different types
    //  The compiler generates machine code for each type a generic function/type is used with

    //  Rust makes us declare the bounds of parameterised types - that is, what traits they must implement
    //  <T: Ord> denotes that T is an ordered type. We must declare this to use the comparison operators on it.
    fn min<T: Ord>(a: T, b: T) -> T {
        if a <= b { a } else { b }
    }

    //  Rewriting our trait type reference function as a generic function
    fn write_helloWorld<T: Write>(out: &mut T) -> std::io::Result<()> {
        //  (unlike a trait type reference function, we can determine the type passed)
        //println!("{}", std::any::type_name::<T>());
        out.write_all(b"hello world\n")?;
        out.flush()
    }
    let mut f = File::create(path_output).unwrap();
    let mut buffer = Vec::<u8>::new();
    write_helloWorld(&mut f).unwrap();
    write_helloWorld(&mut buffer).unwrap();
    assert_eq!(buffer, b"hello world\n");

    //  It is possible to explicitly supply the type parameter:
    write_helloWorld::<File>(&mut f).unwrap();
    write_helloWorld::<Vec<u8>>(&mut buffer).unwrap();

    //  (and sometimes necessary where there are no arguments to deduce type)
    //let v1 = (0..1000).collect();             //  error, can't infer type
    let v2 = (0..1000).collect::<Vec<i32>>();

    //  Supplying multiple (bounds) traits for a type parameter:
    fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    }

    //  Generic functions can have multiple type parameters:
    //fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: R) -> Results 
    //{}

    //  Alternatively:
    //fn run_query<M,R>(data: &DataSet, map: M, reduce: R) -> Results 
    //    where M: Mapper + Serialize,
    //        R: Reducer + Serialize
    //{}
    //  (this same syntax can be used wherever bounds are permitted for generic types)

    //  A generic function can have both lifetime parameters and type parameters:
    //  (lifetime parameters come first)
    //fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
    //    where P: MeasureDistance
    //{}
    //  <(lifetimes do not impact machine code, they are provided for checking by the compiler)>

    //  Enums and Structs can also use type parameters 

    //  Indervidual methods can be generic (even if the type they are defined on is not):
    //struct PancakeStack {};
    //impl PancakeStack {
    //    fn push<T: Topping>(&mut self, goop: T) -> PancakeResult<()> {
    //    }
    //}
    //  Type aliases can be generic
    //type PancakeResult<T> = Result<T,PancakeError>;

    drop(f);
    println!("example_generics, DONE");
}


fn example_traitsVsGenerics() 
{
    //  (book assertion: generics are the more common choice over trait objects)

    trait Vegetable { }

    //  Using a generic type does not allow collections of mixed types:
    struct SaladGeneric<V: Vegetable> {
        contents: Vec<V>,                       //  all entries must have same type
    }

    //  Trait objects allow for collections of mixed types
    struct SaladTraitObject {
        contents: Vec<Box<dyn Vegetable>>,
    }

    //  Trait object functions/type reduce code bloat as there is no need for the compiler to generate machine code for each type we use
    //  Trait objects allow for collections of mixed types

    //  Generic functions do not have the overhead of virtual-function-style vtable lookups
    //  Generic functions also permit optimisations that are not possible for trait object functions

    //  <(Some things cannot be implemented with traits)>
    //  <(Not all traits support trait objects)>

    println!("example_traitsVsGenerics, DONE");
}


fn example_definingTraits()
{
    use std::ops::Range;

    struct Canvas {
    }
    struct Broom {
        x: i32, y: i32, height: i32,
    }
    impl Canvas {
        fn write_at(&mut self, x: i32, y: i32, c: char) { }
    }

    //  To define a trait, provide a list of method signatures
    trait Visible {
        fn draw(&self, canvas: &mut Canvas);
        fn hit_test(&self, x: i32, y: i32) -> bool;
        //  default implementation:
        fn not_required(&self) {
        }
    }

    //  To implement a trait:
    //  (all the trait methods, and only the trait methods may be implemented in this block)
    impl Visible for Broom {
        fn draw(&self, canvas: &mut Canvas) {
            for y in self.broomstick_range() {
                canvas.write_at(self.x, y, '|');
            }
        }
        fn hit_test(&self, x: i32, y: i32) -> bool {
            true
        }
    }
    //  (for other methods, use a regular impl block)
    impl Broom {
        fn broomstick_range(&self) -> Range<i32> {
            self.y - self.height-1 .. self.y
        }
    }

    println!("example_definingTraits, DONE");
}


fn example_defaultMethods()
{
    use std::io::{Write,Result};
    pub struct Sink;

    //  We are only required to implement the traits for which a default implementation is not provided:
    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
        //  allowed, but not required: 
        //fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        //    Ok(())
        //}
    }
    //  (default implementations are implementations provided in the 'trait' block)

    println!("example_defaultMethods, DONE");
}


fn example_traitsAndExistingTypes()
{
    //  <>
    println!("example_traitsAndExistingTypes, DONE");
}


fn main() 
{
    example_traits();
    example_generics();
    example_traitsVsGenerics();
    example_definingTraits();
    example_defaultMethods();
    example_traitsAndExistingTypes();
}

