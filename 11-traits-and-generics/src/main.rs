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
//  Ongoing: 2022-10-14T00:05:00AEDT subtraits overriding (things from) their <parent> trait?
//  Ongoing: 2022-10-14T22:31:15AEDT subtraits <and/or> inheritance(?)
//  Ongoing: 2022-10-14T22:38:46AEDT (qualified calls can be needed for (a type with) two methods of the same name) -> Rust allows this?
//  Ongoing: 2022-10-14T22:40:34AEDT what does '<str as ToString>::to_string()' provide that 'str::to_string()' doesn't?
//  Ongoing: 2022-10-14T22:41:56AEDT (how to know if a method call is 'fully' qualified?)
//  Ongoing: 2022-10-14T22:49:47AEDT Iterator defines 'type Item' (instead of using a type parameter) (why?) [...] (it is a type which is not intended to be parameterised in the implementing class?) [...] (<is that> what the keyword 'type' means in the context of a trait (can structs have them)?) [...] ~~(we do not use 'Self::Item' in the class implementing the trait?~~ -> actually we can (but book example does not))
//  Ongoing: 2022-10-14T23:23:24AEDT make something iterable (without implementing 'Iterator')
//  Ongoing: 2022-10-14T23:58:57AEDT 'T::default()' provides a 0 of any numeric type -> (what is the best way to get a '3' of any numeric type?) [...] (does any trait make '0 as T' available?)
//  Ongoing: 2022-10-15T00:31:19AEDT is the default Output type of Add/Mul not Self (why do we have to specify 'T: Add<Output=T>' for 'dot()'(?))
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
//  Continue: 2022-10-13T00:06:23AEDT de-comment (complete) examples which are commented (required traits not available)
//  Continue: 2022-10-14T22:12:26AEDT explain 'dyn' keyword

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
    //  Rust automatically converts ordinary references into <(references to)> trait objects where needed:
    write_helloWorld(&mut f);
    write_helloWorld(&mut buffer);
    let p_buffer: &mut dyn Write = &mut buffer;
    drop(p_buffer);
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
        println!("type(out)=({})", std::any::type_name::<T>());
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
    //  (or)
    fn alt_top_ten<T>(values: &Vec<T>) 
        where T: Debug + Hash + Eq {
    }

    trait Mapper {};
    trait Reducer {};
    trait Serialize {};
    struct DataSet {};
    struct Results {};

    //  Generic functions can have multiple type parameters:
    fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: R) -> Results 
    {
        Results {}
    }
    //  Alternatively:
    fn run_query_alt<M,R>(data: &DataSet, map: M, reduce: R) -> Results 
        where M: Mapper + Serialize,
            R: Reducer + Serialize
    {
        Results {}
    }
    //  (this same syntax can be used wherever bounds are permitted for generic types)

    struct MeasureDistance {};

    //  A generic function can have both lifetime parameters and type parameters:
    //  (lifetime parameters come first)
    //fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
    //    where P: MeasureDistance
    //{
    //}
    //  <(lifetimes do not impact machine code, they are provided for checking by the compiler)>

    //  Enums and Structs can also use type parameters 

    trait Topping {};
    struct PancakeError {};

    //  Indervidual methods can be generic (even if the type they are defined on is not):
    struct PancakeStack {};
    impl PancakeStack {
        fn push<T: Topping>(&mut self, goop: T) -> Result<(),PancakeError> {
            Ok(())
        }
    }
    //  Type aliases can be generic
    type PancakeResult<T> = Result<T,PancakeError>;

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
    //  Rust allows traits to be implemented for any type introduced in the current crate
    //  A trait created to add a method to an exiting type is an extension trait

    //  Adding extension trait to a family of types:
    use std::io::{self,Write};
    struct HtmlDocument {};
    trait WriteHtml {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
    }
    //  (Implement WriteHtml for every type W that implements Write)
    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
            Ok(())
        }
    }

    //  Coherence rule: when implementing a trait, either the trait or type must be new in the current crate
    //  (to ensure that trait implementations are unique)

    println!("example_traitsAndExistingTypes, DONE");
}


fn example_Self_in_traits()
{
    //  A trait can use the keyword 'Self' as a type
    pub trait Clone {
        fn clone(&self) -> Self;
    }
    pub trait Spliceable {
        fn splice(&self, other: &Self) -> Self;
    }
    //  <('Self' refers to the type which implements the trait?)>

    //  A trait that uses 'Self' type is incompatible with trait objects
    //fn splice_anything(l: &dyn Spliceable, r: &dyn Spliceable) {              //  error
    //}            
    //  <(this is because the compiler has no way to typecheck l/r)>
    //  (trait objects are intended for the simplest kinds of traits)
    //  <(other features with preclude use of trait objects?)>

    //  <(instead use:)>
    pub trait MegaSpliceable {
        fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
    }
    fn splice_anything(l: &dyn MegaSpliceable, r: &dyn MegaSpliceable) {}

    println!("example_Self_in_traits, DONE");
}


fn example_subtraits()
{
    //  We can declare traits as extensions of another trait
    trait Visible {
    }
    trait Creature: Visible {
    }
    //  (All Creatures are Visible)
    //  (Every type that implements Creature must also implement Visible)

    println!("example_subtraits, DONE");
}


fn example_trait_static_methods()
{
    //  Rust allows traits to have static methods/contructors
    trait StringSet {
        fn new() -> Self;
        fn from_slice(strings: &[&str]) -> Self;
        fn contains(&self, string: &str) -> bool;
        fn add(&mut self, string: &str);
    }
    //  <(static methods are those that don't take an argument 'self'?)>

    //  <(Using a static constructor from a generic type:)>
    fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
        let mut unknowns = S::new();
        for word in document {
            if !wordlist.contains(word) {
                unknowns.add(word);
            }
        }
        unknowns
    }

    //  Trait objects don't support static methods
    //  (unless we add 'Self: Sized' bound to each one)
    trait StringSet2 {
        fn new() -> Self
            where Self: Sized;
        fn from_slice(strings: &[&str]) -> Self
            where Self: Sized;
        fn contains(&self, string: &str) -> bool;
        fn add(&mut self, string: &str);
    }
    //  <(Sized: tells Rust trait objects are excused from supporting this method (see ch13))>

    println!("example_trait_static_methods, DONE");
}


fn example_fullyQualified_methodCalls()
{
    //  Equivalent:
    "hello".to_string();                            //  <(unqualified calls?)>
    str::to_string("hello");                        //  qualified call
    ToString::to_string("hello");                   //  qualified call
    <str as ToString>::to_string("hello");          //  fully qualified call
    //  <(in each case the string is passed as 'self' parameter)>
    //  For the first <(unqualified)> case, Rust uses its method lookup rules to determine the function to call

    //  <(qualified calls can be needed for (a type with) two methods of the same name, or where type is ambigous)>
    let zero = 0;
    //zero.abs();           //  error, type of 'zero' is ambigious
    i64::abs(zero);

    //  <S as StringSet>::new()
    //  <(when/where is this needed)>

    println!("example_fullyQualified_methodCalls, DONE");
}


fn example_traits_and_relationshipsBetweenTypes()
{
    //  Traits can describe the relationship between types
    //      std::iter::Iterator         relates iterators with the type they produce
    //      std::ops::Mul               relates types that can be multiplied
    //      <(rand::Rng and rand::Rand)>

    struct ExampleArgs {
        args: Vec<String>,
    };

    //  Associated types: (type variables in a trait)
    pub trait ExampleIterator {
        type Item;      //  associated type
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl Iterator for ExampleArgs {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            self.args.pop()
        }
    }
    //  (outside traits, 'type' defines aliases for existing types)

    fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
        let mut result = Vec::<I::Item>::new();
        //  (or)
        //let mut result = Vec::new();
        for val in iter {
            result.push(val);
        }
        result
    }

    fn dump<I>(iter: I) 
        where I: Iterator, I::Item: Debug       //  must implement 'Debug' for us to be able to print it with '{:?}'
    {
        for (i,val) in iter.enumerate() {
            println!("{}: {:?}", i, val);
        }
    }
    //  (or)
    fn dump_alt_1<I>(iter: I)
        where I: Iterator<Item=String>
    {
        for (i,val) in iter.enumerate() {
            println!("{}: {:?}", i, val);
        }
    }
    //  (or)
    fn dump_alt_2(iter: &mut dyn Iterator<Item=String>) {
        for (i,val) in iter.enumerate() {
            println!("{}: {:?}", i, val);
        }
    }
    //  (not here yet)
    //fn dump_alt_3(iter: &mut dyn Iterator<Item: Debug>) {

    //  Another example of an associated trait:
    trait ExamplePattern {
        type Match;
        fn search(&self, string: &str) -> Option<Self::Match>;
    }

    println!("example_traits_and_relationshipsBetweenTypes, DONE");
}


fn example_genericTraits_operatorOverloading()
{
    //  Rust defines multiplication with the 'Mul' trait
    //  (note 'Self' is the default type parameter)
    pub trait ExampleMul<RHS=Self> {
        type Output;
        fn mul(self, rhs: RHS) -> Self::Output;
    }

    //  'lhs * rhs' is shorthand for 'Mul::mul(lhs, rhs)'
    //  <(The '*' operator is overloaded by implementing the 'Mul' trait)>

    println!("example_genericTraits_operatorOverloading, DONE");
}


fn example_buddy_traits()
{
    use rand::random;
    use rand::Rng;

    //  Buddy traits are traits designed to work together
    //  'Rng' is a trait that can produce integers on demand
    //  'Rand' is a buddy trait. Types that implement it can use Rnd to implement 'rand(rng)' for themselves
    pub trait ExampleRng {
        fn next_u32(&mut self) -> u32;
    }
    pub trait ExampleRand: Sized {
        fn rand<R: Rng>(rng: &mut R) -> Self;
    }

    //  Another example:
    //  'Hasher' is a trait implemented by hashing algorithms
    //  'Hash' is a trait implemented by types that are hashable

    //  <(this approach helps avoid the need for virtual methods / downcasts)>

    println!("example_buddy_traits, DONE");
}


fn example_reverse_engineering_bounds()
{
    use std::ops::{Add,Mul};

    //  Traits serve to explicitly document what kind of types a generic function is valid for.
    //  (as opposed to C++, where the body of the function defines what types are valid)

    //  Getting a zero value for a generic type
    fn get_zero<T: Default>() -> T { T::default() }
    fn get_zeroAlt<T: num::Num>() -> T { T::zero() }

    //  A generic function requires all the right bounds before it will compile
    //  Here we need to be able use the results of Add/Mul on our type -> we must specify the Output type of each
    fn dot<T>(v1: &[T], v2: &[T]) -> T 
        where T: Add<Output=T> + Mul<Output=T> + Default + Copy
    {
        assert_eq!(v1.len(), v2.len());
        let mut total = T::default();
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];      //  'total' must be copyable
        }
        total
    }
    //  <(still cannot use '+=' (now the compiler is just being dumb?))>
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10); 
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);

    //  'num::Num' solves the same problem: <(must still supply 'Copy'?)>
    fn dotAlt<T: num::Num + Copy>(v1: &[T], v2: &[T]) -> T {
        assert_eq!(v1.len(), v2.len());
        let mut total = T::zero();
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];      //  'total' must be copyable
        }
        total
    }
    //  <(same problem -> cannot use '+=')>
    assert_eq!(dotAlt(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10); 
    assert_eq!(dotAlt(&[53.0, 7.0], &[1.0, 5.0]), 88.0);

    println!("example_reverse_engineering_bounds, DONE");
}


fn main() 
{
    example_traits();
    example_generics();
    example_traitsVsGenerics();
    example_definingTraits();
    example_defaultMethods();
    example_traitsAndExistingTypes();
    example_Self_in_traits();
    example_subtraits();
    example_trait_static_methods();
    example_fullyQualified_methodCalls();
    example_traits_and_relationshipsBetweenTypes();
    example_genericTraits_operatorOverloading();
    example_buddy_traits();
    example_reverse_engineering_bounds();
}

