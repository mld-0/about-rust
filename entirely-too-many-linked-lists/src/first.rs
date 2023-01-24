//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-24T00:12:06AEDT a function receiving '&mut self' <(cannot move out of borrowed reference?)> <(must leave itself in a valid state?)>
//  Ongoing: 2023-01-24T00:37:52AEDT demonstrate that there is a problem with this linked list implementation when no 'drop' is provided?
//  }}}
use std::mem;

//  Continue: 2023-01-24T00:42:55AEDT why we needed to implement 'Drop' for 'List'

//  A linked list is a bunch of data on the heap

//  Invalid: recursive type has infinite size
//pub enum List {
//    Empty, 
//    Elem(i32, List),                      //  no way to know how large this expression is
//}

//  We use 'Box<T>' as indirection

//  Suboptiomal: An enum occupies the size of its largest element
//  Here, our trailing 'Empty' node is wasting space
//#[derive(Debug)]
//enum List<T> {
//    Elem(T, Box<List1<T>>),               //  The compiler knows how large this is
//    Empty,
//}


//  First Exercise: A Bad Linked List

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem, 
        //  next: self.head,                      //  error, cannot move out of borrowed mut reference
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            },
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

//  Destructors are provided by the trait 'Drop'
//pub trait Eg_Drop {
//    fn drop(&mut self);
//}

//  <(Need to manually implement 'Drop' to hoist nodes out of their boxes? (why?))>
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

