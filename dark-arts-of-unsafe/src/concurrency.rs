//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-28T23:16:41AEDT "Rust doesn't really have an opinion on how to do concurrency/parallelism" -> (a statement written about Rust 1.0? (how true is it still?)) [...] (later claim: "Rust pretty blatantly inherits the memory model for atomics from C++20")
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
}


#[test]
fn atomics()
{
}

