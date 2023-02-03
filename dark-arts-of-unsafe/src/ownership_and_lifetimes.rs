//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
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
//  }}}
use std::collections::HashMap;
use std::hash::Hash;

//  Continue: 2023-02-03T22:21:47AEDT complete chapter

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

