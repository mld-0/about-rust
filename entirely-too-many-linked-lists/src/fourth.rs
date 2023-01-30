//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-29T22:42:45AEDT Rust books with a chapter/sub-chapter on Rc<RefCell<T>>(?)
//  Ongoing: 2023-01-29T22:44:04AEDT replace 'impl<T> List<T>' with 'impl<T> Node<T>' and there are no YCM warnings (but of course it fails to build) ... (presumedly we'd get such a warning in a --bin project with a main function?)
//  Ongoing: 2023-01-29T22:51:16AEDT where does the advice 'interior mutability should only be used as a last resort' fall vis-a-vis Rust linked list best practice?
//  Ongoing: 2023-01-29T22:57:04AEDT Using 'Ref' / 'RefMut' vs '&T' / '&mut T' (converting from 'RefMut' to '&mut T' (without using unsafe)?)
//  Ongoing: 2023-01-29T22:59:51AEDT 'Node::new()' returning 'Rc<RefCell<Self>>' instead of 'Self'(?)
//  Ongoing: 2023-01-30T20:28:01AEDT behaviour of chained 'Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem' when 'try_unwrap' fails? (are we using it in such a way it won't fail?)
//  Ongoing: 2023-01-30T20:41:47AEDT List/Node/Link should be in a namespace(?)
//  Ongoing: 2023-01-30T21:08:50AEDT (is Rust smart enough to) drop an iterator after we modify the container in question?
//  Ongoing: 2023-01-30T21:24:25AEDT (unpack the statement) 'interior mutability is great for safe applications, not so much for safe libraries'
//  }}}
use std::rc::Rc;
use std::cell::{RefCell,Ref,RefMut};

//  Continue: 2023-01-29T22:51:01AEDT 'inherited mutability' (see link)
//  Continue: 2023-01-29T23:47:55AEDT Reason/need for implementing 'Drop', behaviour if we don't, (detecting this behaviour?) [...] (test for drop?)
//  Continue: 2023-01-30T21:28:50AEDT implementing Iter / IterMut (for a Rc<RefCell> based container)


//  Fourth exercise: Bad Safe Deque
//  <(*not* using Weak for 'prev'?)>
//  <(an example of a safe yet bad design?)>

//  Doubly linked list invariant: each node should have exactly 2 pointers to it


//  Inherited mutability:
//  <()>


//  Interior mutability should be used as a last resort:
//          Introducing inherited mutability
//          Implementation details of logically-immutable methods
//          Mutating implementations of 'Clone'
//  LINK: https://ricardomartins.cc/2016/06/08/interior-mutability
//  {{{
//  Key takeaways:
//      Interior mutability is when you have an immutable reference '&T', but its value can be mutated
//      Sometimes mutable fields are needed inside immutable data structures
//      Cell<T> / RefCell<T> provide interior mutability (in single threaded environments)
//      Cell wraps copy values and doesn't have borrow checking
//      RefCell wraps any kind of value, and performs borrow checking at runtime
//      

//  <>

//  }}}


//  RefCell enforces exclusivity of mutable access at runtime (instead of compile-time) 
//  (use 'Mutex<T>' for <shared/interior> mutability in multi-threaded situations)
//  To access a shared/mutable reference to its contents:
//      fn borrow(&self) -> Ref<'_, T>
//      fn borrow_mut(&self) -> RefMut<'_, T>
//  Ref / RefMut implement Deref / DerefMut, allowing them to behave like &T / &mut T is most cases
//  To consume the RefCell and return its contents:
//      fn into_inner(self) -> T

//  Problem with RefCell:
//  RefCell is not always an implementation detail that can be abstracted away.
//  The lifetime of the returned reference is connected to returned Ref/RefMut, not the value in RefCell.
//  This means Ref/RefMut must live as long as we need the reference to the value in RefCell.


//  Rc<RefCell<T>> is a Rust idiom:
//  Rc provides reference counting, but only allows shared references to its contents.
//  Rc<RefCell<T>> is a way around this restriction.


//  'Result::ok(self) -> Option<T>'
//  Converts Result<T,E> into Option<T> (consumes result, discarding error (if any))
//  ('Result::unwrap()' requires the type in question implements 'Debug', 'result.ok().unwrap()' provides a workaround)


//  'Ref::map()'
//  Make a new Ref for a component of the borrowed data
//  (Allows us to map over Ref like we can map over an Option)>
//  Declaration:
//          fn map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
//              where F: FnOnce(&T) -> &U,
//                    U: ?Sized
//  To get Ref<T> from Ref<Node<T>>:
//          'self.head.as_ref().map(|x| { Ref::map(x.borrow(), |y| &y.elem) })'


//  'Ref::map_split()'
//  Splits a Ref into multiple Refs for different components of the borrowed data
//  Declaration:
//          fn map_split<U, V, F>(orig: Ref<'b, T>, f: F) -> (Ref<'b, U>, Ref<'b, V>)
//              where F: FnOnce(&T) -> (&U, &V),
//                    U: ?Sized,
//                    V: ?Sized,


type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}
pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
                Node { elem, next: None, prev: None, }
        ))
    }
}
impl<T> List<T> {

    pub fn new() -> Self {
        List { head: None, tail: None, }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    //  Implementing 'peek()':
    //
    //  invalid option: cannot get '&T' from RefCell (which returns 'Ref<T>')
    //pub fn peek_front_invalid(&self) -> Option<&T> {
    //    self.head.as_ref().map(|x| {
    //        &x.elem
    //    })
    //}
    //
    //  bad option: make &T available as 'Ref<Node<T>>'
    //  (requires 'Node' to be a public type')
    pub fn peek_front_bad(&self) -> Option<Ref<Node<T>>> {
        self.head.as_ref().map(|x| {
            x.borrow()
        })
    }
    //
    //  solution:
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|x| {
            //x.borrow().map(|y| &y.elem)
            Ref::map(x.borrow(), |y| &y.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |x| &x.elem)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |x| &mut x.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |x| &mut x.elem)
        })
    }

}


//  Rc cannot handle cycles. It will keep our doubly-linked list alive so long as nodes refer to each other
//  <((demonstrate?) need for dtor?)>
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}


pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}


//  Implementing Iter / IterMut: exercise skips them as possible-but-too-hard
//  <(Iterators that do not consume the list: Iter / IterMut, provide challenges (when working with Rc<RefCell>) that IntoIter does not)>
//  <(Iterator invalidation: <>)>
//  Interior mutability is great for safe applications, not so much for safe libraries.
//  {{{
//pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);
//impl<T> List<T> {
//    pub fn iter(&self) -> Iter<T> {
//        Iter(self.head.as_ref().map(|head| head.borrow()))
//    }
//}
//impl<'a, T> Iterator for Iter<'a, T> {
//    type Item = Ref<'a, T>;
//    //  invalid: returned reference only lives as long as 'node_ref'
//    //fn next(&mut self) -> Option<Self::Item> {
//    //    self.0.take().map(|node_ref| {
//    //        self.0 = node_ref.next.as_ref().map(|x| x.borrow());
//    //        Ref::map(node_ref, |x| &x.elem)
//    //    })
//    //}
//    //
//    //  invalid:
//    //fn next(&mut self) -> Option<Self::Item> {
//    //    self.0.take().map(|node_ref| {
//    //        let (next, elem) = Ref::map_split(node_ref, |x| {
//    //            (&x.next, &x.elem)
//    //        });
//    //        self.0 = if next.is_some() {
//    //            Some(Ref::map(next, |x| &**x.as_ref().unwrap()))
//    //        } else {
//    //            None
//    //        };
//    //        elem
//    //    })
//    //}
//}
//  }}}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_and_pop_front() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        list.push_front(1); list.push_front(2); list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test] 
    fn push_and_pop_back() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);
        list.push_back(1); list.push_back(2); list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        list.push_back(4); list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        list.push_front(1); list.push_front(2); list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    //  test_iter() / test_iter_mut()
    //  {{{
    //#[test]
    //fn test_iter() {
    //}
    //#[test] 
    //fn test_iter_mut() {
    //}
    //  }}}

}

