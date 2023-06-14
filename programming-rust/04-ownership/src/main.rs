//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-09-17T02:26:14AEST there must be a way to create a Vector<String> from literal strings without having to repeat the '.to_string()' method for each indervidual literal string
//  Ongoing: 2022-09-17T03:02:08AEST why 'pop()' has to be unwrapped but 'swap_remove()' doesn't
//  Ongoing: 2022-09-17T03:31:30AEST Copy vs Clone?
//  Ongoing: 2022-09-17T03:36:53AEST Would it be best if Rc example were really equivalent to Python example (use ['udon', 'ramen', 'soba'] as value)
//  Ongoing: 2022-09-17T03:38:29AEST meaning of '.clone()' for an Rc ((and) difference without it?)
//  }}}

//  Rust makes two promises
//  The programmer controls the lifetime of each <value>
//  The program will never use a pointer to a freed object
//  Rust meets these contradictory promises by restricting how pointers can be used
//  These same restrictions also permit safe (free from data races) concurrent programming

//  C++ solves the problem of ownership with resource management classes
//  It is the programmers responsibility not to use a pointer to a resource that has been freed

//  In Rust, every value has a single owner that determines its lifetime.
//  A variable owns its value. When the owner is freed (dropped), the owned value is dropped too.

//  When control leaves the block in which the variable is declared, it is dropped.
fn print_padovan() {
    let mut padovan = vec![1,1,1];
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10)=({:?})", padovan);
}   //  'padovan' dropped here

//  A Box owns the heap space it points to.
fn example_box_label() {
    let point = Box::new((0.625,0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}   //  'point' / 'label' dropped here


//  Structs own their fields
//  Arrays and vectors own their elements

//  Each value has a single owner
//  A single variable may own many other values.
//  Every value in a Rust program is a member of some tree, and each tree is rooted at some variable.
//  The correct way to release a value is to remove it from the ownership tree.
//  <(Because ownership is singular, loops in this tree cannot form. It is this restriction that allows many of Rusts powerful safety guarantees)>

//  Values can be moved from one owner to another.
//  The standard library provides reference-counted pointer types 'Rc' and 'Arc' for when multiple ownership is required
//  One can 'borrow a reference' to a value; references are non-owning pointers with limited lifetimes.


//  Assignment
//  In Rust, most operations like assigning a value to a variable, passing it to a function, or returning it, do not perform copies, they perform moves. The source relinquishes ownership of the value to the destination and becomes uninitialized, and the destination now controls the value's lifetime.

//  Consider assignment in python versus C++
//      s = ['udon', 'ramen', 'soba']
//      t = s
//      u = s
//  Each assignment creates a new reference to the same list. Assignment is expensive.
//      vector<string> s = {"udon", "ramen", "soba"};
//      vector<string> t = s;
//      vector<string> u = s;
//  Each assignment creates a deep copy of the origional list. Assignment is expensive.


//  The equivalent code is an error in Rust:
fn example_list_assignment_move() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s;         //  's' is now uninitialized
    //let u = s;        // error, cannot assign from uninitialized variable
}
//  Use the '.clone()' method to copy a vector
//  <(generally clone can be understood to be a deep copy - but not necessarily since custom types are free to implement to implement clone)>
fn example_list_assignment_copy() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s.clone();
    let _v = s.clone();
}
//  (Alternatively use '.copy()' for values than can safely be duplicated via memcpy)


//  Initialization is providing a value for a variable as it comes into scope in a 'let' statement.
//  When we assign to a variable that is already initialized, the prior value is dropped.
fn example_assign_prior_initalized() {
    let mut a = "Govinda".to_string();
    a = "Siddhartha".to_string();           //  value "Govinda" is dropped here

    let mut b = "Govinda".to_string();
    let c = b;
    b = "Siddhartha".to_string();           //  Nothing is dropped
}


//  If a variable has been moved in any branch of a block, at the end of that block, it is considered uninitialized unless a new values is also provided
fn example_move_and_control_flow() {
    let c = true;
    let mut x = vec![10, 20, 30];
    if c {
        let j = 53;
    } else {
        let k = x;
    }
    //  x is now considered to be uninitialized
}


//  Rust does not permit elements to be moved out of the vector with assignment
fn example_move_and_indexed_content() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    println!("v=({:?})", v);

    //  If this were allowed, our vector would be left with an empty element.
    //let third = v[2];     //  error, cannot move out of vector

    //  Other options for removing a value from a vector:
    //  1)  pop value off end
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");
    //  2)  move out of the middle of the vector, moving last element into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    //  3)  swap in another value for the one we are taking 
    let third = std::mem::replace(&mut v[2], "ABC".to_string());
    //  <(4) '.take()'?)>
    assert_eq!(third, "103");
    assert_eq!(v, vec!["101", "104", "ABC"]);

    //  Collection types like Vec generally offer methods to consume all their elements in a loop
    let a = vec!["abc".to_string(), "def".to_string(), "hij".to_string()];
    for mut s in a {
        s.push('!');
        println!("{}", s);
    }
    //  'a' is now uninitalized

    //  Ongoing: 2022-09-17T03:14:36AEST difference between a mutable vector, and a vector of mutable things? 

    //  If one needs to move values out of a container, use 'Option<T>'
    struct Person { name: Option<String>, birth: i32 };
    let mut composer = Person { name: Some("Palestrina".to_string()), birth: 1525 };
    //  <(Remove a value from Option with '.take()')>
    let first_name = composer.name.take().unwrap();
    assert_eq!(first_name, "Palestrina");
    assert_eq!(composer.name, None);
}


//  Assignment of 'Copy' types performs a copy, not a move
//  Only types for which a bitwise copy is sufficent are suitable to be Copy types
//  All the standard integer, floating-point, char, and bool <primatives> are Copy types.
//  An array or tuple of Copy types is a Copy type.
//  <(As a general rule, any type with a non-trivial dtor cannot be a Copy type)>
fn example_copy_types() {
    let str1 = "ambulance".to_string();
    let str2 = str1;    //  performs move
    //  str1 is uninitalised

    let num1 = 36;
    let num2 = num1;    //  performs copy
    //  num1 is still defined

    //  By default, struct / enum types are not Copy.

    //  If a struct has all Copy types, it can be made a Copy type with:
    #[derive(Copy, Clone)]
    struct Label { num: u32 };

    //  Changing a type declared Copy to later be non-Copy is generally non-trivial.

    //  Rust does not permit customized copy/move-ctors.
    //  Every move is a byte-for-byte shallow copy.
    //  <(Every copy is also a byte-for-byte shallow copy except the source remains initalized)>

    //  Copy and Clone are examples of 'traits'
}


//  Rust provides reference-counted pointer types 'Rc' / 'Arc' 
//  These are generally similar, however Arc is atomic - safe to share between threads directly.
//  Use Rc where Arc is not required (Rust will prevent Rc being passed accross thread boundries).
//  A value owned by an Rc pointer is <(always?)> immutable.
fn example_rc_arc() {
    use std::rc::Rc;
    //  Note: this is equivelent to our Python assignment example (see above)
    let s: Rc<String>  = Rc::new("abcdefhijklmnopqrstuvwxyz".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}
//  It is possible to create a cycle of ownership with Rc pointers (see: 'interior mutability')
//  Use 'std::rc::Weak' for a <(non-owning?)> pointer (to avoid creating cycles)


fn main() {
    print_padovan();
    example_box_label();
    example_list_assignment_move();
    example_list_assignment_copy();
    example_assign_prior_initalized();
    example_move_and_control_flow();
    example_move_and_indexed_content();
    example_copy_types();
}

