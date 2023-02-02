//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
//  Ongoings:
//  {{{
//  }}}

//  The Rust programming language consists of safe and unsafe Rust
//  Safe Rust cannot cause UB

//  Safe Rust must implicitly trust any unsafe Rust it calls is written correctly
//  Unsafe Rust cannot make any assumptions about the correctness of the Safe Rust it interacts with


//  On functions, 'unsafe' means the function has constraints that cannot be checked by the compiler that the caller must uphold
//  On traits, 'unsafe' means the trait has contracts that any implementation must uphold
//  On blocks, 'unsafe' allows unsafe actions to be performed


//  'unsafe' is an API design decision. It indicates the responsibility is on the implementer to provide a correct implementation.


//  Standard library unsafe traits:
//          Send (can be moved to another thread)
//          Sync (can be shared by reference with another thread)
//          GlobalAlloc

//  <(Send / Sync are automatically implemented for types containing only other Send/Sync types)>


//  Unsafe actions:
//          Dereferencing raw pointers
//          Calling unsafe functions
//          Implementing unsafe traits
//          <(Mutate static variables)>
//          Access fields of a union


//  Safety is non-local: an unsafe block pollutes a whole module
//  Unsafe code must rely on the correctness of some safe code, but must not rely on the correctness of all safe code. Such code should be kept private at the module boundary.

