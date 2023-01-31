//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-30T22:19:25AEDT (no error compiler error for incorrect self-mutable-references List implementation unless we try and use it) (Rust's promise is "if it compiles it's correct"?)
//  Ongoing: 2023-01-30T22:57:34AEDT containers whose contents does/doesn't change address when moved
//  Ongoing: 2023-01-30T23:06:56AEDT (we have labeled push_end / pop_front correctly?)
//  Ongoing: 2023-01-30T23:15:16AEDT miri is an option (of its kind) or the option?
//  Ongoing: 2023-01-31T18:41:05AEDT miri requires test containing UB to be run to detect UB (compilation is not enough)
//  Ongoing: 2023-01-31T18:53:30AEDT error detected by miri comes with caravat 'this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental'
//  Ongoing: 2023-01-31T19:06:41AEDT run tests with output: 'cargo test -- --nocapture'
//  Ongoing: 2023-01-31T21:08:23AEDT making UBList correct (without resorting to an all-raw-pointer solution)? [...] spot the line(s) with the error(?)
//  Ongoing: 2023-01-31T21:27:56AEDT when is it allowed to cast '*const T' to '*mut T'?
//  Ongoing: 2023-01-31T21:41:43AEDT compilation flag - only run UB test cases if given?
//  Ongoing: 2023-01-31T21:42:45AEDT why 'push_end_fixed()' (appears to be) correct if we iterate through the list a pointer to the last node (but not if we use the one we have stored ... (are we sure the one we have stored is valid?)) [...] (is the exercise telling the truth that 'we are invalidating the reborrow of a pointer into a Box whenever we use the box) ... (the reason we have to get 'tail' ourselves is said pointer has been <invalidated> since we saved it?) [...] (test (besides miri) for have we invalidated our raw pointer?)
//  Ongoing: 2023-01-31T21:47:01AEDT when *won't* miri flag a shared reference being cast to a mutable pointer? (and is it still UB if there is a case where miri won't flag it?)
//  Ongoing: 2023-01-31T22:11:29AEDT 'pointer::add()' is not unsafe in exercise?
//  Ongoing: 2023-01-31T22:23:45AEDT 'arr[..]' slicing notation?
//  Ongoing: 2023-01-31T22:25:07AEDT (there must be better examples for split_at_mut / as_mut_ptr?)
//  Ongoing: 2023-01-31T22:28:27AEDT this exercise contains a lot of "I think" and "I don't know" for a guy who wrote a book on Rust UB?
//  }}}
use std::mem;
use std::ptr;

//  Continue: 2023-01-30T23:22:41AEDT complete exercise

//  On unsafe Rust (see below)
//  LINK: https://doc.rust-lang.org/nightly/nomicon/
//  {{{
//  }}}

//  Fifth exercise: Ok unsafe queue
//  A queue requires pushing/popping from opposite ends of our linked list
//  Pushing/popping from the end of a linked list efficiently requires us to store a pointer to the end
//  The goal is to use unsafe to provide a safe API


//  Contention: performance guarantees are part of an interface
//  (don't offer push_end/pop_end unless they are implemented efficiently)

//  Contention: raw pointers can be a prefereable alternative to Rc<RefCell>


//  'mem::replace(dest: &mut T, src: T) -> T'
//  Move 'src' into 'dest', returning previous value of 'dest'


//  Rust provides raw (C-like) pointers
//          *const T
//            *mut T
//  These allow us to circumvent many of Rusts restrictions
//  They can only be dereferenced in an unsafe block
//  <(Raw pointers are not automatically dereferenced)>

//  Getting a null pointer
//          ptr::null()                 0 as *const _
//          ptr::null_mut()             0 as *mut _


//  The contents of Box<T> has a stable address, even if we move it around
//  (dropping the box leaves us with a dangling pointer)
//  <(other containers?)>

//  Raw pointer into Box<T>:
//  <()>


//  Invalid: storing a reference to ourselves, inside ourselves
//  {{{
//  <(each part of the code is correct indervidually, but if we try and use it, Rust rejects our attempt to create multiple mutable references)>
//type Link<T> = Option<Box<Node<T>>>;
//pub struct List<'a, T> {
//    head: Link<T>,
//    tail: Option<&'a mut Node<T>>,
//}
//struct Node<T> {
//    elem: T,
//    next: Link<T>,
//}
//impl<'a, T> List<'a, T> {
//    pub fn new() -> Self {
//        List { head: None, tail: None, }
//    }
//    pub fn push(&'a mut self, elem: T) {
//        let new_tail = Box::new(Node { elem, next: None, });
//        let new_tail = match self.tail.take() {
//            Some(old_tail) => {
//                old_tail.next = Some(new_tail);
//                old_tail.next.as_deref_mut()
//            }
//            None => {
//                self.head = Some(new_tail);
//                self.head.as_deref_mut()
//            }
//        };
//        self.tail = new_tail;
//    }
//    pub fn pop(&'a mut self) -> Option<T> {
//        self.head.take().map(|head| {
//            let head = *head;
//            self.head = head.next;
//            if self.head.is_none() {
//                self.tail = None;
//            }
//            head.elem
//        })
//    }
//}
//  }}}

//  Verifying unsafe Rust is free of UB is a difficult challenge

//  Miri: Tool for detecting undefined behaviour / memory errors in Rust programs
//  LINK: https://github.com/rust-lang/miri
//  (catches some, not all, undefined behaviour)
//  Installing/using Miri:
//          rustup +nightly component add miri
//          (may require a specific nightly version: exercise uses 'nightly-2022-01-21')
//          rustup default nightly
//          cargo clean
//          cargo miri test     *or*    cargo miri run
//          (upon which, miri will ask to install rust-src for the current toolchain)
//  run without switching to nightly:
//          cargo +nightly miri test
//
//  Miri is an experimental interpreter for Rust's mid-level intermediate representation (MIR)
//  (like a combination of 'ubsan' and 'tsan')
//  It can detect:
//  {{{
//          Memory leaks
//          Out-of-bounds memory accesses and use-after-free
//          Invalid use of uninitialized data
//          Violation of intrinsic preconditions (an unreachable_unchecked being reached, calling copy_nonoverlapping with overlapping ranges, ...)
//          Not sufficiently aligned memory accesses and references
//          Violation of some basic type invariants (a bool that is not 0 or 1, for example, or an invalid enum discriminant)
//          Experimental: Violations of the Stacked Borrows rules governing aliasing for reference types
//          Experimental: Data races (but no weak memory effects)
//  }}}
//  (but it sufferes from both false-positives and false-negatives)


//  Stacked borrows:
//  <(An experimental semantic model for Rust)>


//  Pointer aliasing:
//  When can the compiler assume it's safe to cache values instead of loading them over and over

#[test]
fn example_reborrowing()
{
    //  We re-borrow mutable reference 'ref1' as 'ref2'
    //
    //  invalid: can't use ref1 until ref2 is dropped
    //let mut data = 10;
    //let ref1 = &mut data;
    //let ref2 = &mut *ref1;
    //*ref1 += 1;
    //*ref2 += 1;
    //assert_eq!(data, 12);
    //
    //  valid: ref2 is dropped before ref1 can be used
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    *ref2 += 1;
    *ref1 += 1;
    assert_eq!(data, 12);

    //  Re-borrows 'nest', with only the most recent being live at any given time.
    //  This forms a "borrow stack"
    //  The compiler can enforce correct usage of stacked borrows in safe code
}


#[test]
fn example_UB_stacked_borrows()
{
    //  When converting a reference to a raw pointer, it's *basically* like taking a re-borrow
    //  <(how does that work with the freedom of raw pointers? article: it's complicated and I don't know)>

    //  Contention: Once you start using raw pointers, try to only use raw pointers 
    //  (this makes it as unlikely as possible to accidentally lose the raw pointer's "permission" to access the memory)

    //  Given a pointer to one variable, it is undefined to use pointer arithemetic to try and get a pointer to another variable.

    //  Mistake in UB version of implementation: every time we accessed 'Box' containing final node, we were <probably?> invalidating the re-borrow of the raw pointer 'tail' to its contents

    //  invalid: runs correctly but flagged as UB by miri
    //  unsafe {
    //      let mut data = 10;
    //      let ref1 = &mut data;
    //      let ptr2 = ref1 as *mut _;
    //      let ref3 = &mut *ptr2;
    //      let ptr4 = ref3 as *mut _;
    //      // Access the first raw pointer first
    //      *ptr2 += 2;
    //      // Then access things in "borrow stack" order
    //      *ptr4 += 4;
    //      *ref3 += 3;
    //      *ptr2 += 2;
    //      *ref1 += 1;
    //      assert_eq!(data, 22);
    //  }
    //
    //  valid: pointers are used (and dropped) in the order they are re-borrowed
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;
        assert_eq!(data, 20);
    }
}


#[test]
fn example_UB_array_pointers()
{
    //  We can only borrow a mutable reference to a single element of an array
    //  Rust solves this by providing 'split_at_mut()' / 'as_mut_ptr()' 

    //  <(miri is objecting to the fact 'ptr3_at_1' is not in the borrow stack)>
    //  unsafe {
    //      let mut data = [0; 10];
    //      let ref1_at_0 = &mut data[0];
    //      let ptr2_at_0 = ref1_at_0 as *mut i32;
    //      //  'pointer::add()' is unsafe, 'pointer::wrapping_add()' safe(?)
    //      let ptr3_at_1 = ptr2_at_0.add(1);
    //      *ptr3_at_1 += 3;
    //      *ptr2_at_0 += 2;
    //      *ref1_at_0 += 1;
    //      println!("data=({:?})", data);
    //  }

    //  <(Splitting an array turns a borrow stack into a borrow tree)>

    //  'split_at_mut()'
    //  <()>
    unsafe {
        let mut data = [0; 10];
        let s1 = &mut data[..];
        let (s2_at_0, s3_at_1) = s1.split_at_mut(1);
        let r4_at_0 = &mut s2_at_0[0];
        let r5_at_1 = &mut s3_at_1[0];
        let p6_at_0 = r4_at_0 as *mut _;
        let p7_at_1 = r5_at_1 as *mut _;
        *p7_at_1 += 7;
        *p6_at_0 += 6;
        *r5_at_1 += 5;
        *r4_at_0 += 4;
        assert_eq!(data[0], 10);
        assert_eq!(data[1], 12);
        assert_eq!(data[2], 0);
    }

    //  'as_mut_ptr()'
    //  <(only correct for given order of reference/pointer useage?)>
    //  <()>
    unsafe {
        let mut data = [0; 10];
        let s1_all = &mut data[..];
        let p2_all = s1_all.as_mut_ptr();
        let p3_at_0 = p2_all;
        let p4_at_1 = p2_all.add(1);
        let r5_at_0 = &mut *p3_at_0;
        let r6_at_1 = &mut *p4_at_1;
        *r6_at_1 += 6;
        *r5_at_0 += 5;
        *p4_at_1 += 4;
        *p3_at_0 += 3;
        for i in 0..s1_all.len() {      //  can't use 'data' until 's1_all' has been dropped
            *p2_all.add(i) += i;
        }
        for (i, x) in s1_all.iter_mut().enumerate() {
            *x += i;
        }
        assert_eq!(data, [8,12,4,6,8,10,12,14,16,18]);
    }

}


#[test]
fn example_UB_shared_references()
{
}


#[test]
fn example_UB_interior_mutability()
{
}


#[test]
fn example_UB_Box()
{
}


//  Example: valid implementation as per tests, UB as per Miri
//  {{{
type UBLink<T> = Option<Box<UBNode<T>>>;
pub struct UBList<T> {
    head: UBLink<T>,
    tail: *mut UBNode<T>,
}
struct UBNode<T> {
    elem: T,
    next: UBLink<T>,
}
impl<T> UBList<T> {
    pub fn new() -> Self {
        UBList { head: None, tail: ptr::null_mut(), }
    }
    pub fn push_end(&mut self, elem: T) {
        self.push_end_UB(elem)
    }
    pub fn push_end_UB(&mut self, elem: T) {
        let mut new_tail = Box::new(UBNode { elem, next: None, });
        //  Getting raw pointer to contents of Box:
        //  {{{
        //let raw_tail: *mut _ = &mut *new_tail;
        //let raw_tail = &mut *new_tail as *mut _;
        //let raw_tail: *mut _ = new_tail.as_mut();
        //  }}}
        let raw_tail = new_tail.as_mut() as *mut _;
        if !self.tail.is_null() {
            unsafe { 
                (*self.tail).next = Some(new_tail); 
                self.tail = (*self.tail).next.as_mut().unwrap().as_mut() as *mut _;
            }
        } else {
            self.head = Some(new_tail);
            self.tail = self.head.as_mut().unwrap().as_mut() as *mut _;
        }
        //self.tail = raw_tail;
    }
    //pub fn push_end_UB(&mut self, elem: T) {
    //    let mut new_tail = Box::new(UBNode { elem, next: None, });
    //    //  Getting raw pointer to contents of Box:
    //    //  {{{
    //    //let raw_tail: *mut _ = &mut *new_tail;
    //    //let raw_tail = &mut *new_tail as *mut _;
    //    //let raw_tail: *mut _ = new_tail.as_mut();
    //    //  }}}
    //    let raw_tail = new_tail.as_mut() as *mut _;
    //    if !self.tail.is_null() {
    //        unsafe { (*self.tail).next = Some(new_tail); }
    //    } else {
    //        self.head = Some(new_tail);
    //    }
    //    self.tail = raw_tail;
    //}
    pub fn push_end_fixed(&mut self, elem: T) {
        let mut new_tail = Box::new(UBNode { elem, next: None, });
        if !self.tail.is_null() {
            //  valid: <(or is still UB, just uncaught by miri?)>
            let mut node = &mut self.head;
            while node.as_ref().unwrap().as_ref().next.is_some() {
                node = &mut node.as_mut().unwrap().as_mut().next;
            }
            let p1 = node.as_mut().unwrap().as_mut() as *mut _;
            let p2 = p1 as *mut UBNode<T>;
            //
            //  invalid:
            //let p2: *mut UBNode<T> = self.tail;
            unsafe { 
                (*p2).next = Some(new_tail); 
                self.tail = (*p2).next.as_mut().unwrap().as_mut() as *mut _;
            }
        } else {
            self.head = Some(new_tail);
            self.tail = self.head.as_mut().unwrap().as_mut() as *mut _;
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}
//  }}}



type Link<T> = *mut Node<T>;
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}



#[cfg(test)]
mod test {

    //use super::UBList;
    //#[test] 
    //fn basics_UB() {
    //    let mut list = UBList::new();
    //    assert_eq!(list.pop_front(), None);
    //    list.push_end(1); list.push_end(2); list.push_end(3);
    //    assert_eq!(list.pop_front(), Some(1));
    //    assert_eq!(list.pop_front(), Some(2));
    //    list.push_end(4); list.push_end(5);
    //    assert_eq!(list.pop_front(), Some(3));
    //    assert_eq!(list.pop_front(), Some(4));
    //    assert_eq!(list.pop_front(), Some(5));
    //    assert_eq!(list.pop_front(), None);
    //}

}

