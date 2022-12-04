//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-12-01T00:32:04AEDT 'unsafe' allows Rust to be written in Rust ... it allows efficent buffer management of Vec (I follow), std::io to talk to the OS (I'm ok-<ish>), provide std::sync/std::thread concurrency primatives (I follow less<?>) ... ((now that you mention it) please explain the first claim too)
//  Ongoing: 2022-12-01T22:38:09AEDT How is newtype 'Ascii' enforcing behaviour at runtime - (won't any error due to non-ascii bytes be at runtime?) [...] (or does 'protected at compiletime' mean that if the program compiles, any such errors that arise will be caught?)
//  Ongoing: 2022-12-01T22:50:02AEDT A struct-tuple (tuple-struct) with a single variable has the same layout in memory as that variable would have(?)
//  Ongoing: 2022-12-01T22:54:29AEDT (recall), implementing 'From' also implements 'To'(?)
//  Ongoing: 2022-12-01T23:00:57AEDT Is invalid utf8 data in a String considered undefined behaviour? 
//  Ongoing: 2022-12-01T23:15:17AEDT meaning of the term 'contact' in the context of Rust
//  Ongoing: 2022-12-01T23:22:01AEDT <(definition: aliasing - where two references point to <the-same/overlapping> memory)>
//  Ongoing: 2022-12-02T00:29:09AEDT detectable-ness of Rust UB (is Rust like C in that a body of code cannot reasonably be judged to be UB free?)
//  Ongoing: 2022-12-04T19:27:32AEDT 'Rust code that does not use unsafe features is guaranteed to follow all these rules if it compiles' (where breaking said rules is UB) ... ((another source that says) not using unsafe features = no UB(?))
//  Ongoing: 2022-12-04T20:07:11AEDT (remind me) what is a deref coercision
//  Ongoing: 2022-12-04T20:26:11AEDT overflow in our 'distance()' (pointer subtraction function) (conversion from *const T to isize)(?) [...] (book acknowledges(?) that it is relying on start/end pointer belong to the same memory block to avoid overflow when converting to 'isize' (is that a hard rule or a practical one?))
//  Ongoing: 2022-12-04T20:51:26AEDT (does a type like 'RefWithFlag') (using an 'assert' in the ctor) catch invalid use at compile-time or runtime (and what is the significance of this vis-a-vis Rust safety) 
//  Ongoing: 2022-12-04T21:01:46AEDT lifetimes of references borrowed from derefenced raw pointers - a topic no sufficently covered here(?) 
//  Ongoing: 2022-12-04T23:29:53AEDT 'The size of a type is rounded up to a multiple of its aligment' -> this is true of Rust, or this is true more generally?
//  Ongoing: 2022-12-04T23:32:39AEDT any primative alignments for which x86/ARM differ?
//  Ongoing: 2022-12-04T23:34:46AEDT alignment of tuples (being smaller than their size?)
//  Ongoing: 2022-12-04T23:58:14AEDT clairfy, 'offset' vs 'wrapping_offset' (book says the later is for going beyond end of the array (going by name alone, shouldn't that be the former?))
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//  Continue: 2022-12-04T23:57:37AEDT 'example_raw_pointers/pointer_arithmetic', 'p.offset(o)' / 'p.wrapping_offset(o)' examples

//  Use pointers only were references are inadequate (memory management / FFI [foreign-function-interfaces] / <>)
//  References: shared '&' / mutable '&mut'
//  Pointers: '*const'  / '*mut*

//  Shared references in long form: "&'a T", parameters: 'a / T
//  pointers have no lifetime parameter (compiler cannot check validity of use)
//  Pointers: stored in 'usize', can be null

//  Pointers can be cast to different types, different constness/mutablity using 'as'
//  References can be cast to pointers using 'as'
//  Pointers can be converted to references by dereferencing them and borrowing a reference from the result
//  Pointers are not automatically dereferenced by '.'

//  C++ smart pointer equivalents:
//      std::unique_ptr         Box
//      std::shared_ptr         Arc


//  Rust provides the 'unsafe' block for certain operations
//  <(Rust's normal level of safety is not available (in unsafe blocks)/(for unsafe operations))>
//  Unsafe features can result in undefined behaviour if not used <carefully/correctly>
//  (Undefined behaviour is bad(TM))


//  <(The declaration of an unsafe block is an assurance to Rust that the code within does not break its safety rules)>


fn example_unsafe_blocks()
{
    //  An unsafe block is an ordinary block preceded by the keyword 'unsafe'
    unsafe {
        String::from_utf8_unchecked(vec![20,30,40,50]);
    }

    //  Avoid unsafe blocks except where they are strictly necessary

    //  Unsafe blocks permit 4 additional operations:
    //      unsafe functions can be called
    //      raw pointers can be dereferenced
    //      mutable static variables can be accessed
    //      FFI functions/variables can be accessed

    //  Safe code can create, pass, and compare raw pointers

    //  <(what makes a function unsafe? (consider 'String::from_utf_unchecked' <- function is unsafe because it violates String's <guarantee/invariant> to always be valid utf8(?)))>

    fn eg_ascii() 
    {
        //  Custom type 'Ascii', buffer guaranteed to hold valid ASCII
        //  Use unsafe block to allow efficent conversion to String, without requiring unsafe interface
        //  Since Vec<u8> Ascii.0 is a private variable, it can only be accessed by associated functions - this allows us to validate any data placed in it
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Ascii(Vec<u8>);
        #[derive(Debug, Eq, PartialEq)]
        pub struct NotAsciiError(pub Vec<u8>);
        impl Ascii {
            pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
                if bytes.iter().any(|&b| !b.is_ascii()) {
                    return Err(NotAsciiError(bytes))
                }
                Ok(Ascii(bytes))
            }
        }
        impl From<Ascii> for String {
            fn from(ascii: Ascii) -> String {
                unsafe { String::from_utf8_unchecked(ascii.0) }
            }
        }

        //  'Ascii' is an example of a newtype - a wrapper around an existing type that enforces certain behaviours (and type safety) <(at compile-time)>

        let b: Vec<u8> = b"ASCII and ye shall recieve".to_vec();
        let a1: Ascii = Ascii::from_bytes(b).unwrap();
        let a2: Ascii = a1.clone();
        let s1: String = String::from(a1);
        let s2: String = Ascii::into(a2);
        assert_eq!(s1, "ASCII and ye shall recieve");
    }
    eg_ascii();

    println!("example_unsafe_blocks, DONE");
}


fn example_unsafe_functions()
{
    //  An unsafe function is a function where 'fn' is preceded by the keyword 'unsafe'
    //  Unsafe functions can only be called from inside unsafe blocks
    //  The body of an unsafe function is an unsafe block

    //  Declaring a function 'unsafe' is a warning that it has a contact that must be satisfied to avoid undefined behaviour
    //  In this case, we are not validating that 'bytes' is valid ASCII data
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Ascii(Vec<u8>);
    impl Ascii {
        //  We declare this function unsafe because the caller must follow rules Rust cannot enforce to avoid undefined behaviour
        //  Note that it does not return a Result
        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    //  <(The presence of unsafe functions implies invalid inputs can cause undefined behaviour)>
    //  <(The consiquences of violating the contract of an unsafe block may not occur until after leaving that block)>
    //  To avoid UB, not just the unsafe function must be correct, so must all code that calls it

    //  A function that can be misused in a way that compiles, but causes UB, is should be marked unsafe 
    //  (and a function that cannot should not be market unsafe, even if it contains unsafe blocks)

    println!("example_unsafe_functions, DONE");
}


fn example_undefined_behaviour()
{
    //  UB is a powerful tool for compiler writers to permit optimizations

    //  <('aliasing' refers to two variables pointing to the same value)>

    //  In Rust, mutable references never alias. Breaking this rule in any way results in undefined behavior.
    //  There is no safe way to create two &mut references to the same value, and creating two mutable references via unsafe code is UB.

    //  Rust optimises code in accordance with its rules for well-behaved programs
    //          must not read uninitialized memory
    //          must not create invalid primitives 
    //                  null references/boxes
    //                  bools that are not 0/1
    //                  invalid enum values
    //                  non unicode char / non utf8 str/String
    //          no reference may outlive its referent
    //          shared references are read-only
    //          mutable references are exclusive
    //          must not deference null / incorrectly-aligned / dangling pointers
    //          must not use pointer to access memory outside allocation associated with pointer
    //          must not contain data races (two threads writing/reading memory without synchronization)
    //          <(must not unwind <across> a call made from another language)>
    //          must not violate contracts of standard library functions
    //  Violating any of these rules is UB
    //  Rust code that does not use unsafe features is guaranteed to follow all these rules if it compiles

    //  <(another source on UB in Rust(?))>
    //  <>

    println!("example_undefined_behaviour, DONE");
}


fn example_unsafe_traits()
{
    //  <(An unsafe trait has a contract Rust cannot check that implementers must satisfy to avoid UB)>

    //  Classic example of unsafe traits: 'std::marker::Send' / 'std::marker::Sync'
    //  'Send' requires implementers to be safe to move to another thread
    //  'Sync' requires implementers to be safe to share among threads by shared reference

    //  Another unsafe trait is 'core::nonzero::Zeroable' (nightly) for types that can be zero-ed 
    pub unsafe trait Eg_Zeroable {}
    unsafe impl Eg_Zeroable for u8 {}
    fn zeroed_vector<T: Eg_Zeroable>(len: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(len);
        unsafe {
            std::ptr::write_bytes(result.as_mut_ptr(), 0, len);
            result.set_len(len);
        }
        result
    }
    let v: Vec<u8> = zeroed_vector(1000);
    assert!(v.iter().all(|&u| u == 0));

    //  0 is not a valid value for a reference, hence implementing 'Eg_Zeroable' for a container holding a reference is undefined (and Rust will not produce an error)

    //  <(unsafe code must not depend on the correctness of the implementation of safe traits)>
    //  <('std::hash::Hasher' is not an unsafe trait, but it has a requirement: that the same bytes produce the same hash every time)>

    println!("example_unsafe_traits, DONE");
}


fn example_raw_pointers()
{
    //  A raw pointer is equivalent to a C/C++ pointer
    //  Raw pointers can form all sorts of structures that Rust's constrained pointer types cannot
    //  They can only be dereferenced in an unsafe block, since Rust cannot verify their use is safe
    //  (Creating, modifying, and comparing pointers is considered safe)
    //  A raw pointer can be NULL ('std::ptr::null()')

    //  <(A raw pointer has the same size as 'usize' / 'isize')>
    //  Converting a raw pointer to/from a usize is well defined <(what about isize?)>

    //  Raw pointers are neither 'Send' / 'Sync'
    //  (Therefore, no type that includes raw pointers implements these traits by default)
    //  <(This makes raw pointers safe to share between threads)>

    //  There are two kinds of raw pointer: '*mut T' / '*const T'

    let mut x = 10;
    let y = Box::new(20);

    //  Raw pointers can be cast from references, and de-referenced with the '*' operator
    let p_x  = &mut x as *mut i32;
    let p_y = &*y as *const i32;
    unsafe { *p_x += *p_y; }
    assert_eq!(x, 30);

    //  Example: convert Option<&T> to a raw pointer
    fn option_to_raw<T>(opt: Option<&T>) -> *const T {
        match opt {
            None => std::ptr::null(),
            Some(r) => r as *const T,
        }
    }
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());

    //  <(a raw pointer to an unsized type is a fat pointer (just like a reference or Box would be))>
    //  '*const [u8]' includes a length as well as an address

    //  A pointer to a trait object like '*mut std::io::Write' includes a vtable as well as an address

    //  Raw pointers are not automatically dereferenced by the '.' operator
    //  Use '(*p).method()'

    //  Raw pointers do not implement Deref, deref coercions do not apply

    //  Rust will implicitly coerce references to raw pointers (but not vice-versa)

    //  The comparison operators compare the address stored in a new pointer
    //  Hashing a raw pointer hashes the address, not the value stored there

    //  'std::fmt::Display' does not support raw pointer
    //  'std::fmt::Debug' / 'std::fmt::Pointer' display raw pointers as hex addresses (without dereferencing them)

    //  The '+' operator does not handle raw pointers
    //  <(Use 'offset()' / 'wrapping_offset()' methods to add to a pointers value)>

    //  The '-' operator does not handle raw pointers
    //  (But it is possible to implement a 'distance()' method ourselves)
    fn distance<T>(l: *const T, r: *const T) -> isize {
        (l as isize - r as isize) / std::mem::size_of::<T>() as isize
    }
    let trucks = vec!["garbage", "dump", "moonstruck"];
    let first = &trucks[0];
    let last = &trucks[2];
    assert_eq!(distance(last, first), 2);
    assert_eq!(distance(first, last), -2);

    //  Some conversions between references and pointers require several steps
    //&vec![42_u8] as *const String;                    //  invalid
    &vec![42_u8] as *const Vec<u8> as *const String;

    //  'as' cannot cast raw pointers into references
    //  This requires the pointer to be dereferenced (in an unsafe block), then borrowing a reference from the dereferenced value
    //  A reference created this way has an unconstrained lifetime

    //  Some types have methods like 'as_ptr()' / 'as_mut_ptr()' to return raw pointers to their contents
    //  Owning pointer types like 'Box' / 'Rc' / 'Arc' have 'into_raw()' / 'from_raw()' that convert to/from raw pointers

    //  A pointer can also be created by conversion from an integer
    //  (although only integers obtained from pointers in the first place should be used this way)


    fn dereferencing_safely()
    {
        //  Dereferencing null / dangling pointers is UB
        //  Dereferencing pointers not properly aligned for their referent type is UB

        //  Only borrow a reference to a dereferenced raw pointer if it follow the rules:
        //          No reference may outlive its referent
        //          Shared access is read only
        //          Mutable access is exclusive access

        //  Do not use the value of a raw pointer unless it is valid value of its given type

        //  'offset()' / 'wrapping_offset()' may only be used to access bytes within the block of memory the origional poitner refered to (or the first byte beyond it)
        //  (any pointer produced by manual pointer arithmetic must also follow this rule)

        //  Do not violate the invariants of the type being accessed by raw pointer
    }
    dereferencing_safely();


    fn eg_RefWithFlag()
    {
        //  Here we create custom type, 'RefWithFlag', which uses bit-manipulation to stores a reference of a 2-byte-aligned and a bool value in a single machine word (The zero-th byte of a 2-byte-aligned value will always be zero - we use this space to store our bool)

        //  <('PhantonData' is a type that occupies no space)>
        use std::marker::PhantomData;

        //  <(get the byte-alignment of a type)>
        use std::mem::align_of;

        //  'behaves_like' is necessary to provide lifetime information for reference returned by 'get_ref'

        pub struct RefWithFlag<'a, T: 'a> {
            ptr_and_bit: usize,
            behaves_like: PhantomData<&'a T>,
        }

        impl<'a, T: 'a> RefWithFlag<'a, T> {
            pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
                assert!(align_of::<T>() % 2 == 0);      //  only 2-byte aligned types
                RefWithFlag {
                    ptr_and_bit: ptr as *const T as usize | flag as usize,
                    behaves_like: PhantomData,
                }
            }
            pub fn get_ref(&self) -> &'a T {
                unsafe {
                    let ptr = (self.ptr_and_bit & !1) as *const T;
                    &*ptr
                }
            }
            pub fn get_flag(&self) -> bool {
                self.ptr_and_bit & 1 != 0
            }
        }

        let v = vec![10, 20, 30];
        let f = RefWithFlag::new(&v, true);
        assert_eq!(f.get_ref()[1], 20);
        assert_eq!(f.get_flag(), true);

        //let x: u8 = 53;                               //  1-byte aligned type
        //let f = RefWithFlag::new(&x, true);           //  panics at runtime
    }
    eg_RefWithFlag();


    //  Nullable pointers
    //  A null raw pointer is a zero address
    //  'std::ptr::null<T>()' returns a '*const T' null pointer
    //  'std::ptr::null_mut<T>()' returns a '*mut T' null pointer

    //  Checking whether a raw pointer is null:
    //      '.is_null()' - return whether a pointer is null
    //      '.as_ref()' - return an Option<&'a T> (which is None in the case of a null pointer)


    fn type_size_and_alignment()
    {
        use std::mem::size_of;
        use std::mem::align_of;

        //  Any Sized type occupies a constant number of bytes in memory, and must be placed at an address that is some multiple of its alignment value (which is architecture specific)
        //  <(This alignment value will always be a power of 2)>
        //  The size of a type is rounded up to a multiple of its aligment

        //  'std::mem::size_of<T>()' returns the size of type 'T' in bytes
        //  'std::mem::align_of::<T>()' returns the alignment of type 'T"

        assert_eq!(8, size_of::<(i32, i32)>());
        assert_eq!(8, size_of::<(i32, u8)>());
        assert_eq!(4, align_of::<(i32, i32)>());
        assert_eq!(4, align_of::<(i32, u8)>());

        //  <(The size/alignment of an Unsized type depends on the  value at hand)>
        //  'std::mem::size_of_val(x: &T)' returns the size of a given reference
        //  'std::mem::align_of_val(x: &T)' returns the alignment of a given reference

        let s: &[i32] = &[1, 3, 9, 27, 81];
        assert_eq!(5*size_of::<i32>(), std::mem::size_of_val(s));
        assert_eq!(align_of::<i32>(), std::mem::align_of_val(s));

        let t: &str = "alligator";
        assert_eq!(9*size_of::<u8>(), std::mem::size_of_val(t));
        assert_eq!(align_of::<u8>(), std::mem::align_of_val(t));

        use std::fmt::Display;
        let unremarkable: &dyn Display = &193_u8;
        let remarkable: &dyn Display = &0.0072973525664;
        //  Information is that of underlying type, which is determine from trait object vtable 
        assert_eq!(size_of::<u8>(), std::mem::size_of_val(unremarkable));
        assert_eq!(align_of::<u8>(), std::mem::align_of_val(unremarkable));
        assert_eq!(size_of::<f64>(), std::mem::size_of_val(remarkable));
        assert_eq!(align_of::<f64>(), std::mem::align_of_val(remarkable));
    }
    type_size_and_alignment();


    fn pointer_arithmetic()
    {
        use std::marker::PhantomData;

        //  The elements of an array / slice / vector are laid out as a contiguous block of memory
        //  Elements are reguarly spaced and consistent size
        //  This allows us to access elements in memory given the address of the start of the block, and the offset (size type * element index)

        //  This sequential layout makes raw pointers useful as array traversal bounds
        //  It is valid to use the first byte after the end of the array as a bound

        //  Definition: standard library iterator over a slice
        struct Eg_Iter<'a, T: 'a> {
            ptr: *const T,
            end: *const T,
            //  ...
            unused: PhantomData<&'a T>,         //  (to silence "unused 'a" warning)
        }

        //  <(Use 'p.offset(o)' to access '*(p+o)')>
        //  <(Use 'p.wrapping_offset(o)' to offset pointers beyond the limits of the array
        //  (it is undefined to dereference a pointer offset beyond the end of the array)
    }
    pointer_arithmetic();


    fn moving_into_outOf_memory() 
    {
        //  <>
    }
    moving_into_outOf_memory();


    fn panic_safely_in_unsafe_code()
    {
        //  <>
    }
    panic_safely_in_unsafe_code();


    println!("example_raw_pointers, DONE");
}


fn example_Foreign_Functions()
{
    //  <>

    println!("example_Foreign_Functions, DONE");
}


fn main() 
{
    example_unsafe_blocks();
    example_unsafe_functions();
    example_undefined_behaviour();
    example_unsafe_traits();
    example_raw_pointers();
    example_Foreign_Functions();
}

