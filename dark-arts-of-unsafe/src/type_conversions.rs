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
//  Ongoing: 2023-02-16T21:30:34AEDT 'where V: Sized' (means V is *not* optionally sized?)
//  Ongoing: 2023-02-16T21:42:01AEDT on overflow (when casting int->float)? [...] (from the very next line: 'with the current set of numeric types, overflow can only happen on u128 as f32 for values greater or equal to f32::MAX + (0.5 ULP)')
//  Ongoing: 2023-02-16T21:45:09AEDT 'casts involving rounding (int->float / f64->f32) are slow on unsupported hardware' ... (just how old/basic are we talking?)
//  Ongoing: 2023-02-16T21:54:47AEDT meaning of term 'true cast'?
//  Ongoing: 2023-02-16T21:56:48AEDT talking about corner cases of <pointer> casts -> (like what?)
//  Ongoing: 2023-02-16T21:57:29AEDT meaning of 'raw slice' (a slice without a length) (or does it refer to the fact these are pointers to slices eg: '*const [u16]')?
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
    //  <(auto-derived 'Clone' implementation equivalent to:)>
    //  <(contention: this is equivalent to the 'derive(Clone)' implementation
    struct MyContainer_ii<T>(Arc<T>);
    impl<T> Clone for MyContainer_ii<T> 
        where T: Clone 
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
    fn clone_mycontainer_ii<T>(foo: &MyContainer_ii<i32>, bar: &MyContainer_ii<T>) {
        //  <(calls?)>
        let foo_cloned = foo.clone();       //  T = MyContainer_ii<i32>
        //  <(calls?)>
        let bar_cloned = bar.clone();       //  T = &MyContainer_ii<T>
    }
    fn clone_mycontainer_iii<T>(foo: &MyContainer_iii<i32>, bar: &MyContainer_iii<T>) {
        //  <(calls?)>
        let foo_cloned = foo.clone();       //  T = MyContainer_iii<i32>
        //  <(calls?)>
        let bar_cloned = bar.clone();       //  T = MyContainer_iii<T>
    }


    //  Dot operator (Method lookup) definitive guide (see below)
    //  LINK: https://rustc-dev-guide.rust-lang.org/method-lookup.html
    //  {{{
    //  }}}
}


#[test]
fn casts()
{
    //  Casts are a superset of coercisons: every coercison can be explicitly invoked via cast

    //  Pointer casts are infallible at runtime. There will be no indication/error if such a cast triggers a corner case.

    //  Lengths are not adjusted when casting raw slices:
    //          *const [u16] as *const [u8]
    //  produces a slice that only includes half the origional memory

    //  Casting is not transitive
    //  Just because 'e as U1 as U2' is valid doesn't necessarily mean 'e as U2' is valid

    //  LINK: https://doc.rust-lang.org/nightly/reference/expressions/operator-expr.html#type-cast-expressions
    //  {{{

    //  TypeCastExpression:
    //          Expression as TypeNoBounds

    //  Valid casts:
    //          int/float               int/float
    //          enum                    int
    //          bool/char               int
    //          u8                      char
    //          *T                      *V (where V: Sized) [1]
    //          *T (where T: Sized)     int
    //          int                     *V (where V: Sized)
    //          &[mut] T                *[mut] T
    //          *[mut] [T; n]           *[mut] T
    //          <Function item>         <Function pointer>
    //          <Function item>         *V (where V: Sized)
    //          <Function item>         int
    //          <Function pointer>      *V (where V: Sized)
    //          <Function pointer>      int
    //          <Closure> [2]           <Function pointer>
    //  [1]: or T/V are compatible unsized types (eg: slices, same-trait-object)
    //  [2]: Closure must not capture any local variables

    //  <(Function item: <?>)>

    //  <(obtaining a mut pointer from a const ref is allowed, using it to modify said value is UB)>

    //  Integer casts:
    //          casting between integers of the same size (eg: i32->u32) is a no-op
    //          casting from larger->smaller int will truncate
    //          casting unsigned smaller->larger int will zero-extend
    //          casting signed smaller->larger int will sign-extend

    //  Float casts:
    //          float->int with decimals will round towards zero
    //          float->int where float > intmax will become intmax
    //          float->int where float < intmin will become intmin
    //          int->float will produce the closet possible float
    //          int->float will produce +/- inf on overflow
    //          f32->f64 is perfect and lossless
    //          f64->f32 will produce the closest possible f32
    //          f64->f32 will produce +/- inf on overflow
    //  (int->float and f64->f32 casts involving rounding are slow on unsupported hardware)

    //  Enum casts are limited to:
    //          unit-only enums
    //          field-less enums (without explicit discriminates)

    //  Primative casts:
    //          bool casts to 0/1
    //          char casts to the value of the code point (then uses a numeric cast if needed)

    //  u8 casts:
    //          to char with corresponding code point

    //  <(Rust's memory model is still under development)>

    //  }}}
}


#[test]
fn transmutes()
{

}

