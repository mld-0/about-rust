//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![feature(negative_impls)]
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-28T23:16:41AEDT "Rust doesn't really have an opinion on how to do concurrency/parallelism" -> (a statement written about Rust 1.0? (how true is it still?)) [...] (later claim: "Rust pretty blatantly inherits the memory model for atomics from C++20")
//  Ongoing: 2023-03-01T02:01:27AEDT claim: Send/Sync are the *only* automatically derived traits(?)
//  Ongoing: 2023-03-01T02:10:17AEDT compiler doesn't object to manually implementing either Send/Sync (for a type for which neither is automatically derived) without the other
//  Ongoing: 2023-03-01T02:19:06AEDT which standard library containers</other-types> are Send/Sync?
//  }}}

//  Continue: 2023-02-28T23:43:05AEDT data_race_conditions, explanations for Examples 1/2, demonstration of race/error
//  Continue: 2023-02-28T23:44:09AEDT complete chapter.


//  <(Rust provides standard OS threads/blocking sys-calls instead of trying to define its own concurrency paradigm)>
//  <(Concurrency should be contained within a library, making interacting with other code a matter of providing the correct lifetimes and Send/Sync where appropriate)>


#[test]
fn data_race_conditions()
{

    //  <(A data race is UB, and therefore impossible in Safe Rust)>
    //  Safe Rust guarantees an absence of data races, that is where:
    //          two/more threads concurrently accessing a memory location,
    //          one/more of them is a write, and
    //          one/more of them is unsynchronized

    //  This is mostly prevented through Rust's ownership system (exclusivity of mutable references)
    //  (However, interior mutability complicates this, hence the 'Send' / 'Sync' traits)

    //  Rust does not prevent general race conditions:
    //  A Safe program can get deadlocked or have incorrect synchronization
    //  <(However, for a race condition to violate memory safety requires Unsafe code)>


    //  Example: <(1)>
    //  <>
    let data = vec![1,2,3,4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();
    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    });
    println!("{}", data[idx.load(Ordering::SeqCst)]);


    //  Example: <(2)>
    //  <>
    let data = vec![1,2,3,4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();
    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    });
    if idx.load(Ordering::SeqCst) < data.len() {
        unsafe {
            println!("{}", data.get_unchecked(idx.load(Ordering::SeqCst)));
        }
    }
}


#[test]
fn send_and_sync()
{
    //  std::marker::Send / std::marker::Sync: unsafe traits fundamental to concurrency in Rust
    //  (Incorrectly implementing unsafe traits can lead to UB)

    //  Send: a type is safe to send to another thread
    //  Sync: a type is safe to share between threads (T is Sync iff &T is Send)
    //  These are marker traits (they do not have associated functions/data)

    //  These are auto traits: any type comprised entirely of Send/Sync types is Send/Sync
    //  Most primitives are both Send/Sync (meaning most types are Send/Sync)
    //  Exceptions: 
    //          raw pointers are neither Send/Sync
    //          Rc is neither Send/Sync
    //          UnsafeCell/Cell/RefCell is not Sync
    //  (Rc/RefCell are fundamentally thread-unsafe, while raw pointers imply untracked ownership)
    
    //  We can implement Send/Sync ourselves for types which do not automatically derive it
    struct MyBox(*mut u8);
    unsafe impl Send for MyBox {}
    unsafe impl Sync for MyBox {}

    struct SpecialThreadToken(u8);

    //  We can also unimplement Send/Sync for a type which automatically derives it
    //
    //  <(not available (even in nightly?) method: negative_impls)>
//  #![feature(negative_impls)]
//  impl !Send for SpecialThreadToken {}
//  impl !Sync for SpecialThreadToken {}
    //
    //  <(available method: marker types)>
    use std::marker::PhantomData;
    use core::cell::Cell;
    struct SpecialThreadToken_ii(u8, PhantomData<Cell<()>>);

    //  Most types that use raw pointers should be sufficently encapsulated that Send/Sync can be derived
    //  <(all of Rust's standard collections are Send/Sync (when instantiated for Send/Sync types))>


    //  Example: Send/Sync safe 'Box<T>' implementation
    //  <>
}


#[test]
fn atomics()
{
    //  <>
}

#[test]
fn extra_resources()
{
    //  LINK: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
    //  {{{
    //  }}}

    //  LINK: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
    //  {{{
    //  }}}

    //  LINK: https://doc.rust-lang.org/std/sync/
    //  {{{
    //  High level synchronization objects
    //          Arc
    //          Barrier
    //          Condvar
    //          mpsc
    //          Mutex
    //          Once
    //          RwLock
    //  }}}
}

