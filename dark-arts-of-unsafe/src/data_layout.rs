//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-02T21:04:36AEDT alignment can be smaller than size? (eg: 32-bit aligned 64-bit numbers?)
//  Ongoing: 2023-02-02T21:35:34AEDT Rust setting/parameter - enable output by default for 'cargo test' for this project?
//  }}}

//  Continue: 2023-02-02T21:54:50AEDT size-of / alignment-of
//  Continue: 2023-02-02T21:54:59AEDT more about dynamically sized types / Sized

//  Type Layout:
//  LINK: https://doc.rust-lang.org/nightly/reference/type-layout.html
//  {{{
//  }}}


//  All Rust types have an alignment specified in bytes
//  It is only valid to store values at addresses that are multiples of its alignment
//  A types size must always be a multiple of its alignment

//  Primitives are typically aligned to their size (this is platform specific)

//  Rust provides:
//          structs
//          tuples
//          arrays
//          enums
//          unions

//  An enum is field-less if none of its variants have associated data


#[test]
fn struct_layout()
{
    //  By default, all composite structures have an alignment equal to the maximum of their fields alignments

    //  Rust will insert padding where necessary to ensure all fields are properly aligned and that overall size is a multiple of alignment

    //  Rust is permitted to reorder fields. 
    //  Rust guarantees that all instances of 'A' will have their data laid out in the same way, but it does not guarantee that 'B' will have the same layout as 'A'
    //  (A struct with type parameters may have a different layout for each instantiation)
    struct A { a: i32, b: u64, }
    struct B { a: i32, b: u64, }
    //  Structs in C <(which does not re-order fields?)> are optimally sized when fields are either ordered smallest->largest or largest->smallest


    //  <(size_of / alignment_of?)>
    //  <>
}


#[test]
fn dynamically_sized_types()
{
    //  Dynamically sized types (DSTs): types without statically known size or alignment
    //  Can only exist behind a fat pointer (which contains the information missing from the type)
    //  Examples:
    //          trait objets: 'dyn MyTrait'
    //          slices: '[T]' / 'str'

    //  A trait object is a type that implements the trait specified
    //  A vtable specifies the implementation
    
    //  A slice is a view into some contiguous storage
    //  A fat pointer gives the address and length of the slice

    //  A struct can store a single DST as their last field (making them a DST)
    //  Example: using parameterised types to create such a struct we can create instances of
    struct MySuperSliceable<T: ?Sized> {
        info: u32,
        data: T,
    }
    let s = MySuperSliceable { info: 17, data: [0_u8; 8], };
    //  <(contention: custom DSTs are a half-baked feature)>

    //  <(sized?)>
    //  <>
}


#[test]
fn zero_sized_types() 
{
    //  Zero sized types (ZSTs): Rust allows types that occupy no space
    struct Nothing;
    struct LotsOfNothing { foo: Nothing, qux: (), baz: [u8;0], }

    //  ZSTs are useful in performing optimizations
    //  For example: 'Map<T,()>' can be used as an efficient implementation of 'Set<T>'
}


#[test]
fn empty_types()
{
    //  Empty types are types that cannot be instantiated
    enum Void {}

    //  This can indicate unreachability
    //  For example: 'Result<T,Void>' by definition cannot be Err

    //  Contention: '*const Void' is not a substitute for C's 'void*' (use '*const ()' instead)
    //  (Rust doesn't have safeguards against instantiating empty types in unsafe code - which is UB)
}


#[test]
fn extern_types()
{
    //  An incomplete proposal for modeling C's 'void*' and other "declared but never defined" types
}


#[test]
fn alternative_representations()
{
    //  Rust allows alternative data layout strategies to be specified

    //  'repr(C)'
    //  Use C-like layout (where possible)
    //  ZSTs are still zero-sized (unlike C)
    //  recommendation: use 'rust-bindgen' / 'cbindgen' to manage FFI boundaries

    //  'repr(transparent)'
    //  Can only be used on a struct/single-variant-enum that has a single non-zero sized field
    //  Ensures said struct will have the same layout as its only non-zero sized field

    //  'repr(u*)' / 'repr(i*)'
    //  Specify size to make a fieldless enum

    //  'repr(packed)'
    //  Force Rust to strip padding and align the type to a byte
    //  (this can cause UB, and can have significant performance impacts)

    //  'repr(align(n))'
    //  Forces type to have alignment of at least n, where n is a power of 2
    //  (can have performance benefits in certain cases)
}

