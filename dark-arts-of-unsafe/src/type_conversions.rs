//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-07T22:52:53AEDT no 'smarter' way to convert a struct into another struct with identical member variables (vs 'type_conversions::reinterpret')? 
//  Ongoing: 2023-02-15T22:32:57AEDT type conversion - defined using 'From' vs <alternatives?> (why not to defined converisons using 'From'?)
//  Ongoing: 2023-02-15T23:36:13AEDT Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent you from implementing both traits on one type ... (hence "Fully Qualified Syntax for Disambiguation") ... (Rust rules on multiple methods of the same name?)
//  Ongoing: 2023-02-15T23:43:25AEDT '<T: Cloned>' is a "trait bound"
//  Ongoing: 2023-02-16T00:04:13AEDT 'autoref': <(when '&self' / '&mut self' is <implicitly> inserted as first function argument)>
//  }}}
use std::sync::Arc;

//  Continue: 2023-02-15T23:54:47AEDT 'value.clone()' behaviour with/without 'T: Cloned' trait bound
//  Continue: 2023-02-16T00:06:50AEDT quick-and-dirty "when does type coercion take place" guide
//  Continue: 2023-02-16T00:07:47AEDT complete chapter

#[test]
fn type_conversions() 
{
    //  There are two common problems with typing bits
    //          Reinterpreting those same bits as a different type
    //          Changing the bits to have an equivalent meaning for a different type

    //  Rudimentary conversion between custom types:
    struct Foo { x: u32, y: u16, }
    struct Bar { a: u32, b: u16, }
    fn reinterpret(foo: Foo) -> Bar {
        Bar { a: foo.x, b: foo.y, }
    }
}


#[test]
fn coercions()
{
    //  (Except for receivers) we do not perform type coercions when matching traits
    //  If U is implemented for a trait and T coerces to U, that doesn't constitute an implementation for T

    //  <(quick and dirty what-you-need-to-know about type coercions?)>

    //  Most type coercions involving weakening of types
    //  (with an emphasis on making Rust "just work")


    //  Type coercions definitive guide (see below)
    //  LINK: https://doc.rust-lang.org/nightly/reference/type-coercions.html
    //  {{{
    //  }}}
}


#[test]
fn dot_operator()
{
    //  The dot operator '.' will perform a lot of magic to convert types: 
    //  auto-referencing/dereferencing and coercing until types match

    //  Behaviour: 'value.foo()' (where 'value' is type 'T')
    //      1)  Check 'T::foo(value)' 
    //      2)  Check '<&T>::foo(value)' / '<&mut T>::foo(value)'
    //      3)  Deref/Unsize 'T' and try again
    //                  Deref uses 'T: Deref<Target=U>'
    //                  Unsizing: eg, convert '[i32; 2]' to '[i32]'

    //  (This occurs in more places than it appears, eg: 'array[0]' becomes 'array.index(0)')


    //  Example: 'value.clone()' behaviour with/without 'T: Clone' trait bound 
    //
    //  <(with 'Clone' trait bound given, we <first> attempt <?>)>
    //  <(calls: 'pub fn clone(&self) -> Self')>
    //  'cloned' type: T
    fn do_clone_i<T: Clone>(value: &T) {
        let cloned = value.clone();
    }
    //
    //  <(If we remove the 'Clone' trait bound, we <first/instead> try to call by autoref)>
    //  <(calls: 'pub fn clone(&&self) -> &Self')> <(Self = '&T')>
    //  'cloned' type: &T
    fn do_clone_ii<T>(value: &T) {
        let cloned = value.clone();
    }


    //  <(<default> auto-derived 'Clone' <behaviour/implementation>)>
    //  <>


    //  Example: <?> with auto-derived / custom 'Clone' implementation
    //  <(where auto behaviour for auto-derived function is <wrong?>)>
    //
    #[derive(Clone)]
    struct MyContainer_i<T>(Arc<T>);
    //
    //  <(auto-derived 'Clone' implementation equivalent to:)>
    struct MyContainer_ii<T>(Arc<T>);
    impl<T> Clone for MyContainer_ii<T> where T: Clone
    {
        fn clone(&self) -> Self {
            Self( Arc::clone(&self.0) )
        }
    }
    //
    //  <(problem is fixed by implementing 'Clone' without requiring trait bound 'T: Clone'
    struct MyContainer_iii<T>(Arc<T>);
    impl<T> Clone for MyContainer_iii<T> 
    {
        fn clone(&self) -> Self {
            Self( Arc::clone(&self.0) )
        }
    }
    //  
    //  <(Usage:)>
    fn clone_mycontainer_i<T>(foo: &MyContainer_i<i32>, bar: &MyContainer_i<T>) {
        //  <(calls?)>
        let foo_cloned = foo.clone();       //  T = <?>
        //  <(calls?)>
        let bar_cloned = bar.clone();       //  T = <?>
    }
    fn clone_mycontainer_ii<T>(foo: &MyContainer_ii<i32>, bar: &MyContainer_ii<T>) {
        //  <(calls?)>
        let foo_cloned = foo.clone();       //  T = <?>
        //  <(calls?)>
        let bar_cloned = bar.clone();       //  T = <?>
    }
    fn clone_mycontainer_iii<T>(foo: &MyContainer_iii<i32>, bar: &MyContainer_iii<T>) {
        //  <(calls?)>
        let foo_cloned = foo.clone();       //  T = <?>
        //  <(calls?)>
        let bar_cloned = bar.clone();       //  T = <?>
    }


    //  Dot operator (Method lookup) definitive guide (see below)
    //  LINK: https://rustc-dev-guide.rust-lang.org/method-lookup.html
    //  {{{
    //  }}}
}


#[test]
fn casts()
{
}


#[test]
fn transmutes()
{
}

