//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-12-04T21:43:01AEDT rule that says macro must come after 'allow' <statements>
//  Ongoing: 2022-12-04T21:59:07AEDT trait objects are by definition unsized(?)
//  Ongoing: 2022-12-04T22:00:25AEDT (about) trait objects and 'dyn' keyword? [...] (and 'Sized' and trait objects)
//  Ongoing: 2022-12-04T22:28:15AEDT Rust can deduce 'T = &str', making it valid to pass '&str' to 'f(x: T)', but not to 'f(x: &T)' (unless T is optionally sized)
//  Ongoing: 2022-12-04T22:49:47AEDT 'As an alternative to marking the trait Sized, we can mark indervidual methods as Sized' -> (what is the practical difference?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::mem::size_of;
//  macro: get_func_name!()
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

//  Continue: 2022-12-04T22:53:08AEDT about 'dyn' keyword (and its relationship to sizedness)
//  Continue: 2022-12-04T22:56:31AEDT trait object limitations
//  Continue: 2022-12-04T23:11:03AEDT zero sized types, '!' / 'PhantomData' examples

//  LINK: https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md

//  Definitions:
//      sizedness           property of being sized / unsized
//      sized               type with a known size at compile time
//      unsized/DST         dynamically sized type (size not known at compile time)
//      ?sized              type that may or may not be sized

//  Sized types can be passed by value or reference.
//  Unsized types can't be placed on the stack - they may only be passed by reference.

//  Sized is an auto-trait and a marker-trait
//  It is automatically implemented for a type if all its members are Sized


fn example_type_sizes()
{
    assert_eq!(4, size_of::<i32>());
    assert_eq!(8, size_of::<f64>());
    assert_eq!(8, size_of::<(i32, i32)>());
    assert_eq!(0, size_of::<[i32; 0]>());
    assert_eq!(12, size_of::<[i32; 3]>());
    
    //  32/64-bits, depending on system
    const POINTER_WIDTH: usize = size_of::<&()>();
    println!("POINTER_WIDTH=({})", POINTER_WIDTH);

    //  pointers to sized types are 1-width
    assert_eq!(POINTER_WIDTH, size_of::<&i32>());
    assert_eq!(POINTER_WIDTH, size_of::<&mut i32>());
    assert_eq!(POINTER_WIDTH, size_of::<Box<i32>>());
    assert_eq!(POINTER_WIDTH, size_of::<fn(i32) -> i32>());


    //  unsized struct
    struct Eg_Unsized { field: [i32], }

    //  References to unsized types are 2 words, since they must store a length (or pointer to a vtable)
    assert_eq!(2*POINTER_WIDTH, size_of::<&str>());                 //  slice
    assert_eq!(2*POINTER_WIDTH, size_of::<&[i32]>());               //  slice
    assert_eq!(2*POINTER_WIDTH, size_of::<&dyn ToString>());        //  trait object
    assert_eq!(2*POINTER_WIDTH, size_of::<Box<dyn ToString>>());    //  trait object
    assert_eq!(2*POINTER_WIDTH, size_of::<&Eg_Unsized>());          //  custom unsized type

    //  A slice is unsized as it can have any number of elements
    //  <(A trait objects can be implemented by any number of structs or enums and thus can also be of any size at run-time)>

    println!("{}, DONE", get_func_name!());
}


fn example_Sized_trait()
{
    //  'Sized' is an auto trait (automatically implemented for types that only contain other Sized types)
    //  'Sized' is a marker trait (denotes that size is known at compile time)

    //  (unlike other auto traits, 'Sized' cannot be opted out of)
    //  {{{
    //  actually an error (even with 'feature(negative_impls)'?
    //#![feature(negative_impls)]
    //struct Foo;
    //impl !Send for Foo {}       //  opt out of 'Send'
    //impl !Sync for Foo {}       //  opt out of 'Sync'
    //impl !Sized for Foo {}    //  error
    //  }}}

    println!("{}, DONE", get_func_name!());
}


fn example_Sized_in_generics()
{
    //  Generic parameters are auto-bound with 'Sized' by default
    //  Equivalent:
    fn func_1<T>(t: T) {}
    fn func_2<T: Sized>(t: T) {}

    //  Opt out of 'Sized' with:
    //  (cannot pass by-value)
    fn func_3<T: ?Sized>(t: &T) {}
    fn func_4<T: ?Sized>(t: Box<T>) {}

    //  '?Sized' denotes optionally-sized
    //  (this is a relaxed bound, since it relaxes instead of constraining the type parameter)

    //  claim: when accepting a pointer-type, we usually want to make it optionally sized to increase the range of types the function will accept

    //  Example: cannot pass type 'str' unless type is optionally sized
    use std::fmt::Debug;
    fn print_debug<T: Debug + ?Sized>(t: &T) {
        println!("print_debug, t=({:?})", t);
    }
    print_debug("abc");

    println!("{}, DONE", get_func_name!());
}


fn example_derefAndUnsized_coersion()
{
    //  Rust's most common slices are '&str' / '&[T]'
    //  Many other types coerce to these, allowing us to use them to write flexible APIs

    //  deref coercion: T is coerced into U, where 'T: Deref<Target = U>'
    //  unsized coercsion: T is coerced into U, where 'T: Unsize<U>'

    //  Type coercion most commonly occurs during method/function calls

    trait Eg_Trait { 
        fn method(&self) {}
    }
    impl Eg_Trait for str {
        //  Allow '.method()' to be called on: 'str' / 'String'
    }
    impl<T> Eg_Trait for [T] {
        //  Allow '.method()' to be called on: '&[T]' / 'U: Deref<Target=T>' / '[T; N]'
    }

    fn pass_str(s: &str) {}
    fn pass_slice<T>(s: &[T]) {}

    let s1: &str = "abc def hij";
    let s2: String = "klm nop qrs".to_owned();
    pass_str(s1);
    pass_str(&s2);
    s1.method();
    s2.method();

    let slice: &[i32] = &[1];
    let three_arr: [i32; 3] = [1,2,3];
    let v: Vec<i32> = vec![1];
    pass_slice(slice);
    pass_slice(&v);             //  deref-coercsion
    pass_slice(&three_arr);     //  unsized-coersion
    slice.method();
    v.method();                 //  deref-coersion
    three_arr.method();         //  unsized-coersion

    println!("{}, DONE", get_func_name!());
}


fn example_Trait_objects()
{
    //  Traits are optionally sized by default
    //  (We cannot explicitly declare a trait sized)

    //  This prevents us from passing 'self' by-value, limiting the methods we can define
    //  (Note that it possible to define methods in the trait where 'self' is passed by-value, however it will not be possible to implement such a trait)

    //  We can create a Sized trait (and pass 'self' by-value) by explicitly binding the trait with Sized
    trait Eg_Trait_1: Sized {
        fn method(self) {}
    }
    //  (however we will not be able to implement this trait for unsized types)

    //  To maximise types a trait is compatible with, keep the trait optionally sized, and do not pass 'self' by-value

    //  As an alternative to marking the trait Sized, we can mark indervidual methods as Sized
    trait Eg_Trait_2 {
        fn method(self) where Self: Sized {}
    }
    //  (this allows us to implement the trait for Unsized types, provided we never call the Sized methods)


    //impl Trait for dyn Trait {
    //}
    //  <>


    //  Summary:
    //      all traits are '?Sized' by default
    //      'Trait: ?Sized' is required for 'impl Trait for dyn Trait'
    //      'Self: Sized' can be required on a per-method basis
    //      traits bound by Sized can't be made into trait objects

    println!("{}, DONE", get_func_name!());
}


fn example_Trait_object_limitations()
{
    //  <>

    println!("{}, DONE", get_func_name!());
}


fn example_custom_Unsized_types()
{
    //  A struct that contains an unsized field is unsized
    //  An unsized struct can only have one field, and it must be the last in the struct

    struct Eg_Unsized { field: [i32], }
    //let ms: &Eg_Unsized = &Eg_Unsized { field: [1, 2, 3] };       //  error, size cannot be known at compile-time


    //  <(To instantiate such a type, we must make a signed version of it, then coerce it into an unsigned type. This requries
    struct Eg_MaybeSized<T: ?Sized> { maybe_sized: T, }
    let ms: &Eg_MaybeSized<[i32]> = &Eg_MaybeSized { maybe_sized: [1, 2, 3] };


    //  claim: user defined unsized types are of limited use (right now?)

    println!("{}, DONE", get_func_name!());
}


fn example_zero_sized_type()
{
    //  The most common ZST is the unit type '()'
    //  (all empty blocks evaluate to the unit type)

    //  All instance of a ZST are equal to each other
    //  The compiler is able to optimise away interactions with ZSTs

    //  HashSet<Key> is implemented by HashMap<Key,()>

    //  User defined zero size structs:
    struct Foo;
    //  (We can implement traits on custom zero size structs, but not on '()')


    //  The second most common ZST is the never type '!'
    //  It can be coerced into any other type
    //  It is not possible to create instances of it

    //  Example:
    //  <>

    //  Result<!, Error> indicates that success is impossible
    //  Result<Success, !> indicates that failure is impossible


    //  User defined pseudo-never type
    enum Void {}
    //  For which:
    //  Result<Void, Error> indicates that success is impossible
    //  Result<Success, Void> indicates that failure is impossible


    //  The third most common ZST is 'PhantomData'
    //  (zero sized object that acts like it owns a thing of a given type)
    use std::marker::PhantomData;

    //  Example:
    //  <>

    println!("{}, DONE", get_func_name!());
}


//  Conclusion:
//  only instances of sized types can be placed on the stack, i.e. can be passed around by value
//  instances of unsized types can't be placed on the stack and must be passed around by reference
//  pointers to unsized types are double-width because aside from pointing to data they need to do an extra bit of bookkeeping to also keep track of the data's length or point to a vtable
//  Sized is an "auto" marker trait
//  all generic type parameters are auto-bound with Sized by default
//  if we have a generic function which takes an argument of some T behind a pointer, e.g. &T, Box<T>, Rc<T>, et cetera, then we almost always want to opt-out of the default Sized bound with T: ?Sized
//  leveraging slices and Rust's auto type coercions allows us to write flexible APIs
//  all traits are ?Sized by default
//  Trait: ?Sized is required for impl Trait for dyn Trait
//  we can require Self: Sized on a per-method basis
//  traits bound by Sized can't be made into trait objects
//  Rust doesn't support pointers wider than 2 widths so
//  we can't cast unsized types to trait objects
//  we can't have multi-trait objects, but we can work around this by coalescing multiple traits into a single trait
//  user-defined unsized types are a half-baked feature right now and their limitations outweigh any benefits
//  all instances of a ZST are equal to each other
//  Rust compiler knows to optimize away interactions with ZSTs
//  ! can be coerced into any other type
//  it's not possible to create instances of ! which we can use to mark certain states as impossible at a type level
//  PhantomData is a zero-sized marker struct which can be used to "mark" a containing struct as having certain properties


fn main() 
{
    example_type_sizes();
    example_Sized_trait();
    example_Sized_in_generics();
    example_Trait_objects();
    example_Trait_object_limitations();
    example_custom_Unsized_types();
}

