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
//  Ongoing: 2023-02-16T22:41:07AEDT (alternatives to transmute, first pointer -> usize example is using a reference (and calling it a pointer?))
//  }}}
use std::sync::Arc;

//  Continue: 2023-02-15T23:54:47AEDT 'value.clone()' behaviour with/without 'T: Cloned' trait bound
//  Continue: 2023-02-16T00:06:50AEDT quick-and-dirty "when does type coercion take place" guide

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
    //  Casts are a superset of coercions: every coercion can be explicitly invoked via cast

    //  Pointer casts are infallible at runtime. There will be no indication/error if such a cast triggers a corner case.

    //  Lengths are not adjusted when casting raw slices:
    //          *const [u16] as *const [u8]
    //  produces a slice that only includes half the original memory

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
    //  Transmutes can cause UB in a variety of ways and should be avoided where possible

    //  'mem::transmute<T,U>' 
    //  Takes a value of type T and reinterprets it to have type U
    //  (both types must (be verified to) have the same size)

    //  Transmute has an overloaded return type. It may return a suprising type if the return type is not specified.

    //  Transmuting a shared reference to a mutable reference is UB

    //  Transmuting to a reference without an explicitly provided lifetime produces an unbounded lifetime
    //  (see 'ownership_and_lifetimes')

    //  When transmuting between compound types, verify they are laid out the same way
    //  (unless 'repr(C)' / 'repr(transparent)' are used, declaring the fields of both structs in the same order is not sufficent)
    //  (don't even assume 'Vec<i32>' / 'Vec<u32>' have the same layout)

    //  Uses:
    //          Turning a pointer into a function pointer [1]
    //          Extending a lifetime
    //          Shortening an invariant lifetime
    //  [1]: only valid on machines where function/data pointers have the same size

    //  Example: extend a lifetime
    struct R<'a>(&'a i32);
    unsafe fn extend_lifetime<'a>(r: R<'a>) -> R<'static> {
        std::mem::transmute::<R<'a>, R<'static>>(r)
    }

    //  Example: shorten an invariant lifetime
    unsafe fn shorten_invariant_lifetime<'a,'b>(r: &'a mut R<'static>) -> &'a mut R<'b> {
        std::mem::transmute::<&'a mut R<'static>, &'a mut R<'b>>(r)
    }

    //  Alternative: raw bytes into primative
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    let num = unsafe { std::mem::transmute::<[u8;4], u32>(raw_bytes) };
    assert_eq!(num, 0x12345678);
    //  or
    //          from_ne_bytes()         (native endian)
    //          from_le_bytes()         (little endian)
    //          from_be_bytes()         (big endian)
    let num = u32::from_ne_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    //  Alternatives: pointer conversions
    //  {{{

    //  Turning a <~~pointer~~ reference(?)> into a usize
    let ptr = &0;
    let ptr_num_transmute = unsafe {
        std::mem::transmute::<&i32, usize>(ptr)
    };   
    println!("ptr_num_transmute=({})", ptr_num_transmute);

    //  Turning a *mut T into an &mut T
    let ptr: *mut i32 = &mut 0;
    let ptr_num = unsafe { std::mem::transmute::<*mut i32, &mut i32>(ptr) };
    println!("ptr_num=({})", ptr_num);
    let ptr_num = unsafe { &mut *ptr };     //  Use a reborrow instead
    println!("ptr_num=({})", ptr_num);

    //  Turning an &mut T into an &mut U
    let ptr = &mut 0;
    let val = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };
    let val = unsafe { &mut *(ptr as *mut i32 as *mut u32) };     //  Now, put together `as` and reborrowing - note the chaining of `as` `as` is not transitive

    //  <(using transmute to turn a pointer* to a usize is UB in const contexts)>
    //  <(*they say 'pointer' do they <again> mean 'reference'?)>

    //  }}}

    //  Alterative: str into raw bytes
    let s = "Rust";
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>(s) };
    assert_eq!(slice, &[82, 117, 115, 116]);
    let slice = s.as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    //  Alternative: Vec<&T> into Vec<Option<&T>>
    //  <(includes example that 'could cause UB' (but is ok as per Miri?))>
    //  {{{
    let store = [0, 1, 2, 3];
    let v_orig = store.iter().collect::<Vec<&i32>>();

    // clone the vector as we will reuse them later
    let v_clone = v_orig.clone();

    // Using transmute: this relies on the unspecified data layout of `Vec`, which is a
    // bad idea and could cause Undefined Behavior.
    // However, it is no-copy.
    let v_transmuted = unsafe {
        std::mem::transmute::<Vec<&i32>, Vec<Option<&i32>>>(v_clone)
    };

    let v_clone = v_orig.clone();

    // This is the suggested, safe way.
    // It does copy the entire vector, though, into a new array.
    let v_collected = v_clone.into_iter()
                             .map(Some)
                             .collect::<Vec<Option<&i32>>>();

    let v_clone = v_orig.clone();

    // This is the proper no-copy, unsafe way of "transmuting" a `Vec`, without relying on the
    // data layout. Instead of literally calling `transmute`, we perform a pointer cast, but
    // in terms of converting the original inner type (`&i32`) to the new one (`Option<&i32>`),
    // this has all the same caveats. Besides the information provided above, also consult the
    // [`from_raw_parts`] documentation.
    let v_from_raw = unsafe {
        // Ensure the original vector is not dropped.
        let mut v_clone = std::mem::ManuallyDrop::new(v_clone);
        Vec::from_raw_parts(v_clone.as_mut_ptr() as *mut Option<&i32>,
                            v_clone.len(),
                            v_clone.capacity())
    };  
    //  }}}

    //  Example: implement 'split_at_mut'
    //  {{{

    use std::{slice, mem};

    // There are multiple ways to do this, and there are multiple problems
    // with the following (transmute) way.
    fn split_at_mut_transmute<T>(slice: &mut [T], mid: usize)
                                 -> (&mut [T], &mut [T]) {
        let len = slice.len();
        assert!(mid <= len);
        unsafe {
            let slice2 = mem::transmute::<&mut [T], &mut [T]>(slice);
            // first: transmute is not type safe; all it checks is that T and
            // U are of the same size. Second, right here, you have two
            // mutable references pointing to the same memory.
            (&mut slice[0..mid], &mut slice2[mid..len])
        }
    }

    // This gets rid of the type safety problems; `&mut *` will *only* give
    // you an `&mut T` from an `&mut T` or `*mut T`.
    fn split_at_mut_casts<T>(slice: &mut [T], mid: usize)
                             -> (&mut [T], &mut [T]) {
        let len = slice.len();
        assert!(mid <= len);
        unsafe {
            let slice2 = &mut *(slice as *mut [T]);
            // however, you still have two mutable references pointing to
            // the same memory.
            (&mut slice[0..mid], &mut slice2[mid..len])
        }
    }

    // This is how the standard library does it. This is the best method, if
    // you need to do something like this
    fn split_at_stdlib<T>(slice: &mut [T], mid: usize)
                          -> (&mut [T], &mut [T]) {
        let len = slice.len();
        assert!(mid <= len);
        unsafe {
            let ptr = slice.as_mut_ptr();
            // This now has three mutable references pointing at the same
            // memory. `slice`, the rvalue ret.0, and the rvalue ret.1.
            // `slice` is never used after `let ptr = ...`, and so one can
            // treat it as "dead", and therefore, you only have two real
            // mutable slices.
            (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.add(mid), len - mid))
        }
    }

    //  }}}

    //  'mem::transmute_copy<T,U>' 
    //  Copies 'size_of<U>' bytes of of '&T' and interprets them as a 'U'
    //  (even more unsafe than transmute)
    //  (does not perform size check, it is UB for size_of<U> > size_of<T>)
}

