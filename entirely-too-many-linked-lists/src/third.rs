//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-29T21:37:36AEDT 'map()' <was/is> described as a chainable way to simplify match statements (from 'and_then()' Rust docs)
//  Ongoing: 2023-01-29T21:45:48AEDT (a more idiomatic alternative to 'unwrap_or(None)'?)
//  Ongoing: 2023-01-29T21:49:42AEDT (using a style like 'let list = list.prepend(1).prepend(2).prepend(3)' (is a functional programming thing(?)) (the need for 'let') / (the chaining of statements) (the lack of any 'mut')) [...] ('let' is necessary for each assignment because 'list' is immutable)
//  Ongoing: 2023-01-29T22:14:11AEDT (Rust rules that allow for 'while let Some(mut node) = Rc::try_unwrap(node)')
//  }}}
use std::rc::Rc;

//  Continue: 2023-01-29T22:27:10AEDT more about 'and_then()' / (other such functions)
//  Continue: 2023-01-29T22:28:19AEDT more about how functional programming


//  Third exercise:
//  Functional programming: Persistant immutable singly-linked list

//  Rust provides reference counting garbage collection with 'Rc'
//  Rc is like Box, but it can be duplicated, and we can only borrow a shared reference to its internals


//  'Clone' is implemented for nearly every type
//  It provides a generic way to "get another one just like this one"
//  Rc implements Clone by incrementing the reference count and returning a copy of itself


//  Using 'map()' on a function that returns 'Option<T>' results in a nested 'Option<Option<T>>'
//  'and_then()' is also known as flatmap: option.and_then(func) calls 'func' iff 'option' is not None.
//  <(It returns either 'func(option.unwrap())' or 'None')>
//  <('map().unwrap_or(None)' vs 'and_then()')>
//  LINK: https://hermanradtke.com/2016/09/12/rust-using-and_then-and-map-combinators-on-result-type.html/
//  {{{
//  }}}


//  'Rc::try_unwrap(node)'
//  Returns the inner value if the Rc has exactly 1 strong reference
//  Otherwise return Err


//  'Arc' is like 'Rc', except reference counts are modified atomically (making it threadsafe, unlike Rc)
//  (there is additional overhead due to atomic reference counting)
//  <(putting a type in 'Arc' doesn't magically make it thread safe)>

//  Rust models thread safety with traits: 'Send' / 'Sync'
//  Safe to move between threads: Send
//  Safe to shared between threads: Sync (cannot cause data races)
//  <(If 'T' is Sync, then '&T' is Send)>
//  <(These are automatically derived based on whether a type is totally composed of Send/Sync types)>

//  Almost every type is Send and Sync.
//  Most types are Send because they own their own data.
//  Most types are Sync because the only way to share them between threads is with shared references.

//  Types with interior mutability violate these rules.
//  They allow data to be mutated through a shared reference.
//  Cells provide single threaded interior mutability.
//  <(Locks provide multi threaded interior mutability)>



type Link<T> = Option<Rc<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    //  prepend: add to beginning
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }

    //  tail: remove from beginning
    pub fn tail(&self) -> List<T> {
        //List { head: self.head.as_ref().map(|x| x.next.clone()).unwrap_or(None) }
        List { head: self.head.as_ref().and_then(|x| x.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

}


//  Cannot implement 'into_iter()' or 'iter_mut()'
//  (Rc provides only shared references to elements)

//  Implement 'iter()'
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|x| {
            self.next = x.next.as_deref();
            &x.elem
        })
    }
}


//  Implement 'Drop'
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);
        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

}

