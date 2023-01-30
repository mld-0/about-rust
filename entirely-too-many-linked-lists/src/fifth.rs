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
//  }}}
use std::mem;
use std::ptr;

//  Continue: 2023-01-30T23:22:41AEDT complete exercise

//  On unsafe Rust (see below)
//  LINK: https://doc.rust-lang.org/nightly/nomicon/

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


//  Miri: Tool for detecting undefined behaviour / memory errors in Rust programs
//  LINK: https://github.com/rust-lang/miri
//  (catches some, not all, undefined behaviour)
//  Installing/using Miri:
//  <()>


//  Pointer aliasing:
//  <()>


//  Stack borrows:
//  <()>


//  Invalid: storing a reference to ourselves, inside ourselves is invalid Rust
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


//  Example: valid as per tests, UB as per Miri
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut(), }
    }
    pub fn push_end(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None, });
        //  Getting raw pointer to contents of Box:
        //  {{{
        //let raw_tail: *mut _ = &mut *new_tail;
        //let raw_tail = &mut *new_tail as *mut _;
        //let raw_tail: *mut _ = new_tail.as_mut();
        //  }}}
        let raw_tail = new_tail.as_mut() as *mut _;
        if !self.tail.is_null() {
            unsafe { (*self.tail).next = Some(new_tail); }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
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


#[cfg(test)]
mod test {
    use super::List;

    #[test] 
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        list.push_end(1); list.push_end(2); list.push_end(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        list.push_end(4); list.push_end(5);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);
    }

}

