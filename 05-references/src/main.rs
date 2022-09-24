//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-09-21T04:41:47AEST The '.' operator implicitly dereferences it left operand if needed -> any posibility for <ambiguity>?
//  Ongoing: 2022-09-21T04:50:37AEST mutable shared reference vs mutable reference
//  Ongoing: 2022-09-21T05:07:24AEST if comparison operators see-through references (of the same type) does 'assert_eq!' have the same behaviour?
//  Ongoing: 2022-09-21T05:18:37AEST rvalues in rust?
//  Ongoing: 2022-09-21T22:34:47AEST one can declare a non-mutable variable 'let x', and assign it a value in different statements (but we will not be allowed to use it until we do?)
//  Ongoing: 2022-09-21T23:55:05AEST (surely) a later chapter covers parameter lifetime syntax 'fn f<'a>(p: &'a i32)' in detail(?)
//  Ongoing: 2022-09-22T00:12:29AEST returning a reference that outlives reference recieved as argument (something we probably don't want to do) (or else the default wouldn't be giving them the same lifetime)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a,b| a*b)
}

//  'Box<T>' provides an owning pointer

//  In Rust, references are non-owning pointers.
//  Rust will enforce references must never outlive their referents.
//  Creating a reference is referred to as 'borrowing' (what has been borrowed must eventually be returned).

//  We use references to pass/access values without affecting ownership

//  Shared references (read only):
//      '&e' yields a shared reference to 'e'
//      <(If 'e' has type 'T', then '&e' has type '&T')>
//  Shared references are Copy.
//  We can create as many shared references as needed.

//  Mutable references (read/write):
//      '&mut e' yields a mutable reference to 'e'
//      <(If 'e' has type 'T', then '&mut e' has type '&mut T')>
//  If we create a mutable reference to a value, it <can/must> be the only reference to that value.

//  While any references to a value exist, that value cannot be modified by its owner.
//  <(If a mutable reference to a value exists, that value cannot be accessed by its owner)>

//  These rules ensure at compile-time there can be either multiple readers or a single writer to a value.

//  Passing a value to a function in a way that moves ownership is called passing by value.
//  Alternatively, we may pass by reference.

//  References are created with the '&' / '&mut' operators, and dereferenced with '*'.

//  Rust references are never null.
//  Integers can only be converted to references in unsafe blocks.
//  Rust provides 'Option<&T>' as an alternative to a nullable reference.


//  The '.' operator implicitly dereferences it left operand if needed.
//  <(equivalent when 'works' is a reference: 'works.sort()' / '(*works).sort()')>
//  (this works with any number of references to references)

//  The '.' operator can implicitly borrow a reference to its left operand if needed.
//  <(equivalent when 'works' is not a reference: 'works.sort()' / '(&mut works).sort()')>
//  (this works with any number of references to references)

//  With the exception of the '.' operator <(and the comparison operators?)>, we must use the borrow / dereference operators explicitly, (unlike C++) conversion to/from references does not happen implicitly.


//  C++ vs Rust References:
//      C++ converts implicitly between references/lvalues, Rust only converts implicitly for the '.' operator <(and comparison operators?)>
//      C++ references cannot be re-assigned, Rust references can
//      <(Only the C++ compiler can create references to references)>, Rust allows references to references


fn example_table() 
{
    //  Iterating over a shared reference to a <container> produces a shared reference to each element.
    fn show_table(table: &Table) {
        for (artist, works) in table {
            print!("works by {}: ", artist);
            for work in works {
                print!("'{}', ", work);
            }
            println!();
        }
    }
    //  Iterating over a mutable reference to a <container> produces a mutable reference to each element.
    fn sort_table_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }
    fn make_table() -> Table {
        let mut table = Table::new();
        table.insert( "Gesualdo".to_string(),
            vec![ "many madrigals".to_string(), "Tenebrae Responsoria".to_string() ]
        );
        table.insert( "Caravaggio".to_string(),
            vec![ "The Musicians".to_string(), "The Calling of St. Matthew".to_string() ]
        );
        table.insert( "Cellini".to_string(),
            vec![ "Perseus with the head of Medusa".to_string(), "a salt cellar".to_string() ]
        );
        table
    }

    let mut table = make_table();
    show_table(&table);
    sort_table_works(&mut table);
    println!("example_table DONE");
}


fn example_assigning_references() {
    //  Assigning to a reference makes it point at the new value
    let flag = true;
    let x = 10;
    let y = 20;
    let mut r = &x;
    if flag { r = &y; }
    assert!(*r == x || *r == y);
    //  Compare references with 'std::ptr::eq()'
    assert!(std::ptr::eq(r, &x) || std::ptr::eq(r, &y));
    //assert!(r == &x || r == &y);      //  <(valid for references to primatives, not valid for references to objects?)>
    println!("example_assigning_references DONE");
}


fn example_references_to_references() {
    struct Point { x: i32, y: i32 };
    let p = Point { x: 1000, y: 729 };
    let r: &Point = &p;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    //assert!(&p == r);     //  <invalid?>
    assert_eq!(rrr.y, p.y);
    println!("example_references_to_references DONE");
}


//  The comparison operators see through any number of references, so long as both operands have the same type
//  <(That is: comparing references of the same type compares their values)>
fn example_comparing_references() 
{
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));
    assert!(rrx == rry);
    assert!(!std::ptr::eq(rrx, rry));
    println!("example_comparing_references DONE");
}


//  Rust allows us to borrow a reference to <rvalues> any kind of expression
//  In which case, an anonymous variable is created to store that value
//  If assigned to a reference variable, this anonymous variable lasts as long as the variable it is assigned to
//  Otherwise, the anonymous variable lives to the end of the enclosing statement
//  These references can be used safely, as Rust does not allow dangling references
fn example_borrow_reference_anonymous_value() 
{
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
    println!("example_borrow_reference_anonymous_value");
}


//  References to slices and trait objects are fat pointers.
//  A reference to a slice contains the address and length of that slice.
//  A trait object is a reference to a value that implements a certain trait, it contains the address of the the value and a pointer to the traits implementation to that value.


//  Each reference is assigned a lifetime - the stretch of program for which it is safe to use. Rust will ensure we do not use a reference outside its lifetime.
fn example_reference_safety_lifetimes() 
{
    // invalid: 'x' does not live long enough to be used
    //let r; { let x = 1; r = &x; }
    //assert_eq!(*r, 1);    

    let x = 1; { let r = &x; assert_eq!(*r, x); }

    let mut v = vec![1,2,3];
    let r = &mut v[1]; 
    v[1] = 5;           //  cannot use 'r' beyond this point
    println!("v[1]=({})", v[1]);

    //  For references stored in a container, the lifetime of those references must enclose that of the data structure

    println!("example_reference_safety_lifetimes, DONE");
}


//  Rust uses 'static' to declare global variables. These last the lifetime of the program.
//  Every static must be initialized.
//  A mutable static variable is not thread-safe.

//  Equivalent:
//      fn f(p: &i32) {}
//      fn f<'a>(p: &'a i32) {}
//  Where 'a is the lifetime parameter of p
//  These functions <will/can> not <stash> the reference argument anywhere that will outlive the function call

fn example_reference_safety_receiving_parameters() 
{
    static mut STASH: &i32 = &128;

    //  Since we are assigning our parameter to a static variable, we must specify 'static as the parameter's lifetime
    fn assign_to_STASH(p: &'static i32) { unsafe { STASH = p; } }

    unsafe { println!("STASH=({})", STASH); }

    //  invalid: 'x' does not live long enough to be passed to static reference
    //let x = 53;
    //assign_to_STASH(&x);

    static X: &i32 = &53;
    assign_to_STASH(X);
    unsafe { println!("STASH=({})", STASH); }

    //  <(literals are static)>
    assign_to_STASH(&12);
    unsafe { println!("STASH=({})", STASH); }

    println!("example_reference_safety_receiving_parameters, DONE");
}


fn example_reference_safety_passing_parameters() 
{
    //  'p' will not be saved anywhere that outlives the function call (see above)
    fn recieve_nonstatic<'a>(p: &'a i32) { }

    //  'p' must live as long as 'static
    fn receive_static(p: &'static i32) { }

    let x = 10;
    recieve_nonstatic(&x);

    //  error, borrowed value does not live long enough
    //receive_static(&x);

    println!("example_reference_safety_passing_parameters, DONE");
}

//  When a function takes a single reference as argument, and returns a single reference, it is assumed those two references must have the same lifetime.
//  Equivalent:
//      fn f(v: &[i32]) -> &i32 {}
//      fn f<'a>(v: &'a [i32]) -> &'a i32 {}
//  In Rust, when we recieve a reference as argument and return a reference, the assumption is the returned reference points to something in the input (or possibly alternatively at a static value).

fn example_reference_safety_returning_references() 
{
    //  Reference returned has same lifetime as reference received as argument
    fn first_i<'a>(v: &'a [i32]) -> &'a i32 { return &v[0]; }

    //  invalid: 's' has same lifetime as 'p'
    //let s;
    //{ let p = vec![9,4,1,0,1,4,9]; s = first_i(&p); }
    //assert_eq!(*s, 9);

    let p = vec![9,4,1,0,1,4,9];
    let s = first_i(&p);
    assert_eq!(*s, 9);

    //  Declare a function with multiple lifetime parameters:
    fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 { r }

    println!("example_reference_safety_returning_references, DONE");
}
//  Since Rust will not compile unsafe code, it is a valid approach to start with the simplest possible definition, and then add <loosen-restrictions> add lifetime parameters until it compiles


//  A struct <or other type> that does not have a lifetime parameter does not contain other types with lifetime parameters (that is, references with non-'static lifetimes)
fn example_reference_safety_structs_containing_references() 
{
    //  invalid: must provide lifetime parameter
    //struct S1 { r: &i32 };

    //  References in structs <or other types> must give a lifetime parameter:
    struct S2 { r: &'static i32 };
    struct S3<'a> { r: &'a i32 };

    //  Types with lifetime parameters inside structs <or other types> must give a lifetime parameter

    //  invalid: must provide lifetime parameter
    //struct T3 { s: S3 };

    //  Provide our own lifetime parameter and use that for S3 
    struct T3_1<'a> { s: S3<'a> };
    //  Or use 'static
    struct T3_2 { s: S3<'static> };

    //  S2 does not contain a lifetime parameter
    struct T2 { s: S2 };

    //  <(Book claims example should fail:)>
    ////  Distinct lifetime parameters:
    //struct SA1<'a> { x: &'a i32, y: &'a i32 };
    //let x = 10; 
    //let r;
    //{
    //    let y = 20; 
    //    {
    //        let s = SA1 { x: &x, y: &y };
    //        r = s.x; 
    //    }
    //}

    //  Declare struct with multiple lifetime parameters:
    struct SA2<'a, 'b> { x: &'a i32, y: &'b i32 };

    println!("example_reference_safety_structs_containing_references, DONE");
}


fn example_reference_safety_omitting_lifetime_parameters() 
{
    struct SA<'a, 'b> { x: &'a i32, y: &'b i32 };

    //  <(For functions that do not return a reference, specifying lifetime parameters is not necessary - Rust will assign a distinct lifetime to each <spot/item/parameter> that needs one)> <(structs with references do however require lifetime parameters)>
    //  Equivalent:
    fn sum_rxy_i(r: &i32, s: SA) -> i32 { r + s.x + s.y }
    fn sum_rxy_ii<'a, 'b, 'c>(r: &'a i32, s: SA<'b, 'c>) -> i32 { r + s.x + s.y }

    //  <(If a function that returns a reference only has a single lifetime parameter for its arguments, Rust assumes that must be the lifetime parameter of the returned reference:)>
    //  Equivalent:
    fn first_third_i(p: &[i32; 3]) -> (&i32, &i32) { (&p[0], &p[2]) }
    fn first_third_ii<'a>(p: &'a [i32; 3]) -> (&'a i32, &'a i32) { (&p[0], &p[2]) }

    //  <(If a function that returns a reference has multiple lifetime parameters for its arguments, Rust requires that we specify which applies to the return value)>
    //fn f_i(x: &i32, y: &i32) -> &i32 { x }      //  invalid 
    fn f_ii<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 { x }

    //  <(However, if one of the arguments is a reference to 'self', that is assumed to be the lifetime of the return parameter)>
    struct StringTable { elements: Vec<String> }
    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    println!("example_reference_safety_omitting_lifetime_parameters, DONE");
}


fn example_sharing_vs_mutation() 
{
    fn extend(vec: &mut Vec<f64>, slice: &[f64]) { for x in slice { vec.push(*x); } }

    //  <(Throughout its life, a shared reference makes its referent read-only)>
    //  <(Throughout its life, a mutable reference makes its referent inaccessible)>

    let mut v = vec![4,8,19,27,34,10];
    let r = &v;
    v[0] = 5;                           //  reference 'r' no longer valid
    //println!("v[0]=({})", r[0]);      //  invalid

    let r = &v;
    let a = v;                          //  reference 'r' no longer valid
    //println!("v[0]=({})", r[0]);      //  invalid

    let mut v = vec![4,8,19,27,34,10];
    let r = &mut v;
    v[0] = 5;                           //  mutable reference 'r' no longer valid
    //r[0] = 8;                         //  invalid

    let r = &v;
    println!("v[0]=({})", v[0]);
    println!("r[0]=({})", r[0]);

    let r = &mut v;
    //println!("v[0]=({})", v[0]);      //  invalid, cannot access 'v' during lifetime of mutable reference 'r'
    //println!("r[0]=({})", r[0]);

    //  <(Modifying containers while using <pointers/iterators> to their contents is problematic in many languages)>
    //  Rust's rules for mutation and sharing prevent these kind of errors:
    //  During the lifetime of a shared reference, the value it points to is read only. 
    //  During the lifetime of a mutable reference, the value it points can only be read/modified though said reference
    //  <(This leaves us with no way to invalidate a reference)>

    //  Cannot borrow shared and mutable reference at the same time
    let mut wave = vec![0.0, 1.0, 0.0, -1.0];
    //extend(&mut wave, &wave);         //  invalid


    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    x += 10;


    println!("example_sharing_vs_mutation, DONE");
}



fn main() {
    example_table();
    example_assigning_references();
    example_references_to_references();
    example_comparing_references();
    example_borrow_reference_anonymous_value();
    example_reference_safety_lifetimes();
    example_reference_safety_receiving_parameters();
    example_reference_safety_passing_parameters();
    example_reference_safety_structs_containing_references();
    example_reference_safety_omitting_lifetime_parameters();
    example_sharing_vs_mutation();
}

