//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-03T20:01:43AEDT (assume a pointer might be invalidated whenever a reference would be <dropped/invalidated> in the same <situation>?)
//  Ongoing: 2023-02-03T20:33:17AEDT 'format!()' <returns?> a String(?)
//  Ongoing: 2023-02-03T21:06:20AEDT calling 'data.push(4)' (for Vec data) is described by article as taking a mutable reference to 'data'? [...] (the definition of 'push' looks like: 'push(&mut self, elem: T)'(?)) [...] (compiler error says "mutable borrow occurs here" about 'data.push(4)')
//  Ongoing: 2023-02-03T21:33:53AEDT "Multiple borrows can be tied to the same local variable" - example given is assigning different references to same variable (compares this to multiple borrows assigned to the same variable in a loop)
//  Ongoing: 2023-02-03T21:35:21AEDT "Multiple borrows can be tied to the same local variable" (how example changes if we redeclare 'let x = ' on the second borrow?)
//  Ongoing: 2023-02-03T21:41:06AEDT (empty struct?) (declared as 'struct Foo;' vs 'struct Foo {};'?)
//  Ongoing: 2023-02-03T21:56:07AEDT (clarify) "this will eventually get fixed" refers to 'get_default' "Improperly reduced borrows" example and not 'Foo' "Limits of lifetime" example(?)
//  Ongoing: 2023-02-03T22:12:33AEDT "limits_of_lifetimes", implementation of 'mutate_and_share' / 'share' which works for example 'Foo'?
//  Ongoing: 2023-02-03T22:14:26AEDT "limits_of_lifetimes", *when* (and if) we can expect 'get_default()' example to (as contented) "eventually be fixed"?
//  Ongoing: 2023-02-03T23:09:05AEDT meaning of "formal arguments"?
//  Ongoing: 2023-02-03T23:16:30AEDT where lifetimes are called for in function signatures ((all?) input/output references?)
//  Ongoing: 2023-02-03T23:22:18AEDT "lifetime_elision", 'Equivalent' examples with function bodies?
//  Ongoing: 2023-02-07T22:32:46AEDT meaning of "&'a &'a T" -> (reference to a reference?)
//  Ongoing: 2023-02-08T00:47:06AEDT higher_rank_trait_bounds -> (reason/need for HRTBs (in example/generally)?)
//  Ongoing: 2023-02-08T00:49:57AEDT 'higher_rank_trait_bounds' -> placing the trait bounds in impl block (and not struct declaration block?)
//  Ongoing: 2023-02-08T00:55:09AEDT the real unstable Fn trait(?)
//  Ongoing: 2023-02-15T14:28:12AEDT article uses 'fn love(pet: Animal)' (incorrect) instead of 'fn love(pet: &dyn Animal)' (correct) ... ((it's like that because) this example was never meant to be run?)
//  Ongoing: 2023-02-15T14:51:46AEDT what does it even mean to say one lifetime is a subtype of another?
//  Ongoing: 2023-02-15T15:36:08AEDT subtyping and variance, an explanation to actually explain why example is invalid with / valid without 'mut' reference
//  Ongoing: 2023-02-15T15:42:28AEDT 'mr_snuggles' subtyping/variance example (should it use 'Box<MyCat>' or 'Box<Cat>'?)
//  Ongoing: 2023-02-15T15:50:50AEDT how much subtyping/covariance stuff caries over directly from C++?
//  Ongoing: 2023-02-15T15:55:45AEDT (review subtyping/covariance item after) item on inheritance (specifically) in Rust
//  Ongoing: 2023-02-15T16:22:19AEDT another example of the source material being outdated -> 'may_dangle' is/has being killed(?)
//  Ongoing: 2023-02-15T16:52:00AEDT assert_eq requires 'Debug' to be implemented?
//  Ongoing: 2023-02-15T16:52:53AEDT for each <unclear/incomplete> item - what do we actually need to know?
//  }}}
use std::collections::HashMap;
use std::hash::Hash;

//  Continue: 2023-02-15T17:08:50AEDT improved explanations / meaningful examples

#[test]
fn references()
{
    //  Shared reference:   &T
    //  Mutable reference:  &mut T
    
    //  A reference cannot outlive its referent
    //  A mutable reference cannot be aliased
    
    //  <(Rust doesn't have a defined aliasing model?)>
}


#[test]
fn aliasing()
{
    //  Variables / pointers alias if they refer to overlapping regions of memory
    
    //  Alias analysis is key to several useful optimisations:
    //          Keeping values in registers by proving no pointer accesses the value's memory
    //          Eliminating reads by proving some memory hasn't been written to since we last read it
    //          Eliminating writes by proving some memory is never read before the next write to it
    //          Moving or reordering reads/writes by proving they don't depend on each other

    //  Since mutable references cannot be aliased, 'input' and 'output' cannot refer to the same value
    fn compute(input: &u32, output: &mut u32) {
        if *input > 10 { *output = 1; }
        if *input > 5 { *output *= 2; }
    }
    //  This function can therefore be optimised into:
    fn compute_optimised(input: &u32, output: &mut u32) {
        let mut temp = *output;
        if *input > 10 { temp = 1; }
        if *input > 5 { temp *= 2; }
        *output = temp;
    }

    //  <(A full aliasing model for Rust make take into consideration things like function calls, raw pointers, and UnsafeCell)>
}

#[test]
fn lifetimes()
{
    //  Lifetimes are named regions of code that a reference must be valid for
    //  Types which contain references may be tagged with lifetimes to prevent them being invalidated

    //  Lifetimes are denoted with an apostrophe:
    //          'a
    //          'static

    //  Lifetimes <often> need to be explicitly stated when crossing function boundaries

    //  Each 'let' statement implicitly introduces a scope
    //  The borrow checker always tries to minimize lifetimes

    //  Older versions of Rust kept references alive until end of the containing scope
    
    //  We use pseudo-syntax 'a: { } to denote scope lifetimes


    //  The expression
    let x = 0;
    let y = &x;
    let z = &y;
    //  de-sugars into
    //  'a: {
    //          let x: i32 = 0;
    //          'b: {
    //                  let y: &'b i32 = &'b x;
    //                  'c: {
    //                          let z: &'c &'b i32 = &'c y;
    //                      }
    //              }
    //      }


    //  Example: Returning a reference that outlives its referent is an error:
    //  fn as_str(data: &u32) -> &str {
    //      let s = format!("{}", data);
    //      &s
    //  }
    //  Since we cannot convert '&u32' to '&str', the correct behaviour is to return 'String' by-value
    fn as_str(data: &u32) -> String {
        format!("{}", data)
    }


    //  <(Calling a method that takes '&mut self' as argument counts as taking a mutable reference to that variable)>
    let mut data = vec![1,2,3];
    let x = &data[0];
    data.push(4);           //  'x' cannot live past here
//  format!("{}", x);       //  invalid


    //  References lives from when they are created until they are last used
    let mut data = vec![1,2,3];
    let x = &data[0];
    format!("{}", x);       //  'x' dropped here
    data.push(4);


    //  However, if the value has a dtor, the last use of that variable is when it is called, which is when the value goes out of scope
    struct RefDtor<'a>(&'a i32);
    impl Drop for RefDtor<'_> { fn drop(&mut self) {} }
    {
        let mut data = vec![1,2,3];
        let x = RefDtor(&data[0]);
        format!("{:?}", x.0);
//      data.push(4);       //  invalid
    }                       //  'x' dropped here


    //  There can be multiple possible last uses of a reference, eg: branches of an if-statement
    let flag = true;
    let mut data = vec![1,2,3];
    let x = &data[0];
    if flag {
        format!("{}", x);   //  'x' either dropped here
        data.push(4);
    } else {                //  or here
        data.push(5);
    }


    //  Multiple borrows can be tied to the same local variable
    let mut data = vec![1,2,3];
    let mut x = &data[0];
    format!("{}", x);       //  'x' first borrow dropped here
    data.push(4);
    x = &data[3];
    format!("{}", x);       //  'x' second borrow dropped here


    //  contention: the Rust borrow checker isn't perfect but one should expect it to improved over time
}


#[test]
fn limits_of_lifetimes()
{
    #[derive(Debug)]
    struct Foo;         //  <(different from 'struct Foo {};'(?))>

    //  The lifetimes deduced by Rust are too large for the following example:
    //  contention: "This program is clearly correct according to the reference semantics we actually care about, but the lifetime system is too coarse-grained to handle that"
    impl Foo {
        fn mutate_and_share(&mut self) -> &Self { &*self }
//      fn mutate_and_share<'a>(&'a mut self) -> &'a Self { &*self }
        fn share(&self) {}
//      fn share<'a>(&'a self) {}
    }
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    //foo.share();          //  invalid
    format!("{:?}", loan);


    //  Improperly reduced borrows:
    //  contention: this will eventually get fixed
    //  Rust cannot infer that first mutable borrow 'map.get_mut()' is dropped before second mutable borrow 'map.insert()'
//  fn get_default<'m, K, V>(map: &'m mut HashMap<K,V>, key: K) -> &'m mut V
//      where K: Clone + Eq + Hash, 
//            V: Default 
//  {
//      match map.get_mut(&key) {
//          Some(value) => value,
//          None => { 
//              map.insert(key.clone(), V::default());
//              map.get_mut(&key).unwrap()
//          }
//      }
//  }
    //  Alternative: 
    fn get_default<'m, K, V>(map: &'m mut HashMap<K,V>, key: K) -> &'m mut V
        where K: Clone + Eq + Hash, 
              V: Default 
    {
        if map.get(&key).is_some() {
            map.get_mut(&key).unwrap()
        } else {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }
    }
}

#[test]
fn lifetime_elision()
{
    //  A lifetime position is anywhere lifetimes can be written in a type
    //          &'a T
    //          &'a mut T
    //          T<'a>

    //  Lifetime positions can be either "input" or "output":
    //
    //  For function definitions/types, and traits Fn/FnMut/FnOnce, input refers to the types of the formal arguments, and output refers to result types.
    //  (note that input positions of a fn method definition do not include the lifetime traits that occur in the method's impl header (or trait header))
    //  
    //  For impl headers, all types are input


    //  Elison rules:
    //      - Each elided lifetime in input position becomes a distinct lifetime parameter
    //      - If there is only one input lifetime, it is assigned to all elided output lifetimes
    //      - If 'self' is an input, then its lifetime is assigned to all elided output lifetimes
    //      - Otherwise it is an error to elide an output lifetime


    //  Equivalent:
//  fn print(s: &str);
//  fn print<'a>(s: &'a str);
//
//  fn debug(lvl: usize, s: &str);
//  fn debug<'a>(lvl: usize, s: &'a str);
//
//  fn substr(s: &str, until: usize) -> &str;
//  fn substr<'a>(s: &'a str, until: usize) -> &'a str;
//  
//  fn get_mut(&mut self) -> &mut T;
//  fn get_mut<'a>(&'a mut self) -> &'a mut T;
//
//  fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;
//  fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;
//
//  fn new(buf: &mut [u8]) -> BufWriter;
//  fn new(buf: &mut [u8]) -> BufWriter<'_>;            //  <(2018 rust idioms)>
//  fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;

    //  Invalid:
//  fn get_str() -> &str;
//  fn frob(s: &str, t: &str) -> &str;
}


#[test]
fn unbounded_lifetimes()
{
    //  Unsafe code often produces references/lifetimes out of thin air
    //  Such lifetimes are said to be unbounded. It becomes as big as context demands.

    //  An unbounded lifetime is in many ways equivalent to 'static 
    //  (however it can be used in places where 'static would fail to typecheck)

    //  One should endeavour to bound an unbound lifetime as quickly as possibly 
    //  (especially across function boundaries)

    //  Given a function, any output lifetimes that don't derive from inputs are unbounded
    fn get_str<'a>() -> &'a str { &"abc" }
    //  Will produce '&str' with an unbounded lifetime

    //  Any output lifetime that is elided *must* be bound my an input lifetime

    //  Within a function, bounding lifetimes are error-prone
    //  (the easiest way to bound a lifetime is to return it from a function)
}


//  <(Incomplete explanation)>
#[test]
fn higher_rank_trait_bounds()
{
    //  Consider:
//  struct Closure<F> {
//      data: (u8, u16),
//      func: F,
//  }
//  impl<F> Closure<F> 
//      where F: Fn(&(u8,u16)) -> &u8,
//  {
//      fn call(&self) -> &u8 {
//          (self.func)(&self.data)
//      }
//  }
//  fn do_it(data: &(u8,u16)) -> &u8 { &data.0 }
//  let clo = Closure { data: (0,1), func: do_it };
//  let y = clo.call();
    //  <(How do we express the lifetimes on F's trait bound)>
    //  <(We need to provide some lifetime ... but the lifetime we care about can't be named until we enter the body of 'call()' ('call' works with any lifetime '&self' happens to have at that point))>


    //  where F: Fn(&(u8,u16)) -> &u8
    //  becomes
    //  where for<'a> F: Fn(&'a (u8,u16)) -> &'a u8
    //  or
    //  where F: for<'a> Fn(&'a (u8,u16)) -> &'a u8

    //  <('Fn(a,b,c)->d' is sugar for the real unstable 'Fn' trait)>


    //  Higher rank trait bounds: 
    //  <(declaring lifetime parameters for function parameters)>
    //          for<'a> 
    //  meaning: for all choices of 'a
    //  <(produces an infinite list of trait bounds that must be satisfied)>
    //  (not commonly used outside 'Fn' traits)


    //  Filling in the lifetime parameters:
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }
    impl<F> Closure<F> 
        where for<'a> F: Fn(&'a (u8,u16)) -> &'a u8,
    {
        fn call(&self) -> &u8 {
        //fn call<'a>(&'a self) -> &'a u8 {
            (self.func)(&self.data)
        }
    }
    fn do_it<'b>(data: &'b (u8,u16)) -> &'b u8 { &data.0 }
    let clo = Closure { data: (0,1), func: do_it };
    let y = clo.call();
}


//  <(Incomplete explanation)>
#[test]
fn subtyping_and_variance()
{
    //  Subtyping is a relationship between types allowing a statically typed language to be more permissive

    //  <(subtyping is just another way to say 'is-a' inheritance?)>

    //  <(To keep things simple, we start by considering an extension to the Rust language?)>
    //  {{{

    //  Animal is a supertype of Cat/Dog
    //  Cat/Dog are subtypes of Animal
    trait Animal {
        fn snuggle(&self);
        fn eat(&mut self);
    }
    trait Cat: Animal {
        fn meow(&self);
    }
    trait Dog: Animal {
        fn bark(&self);
    }

    struct MyAnimal {}
    impl Animal for MyAnimal {
        fn snuggle(&self) {}
        fn eat(&mut self) {}
    }
    struct MyCat {}
    impl Animal for MyCat {
        fn snuggle(&self) {}
        fn eat(&mut self) {}
    }
    impl Cat for MyCat {
        fn meow(&self) {}
    }
    struct MyDog {}
    impl Animal for MyDog {
        fn snuggle(&self) {}
        fn eat(&mut self) {}
    }
    impl Dog for MyDog {
        fn bark(&self) {}
    }

    //  <(intuition: anywhere supertype T is expected, a subtype <should/will> also work)>

    //  <(problem: replacing a Cat instance with a Dog instance given a Cat as a mutable reference)>
//  fn evil_feeder(pet: &mut Animal) {
//      let spike = Dog;
//      *pet = spike;
//  }

    //  'static, the forever lifetime, is a subtype of every lifetime

    //  }}}

    //  Subtyping and lifetimes: 'a is a subtype of 'b if 'b outlives 'a

    //  A type ctor's variance is how the subtyping of its inputs affect the subtyping of its outputs
    //          F<T> is covariant if F<Sub> is a subtype of F<Super>
    //          F<T> is contravariant if F<Super> is a subtype of F<Sub>
    //          Otherwise F<T> is invariant

    //  If F<T,U> has multiple type parameters, we talk about variance WRT each parameter separately
    //  <(covariance exists WRT type and lifetime parameters?)>

    //                      'a                  T                   U
    //      &'a T           covariant           covariant           -
    //      &'a mut T       covariant           invariant           -
    //      Box<T>          -                   covariant           -
    //      Vec<T>          -                   covariant           -
    //      UnsafeCell<T>   -                   invariant           -
    //      Cell<T>         -                   invariant           -
    //      fn(T) -> U      -                   contravariant       covariant
    //      *const T        -                   covariant           -
    //      *mut T          -                   invariant           -

    //  Result: even if Cat is a Subtype of Animal, &mut Cat is not a subtype of &mut Animal
    //  (the type checker will prevent us passing '&mut Cat' where '&mut Animal' is expected)
    //  (and the same with raw pointers)

    //  <(Even though references are covariant over their lifetimes, they 'inherit' invariance whenver they're put into a context that could do something bad with that)>


    //  Example: type of 'input' is deduced to be &'static str, so 'val' must also be static
    //  <(we inherit invariance as soon as we put our reference inside '&mut T')>
    fn f1<T>(input: &mut T, val: T) { }
    let mut s1: &'static str = "meow";
    let spike = String::from("bark");
//  f1(&mut s1, &spike);    //  invalid
    //
    //  <()>
    fn f2<T>(input: &T, val: T) { }
    let s1: &'static str = "meow";
    let spike = String::from("bark");
    f2(&s1, &spike);


    //  If we assign a Cat object to 'Box<Animal>', we have destroyed the only information that the object was a Cat
    let mr_snuggles: Box<MyCat> = Box::new( MyCat {} );
    let spike: Box<MyDog> = Box::new( MyDog {} );
    let mut pet: Box<dyn Animal>;
    pet = mr_snuggles;
    //  <('mr_snuggles' is no longer a Cat)>
    pet = spike;
    //  <('spike' is no longer a Dog)>


    //  fn(T) -> U is covariant over U
    //  If a function must produce an Animal, then a function producing a Cat produces an Animal

    //  fn(T) -> U is contravariant over T
    //  If a function accepts an Animal, then it also accepts a Cat


    //  Custom types: <(informally speaking)>
    //  If all uses of field type A are covariant, the custom type is covariant over A
    //  If all uses of field type A are contravariant, the custom type is contravariant over A
    //  Otherwise the custom tyhpe is invariant over A

    use std::cell::Cell;
    struct MyType<'a, 'b, A: 'a, B: 'b, C, D, E, F, G, H, In, Out, Mixed> {
        a: &'a A,                   //  covariant over 'a / A
        b: &'b mut B,               //  covarient over 'b, invariant over B
        c: *const C,                //  covariant over C
        d: *mut D,                  //  invariant over D
        e: E,                       //  covariant over E
        f: Vec<F>,                  //  covariant over F
        g: Cell<G>,                 //  covariant over G
        h1: H,                      
        h2: Cell<H>,                //  invariant over H
        i: fn(In) -> Out,           //  contravariant over In, covariant over Out
        k1: fn(Mixed) -> usize,
        k2: Mixed,                  //  invariant over Mixed
    }

    //  Subtyping and variance
    //  LINK: https://doc.rust-lang.org/nightly/reference/subtyping.html#variance
    //  {{{
    //  (how much here is actually new/different?)
    //  }}}
}


#[test]
fn checking_of_drop()
{
    //  Variables are dropped in reverse order of their definition
    //  Fields of structs/tuples are dropped in order of their definition

    //  The left vector is dropped first
    //  However, from the perspective of the borrow checker, the right vector *does not* outlive it
    let tuple: (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    //  <(This allows us to accidentally create dangling pointers)>


    //  For a generic type to soundly implement Drop, its generic arguments must strictly outlive it
    //  (this is only a problem for generic types, since non-generic types cannot harbour lifetimes other than 'static)
    //  The borrow checker does not examine the Drop method - if it exists, all borrowed data in the object is required to outlive it (whether the Drop method accesses it or not)
    //  contention: this requirement may be relaxed in the future
    struct Inspector<'a>(&'a u8);
    impl<'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("I was only {} days from retirement", self.0);
        }
    }
    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }
    let mut world = World { inspector: None, days: Box::new(1), };
//  world.inspector = Some(Inspector(&world.days));


    //  currently there is the nightly trait 'may_dangle' (requires unsafe Drop implementation), which asserts to the compiler the variable thus marked is not used beyond its life
    //  (compiler error: 'may_dangle' will be removed in the future)
    //  {{{
//  {
//  #![feature(dropck_eyepatch)]
//  struct Inspector<'a>(&'a u8, &'static str);
//  unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
//      fn drop(&mut self) {
//          println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
//      }
//  }
//  struct World<'a> {
//      days: Box<u8>,
//      inspector: Option<Inspector<'a>>,
//  }
//  let mut world = World {
//      inspector: None,
//      days: Box::new(1),
//  };
//  world.inspector = Some(Inspector(&world.days, "gadget"));
//  }
    //  }}}

    //  Use the 'ManuallyDrop' wrapper when the order struct variables are dropped matters
}


//  <(example usecase for PhantomData)>
#[test]
fn phantom_data()
{
    //  Unsafe code allows situations where types/lifetimes are logically associated with a struct, but not actually part of a field

    //  Lifetime/type parameters not used in a struct body are unbounded (and not allowed)
    //  'std::marker::PhantomData' is provided to allow empty fields for holding such parameters
    struct Eg_Iter<'a, T: 'a> {
        ptr: *const T,
        end: *const T,
        _marker: std::marker::PhantomData<&'a T>,
    }

    //                                  'a              T
    //      PhantomData<T>              -               covariant (w/ drop check)
    //      PhantomData<&'a T>          covariant       covariant
    //      PhantomData<&'a mut T>      covariant       invariant
    //      PhantomData<*const T>       -               covariant
    //      PhantomData<*mut T>         -               invariant
    //      PhantomData<fn(T)>          -               contravariant
    //      PhantomData<fn()->T>        -               covariant
    //      PhantomData<fn(T)->T>       -               invariant
    //      PhantomData<Cell<&'a ()>>   invariant       -
}


#[test]
fn splitting_borrows()
{
    //  The borrow checker allows disjoin fields of a struct to be borrowed simultaneously:
    #[derive(PartialEq, Debug)]
    struct Foo { 
        a: i32, 
        b: i32, 
        c: i32, 
    }
    let mut x = Foo { a: 0, b: 0, c: 0, };
    let a = &mut x.a;
    let b = &mut x.b;
    let c = &x.c;
    *b += 1;
    let c2 = &x.c;
    *a += 10;
    assert_eq!(x, Foo { a: 10, b: 1, c: 0, });

    //  However it doesn't allow the same for array/slice elements:
    let mut x = [1, 2, 3];
//  let a = &mut x[0];
//  let b = &mut x[1];
//  println!("{}, {}", a, b);

    //  For this, Rust provides 'split_at_mut()'
    //  (the implementation of which is unsafe)
//  pub fn Eg_split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
//      let len = self.len();
//      let ptr = self.as_mut_ptr();
//      unsafe {
//          assert!(mid <= len);
//          ( from_raw_parts_mut(ptr, mid),
//            from_raw_parts_mut(ptr.add(mid), len-mid)
//            )
//      }
//  }


    trait Eg_Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    //  'Self::Item' has no connection to 'self'
    //  (allowing us to call 'next()' repeatedly while holding all the results concurrently)
    //  (this works for mutable iterators because an iterator can only return each item once)

    //  Implemenations of 'IterMut' do not require unsafe for many containers
}

