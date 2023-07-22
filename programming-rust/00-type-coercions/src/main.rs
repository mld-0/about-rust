//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-21T01:38:58AEDT 'let y = &mut x' vs 'let mut y = &mut x' ((just means we can reassign y to be another borrowed reference) (which we could do anyway by <re-declaring/shaddowing> y?))
//  Ongoing: 2022-10-21T02:13:05AEDT web example (differs in the syntax rules of Rust) 'what-can-coerce-and-where-in-rust' has pub functions in a pub trait -> Ycm forbids us declaring the function public (with an error that it is already there implicitly) [...] (web article date: 2021-07-06)
//  Ongoing: 2022-10-21T02:50:05AEDT ConstHandle/ConstHandle (why declare two seperate structs instead of an Enum?)
//  Ongoing: 2022-10-21T02:51:50AEDT (an example of implict dereferencing: 'println!("x=({})", &x)'?)
//  }}}

//  LINK: https://www.possiblerust.com/guide/what-can-coerce-and-where-in-rust

//  From/Into provide infallible conversions
//  (TryFrom/TryInto are the fallible equivalents)

//  Other conversions:
//      AsRef
//      AsMut
//      Borrow
//      ToOwned
//      <()>

//      std::mem::transmute                 Conversion from any one type to any other. It is unsafe.
//  <(*unsafe - it is up to the user to ensure correct implementation)>
//  <(safe transmute is in ongoing development)>

//  these are all explicit conversions - to perform the conversion, one must call the relevant function
//  (casts are also explicit conversions)


//  Type coercions are implicit type conversions


//  LINK: https://doc.rust-lang.org/reference/type-coercions.html
//  {{{
//  }}}

fn example_reference_downgrade_coercions()
{
    //  Where &mut T is coerced into &T
    //  (always safe)

    //  <(Example)>
    struct RefHolder<'a> {
        x: &'a i64,
    }
    impl<'a> RefHolder<'a> {
        fn new(x: &'a i64) -> RefHolder<'a> {
            RefHolder { x }
        }
    }
    fn print_num_byref(y: &i64) {
        println!("y=({})", y);
    }

    let mut x = 10;
    let y = &mut x;

    //  z is immutable, hence z.x is immutable
    let z = RefHolder::new(y);

    //  y is downgraded from '&mut i64' -> '&i64'
    print_num_byref(y);

    //  Since mutable reference 'y' to 'x' still exists, we cannot borrow another reference from 'x'
    //let z = RefHolder::new(&x);               //  error
    //let z = RefHolder::new(&mut x);           //  error
    //print_num_byref(y);

    //  Cannot borrow immutable reference while shared reference still exists
    //  (even though '&mut x' will be downgraded to '&x' so actually no immutable reference is being borrowed)
    //let y = &x;
    //let z = RefHolder::new(&mut x);           //  error
    //print_num_byref(y);

    let y = &x;
    let z = RefHolder::new(&x);
    print_num_byref(y);


    //  <(Reference downgrades are often not what you want)>
    //  <(specific example: 9 downgrading mut refs to shared refs is safe)>
    //  LINK: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#9-downgrading-mut-refs-to-shared-refs-is-safe
    //  {{{
    //  <>

    //  key takeaways:
    //  try not to re-borrow mut refs as shared refs, or you're gonna have a bad time
    //  re-borrowing a mut ref doesn't end its lifetime, even if the ref is dropped
    //  }}}

    println!("example_reference_downgrade_coercions, DONE");
}


fn example_deref_coercisions()
{
    //  Deref coercisions arise from implementing Deref/DerefMut
    use std::ops::{Deref,DerefMut};

    //  Definitions:
    pub trait ExampleDeref {
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }
    pub trait ExampleDerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }
    //  (note that both must have the same 'Target' type)

    //  Box<T> implements Deref<Target=T>

    //  Smart pointers generally provide their methods as associated functions instead of methods
    //  eg: Box::leak is defined as:
    //      fn leak<'a>(b: Box<T,A>) -> &'a mut T where A: 'a
    //  and is called as:
    //      Box::leak(boxed_type)
    //  (not):
    //      boxed_type.leak()

    //  Implicit dereferencing occurs when:
    //      using the '.' operator
    //      <>

    //  <(Implicit dereferencing and ambiguities)>

    println!("example_deref_coercisions, DONE");
}


fn example_raw_pointer_coercions()
{
    //  Raw pointer '*mut T' may be coerced to '*const T'
    //  <>

    //  <(converting pointers and safety)>

    //  Rust permits explicit casting of '*const T' to '*mut T'
    //  <(can we guess - don't cast away const (unless you really have to)?)>
    //  <(Casting away const of pointers can be necessary when working with C-APIs)>
    //  <(casting away const of a pointer is undefined if the origional underlying pointer is not mutable)>

    #[derive(Debug)]
    struct PtrHandle { ptr: *const i32, };

    let mut x = 5;

    //  <(use 'as' to cast reference to pointer)>
    let px = &mut x as *mut i32;

    //  '*mut i32' is coerced to '*const i32'
    let h = PtrHandle { ptr: px };

    println!("px=({:?})", px);
    println!("h=({:?})", h);

    println!("example_raw_pointer_coercions, DONE");
}


fn example_reference_and_raw_pointer_coercions()
{
    //  Coercisions from references to raw pointers 
    //      '&T' -> '*const T'
    //      '&mut T' -> '*mut T'
    //  (this is safe, although dereferencing the resulting pointer is not)
    //  <(not (the other way) raw pointers to references? (can it be done or is it just unsafe))>
    
    //  <(coercions (still) work when generic types are present too)>

    //  'Debug' prints the address of a pointer vs the value of a reference
    #[derive(Debug)]
    struct ConstHandle<T> { ptr: *const T, }
    #[derive(Debug)]
    struct MutHandle<T> { ptr: *mut T, }
    #[derive(Debug)]
    struct ConstRef<'a,T> { ptr: &'a T, }
    #[derive(Debug)]
    struct MutRef<'a,T> { ptr: &'a mut T, }

    let mut x = 5;
    let m = MutHandle { ptr: &mut x, };     //  coerce '&i32' -> '*const i32'
    let c = ConstHandle { ptr: &x, };       //  coerce '&mut i32' -> '*mut i32'
    println!("m=({:?})", m);
    println!("c=({:?})", c);

    //  error, cannot borrow shared reference while mutable reference exists 
    //let m = MutRef { ptr: &mut x, };
    //let c = ConstRef { ptr: &x, };                        //  error
    //println!("m=({:?})", m);
    //println!("c=({:?})", c);
    //  (note that borrowed pointers do not have this problem)

    println!("example_reference_and_raw_pointer_coercions, DONE");
}


fn example_function_pointer_conversions()
{
    println!("example_function_pointer_conversions, DONE");
}

fn example_subtype_coercions()
{
    println!("example_subtype_coercions, DONE");
}

fn example_never_conversions()
{
    println!("example_never_conversions, DONE");
}

fn example_slice_coercions()
{
    println!("example_slice_coercions, DONE");
}

fn example_trait_object_coercions()
{
    println!("example_trait_object_coercions, DONE");
}

fn example_trailing_unsized_coercions()
{
    println!("example_trailing_unsized_coercions, DONE");
}

fn example_least_upper_bound_coercions()
{
    println!("example_least_upper_bound_coercions, DONE");
}

fn example_transitive_coercions()
{
    println!("example_transitive_coercions, DONE");
}

fn example_coercion_sites()
{
    println!("example_coercion_sites, DONE");
}

fn example_coercion_propagating_expressions()
{
    println!("example_coercion_propagating_expressions, DONE");
}

fn example_unsized_coercions_and_coercion_sites()
{
    println!("example_unsized_coercions_and_coercion_sites, DONE");
}

fn main() 
{
    example_reference_downgrade_coercions();
    example_deref_coercisions();
    example_raw_pointer_coercions();
    example_reference_and_raw_pointer_coercions();
    example_function_pointer_conversions();
    example_subtype_coercions();
    example_never_conversions();
    example_trailing_unsized_coercions();
    example_least_upper_bound_coercions();
    example_transitive_coercions();
    example_coercion_sites();
    example_coercion_propagating_expressions();
    example_unsized_coercions_and_coercion_sites();
}

