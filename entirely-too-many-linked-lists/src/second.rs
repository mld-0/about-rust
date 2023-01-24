//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-24T21:16:51AEDT 'impl<T> List<T>' or 'impl<T> Drop for List<T>' -> significance of first and second T (case where either can be omitted?)
//  Ongoing: 2023-01-24T21:18:29AEDT updating 'first.rs' to use a parameterised type -> really just a matter of adding '<T>' everwhere required
//  Ongoing: 2023-01-24T21:26:14AEDT 'Link' / 'List' / 'Node' (the correct order to place their declarations (definitions?) in?)
//  Ongoing: 2023-01-24T21:30:43AEDT Equality between Some(&3) and 'list.peek_mut()' -> These are equal (but <presumedly> references to *different* 3s?)
//  Ongoing: 2023-01-24T21:34:47AEDT 'list.peek_mut().map(|&mut x| { x = 42 })' being invalid (because type is deduced as '&mut (&mut T)'?) - (this is <somewhat> <more> <like> C++ type deduction rules(?)) 
//  Ongoing: 2023-01-24T21:41:16AEDT (was there an item on lambdas) (obviously it wasn't enough) [...] (are they even called lambdas?)
//  Ongoing: 2023-01-24T21:51:37AEDT how up-to-date is this article? (weight to put on claims like 'no yield statement *yet*'?)
//  Ongoing: 2023-01-24T22:28:55AEDT ((are) asserts best practice on writing tests?)
//  Ongoing: 2023-01-24T22:38:54AEDT lifetime elision first equivalents example -> 'fn foo(&A) -> &B' (always) implies A/B have the same lifetime (or does implementation come into this?) [...] (is 'only 1 reference in input, output lifetime must be derived from input lifetime' an actual rule?) [...] (what about 'many inputs, 1 return value' case?)
//  Ongoing: 2023-01-24T22:41:47AEDT would lifetime parameters be so annoying if their syntax (specifically the single quote) weren't so annoying?
//  Ongoing: 2023-01-24T22:45:59AEDT lifetime parameters item
//  Ongoing: 2023-01-24T23:08:09AEDT 'iter3()' (lifetime elision example) can replace 2nd /'a/ with _ but not first?
//  Ongoing: 2023-01-24T23:29:08AEDT claim: we learned 'Iterator::<Iter>::next()' is incorrect when writing 'Iterator::<IterMut>::next()' (former is only valid where T is a copy type?) - (but, final code for second lesson does not address this) ... (does 'iter()' work for non-Copy T types?) [...] (or, are they saying Option<&T> is copy even if T isn't?)
//  Ongoing: 2023-01-24T23:32:33AEDT (how to) designate a custom type as non-copy?
//  Ongoing: 2023-01-24T23:39:22AEDT claim, same technique (consuming elements with '.take()') can be used to generate a safe IterMut for a tree
//  Ongoing: 2023-01-25T00:00:30AEDT Rust function/macro 'is_type_copy(T)' / 'is_type_copy<T>()'
//  }}}
use std::mem;

//  Continue: 2023-01-24T03:14:05AEDT 'option.take().map(|x| f(x))' is an idiom(?) (only execute 'f(x)' (on 'option's value if option is not none?) [...] (the idiom is caling '.map()' on None?)
//  Continue: 2023-01-24T23:00:24AEDT when is it necessary to provide lifetime parameters?


//  Second Exercise: <(Beginning with copy of first)>

//  'option.take()' is equivalent to 'mem::replace(&mut option, None)' 

//  <((what is the name of the) idiom? using '.map()' to consume Some(T) and ignore None, eg: 'option.take().map(|x| f(x))' / 'list.peek_mut().map(|x| { *x = 42 })' )>
//  <('.map()' does nothing when given None as input iterator?)>
//  {{{
//
//        //match self.head.take() {
//        //    None => {
//        //        None
//        //    },
//        //    Some(node) => {
//        //        self.head = node.next;
//        //        Some(node.elem)
//        //    }
//        //}
//        //  <(replacement for?)>
//        self.head.take().map(|node| {
//            self.head = node.next;
//            node.elem
//        })
//
//  }}}

//  turbofish: '::<SomeType>'

//  If '&T' is copy, then so is 'Option<&T>'


//  Replace 'enum Link { Empty, More(Box<Node>), }' with 'Option<Box<Node>>'
type Link<T> = Option<Box<Node<T>>>; 
pub struct List<T> {
    head: Link<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { 
            head: None,
        }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem, 
            next: self.head.take(),
        });
        self.head = Some(new_node)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        //  Use '.as_ref()' to borrow a reference from a value inside Option
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        //  '.as_mut()' is the mutable equivalent to '.as_ref()'
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take(); 
        }
    }
}


//  std::iter::Iterator: relates iterators with the type they produce
//  The iterator continues so long as 'next()' returns Some(Item)
pub trait Eg_Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
//  <(article claim: Rust has nothing like 'yield' *yet*)>


//  A collection should endevour to implement 'Iterator' for:
//          IntoIter        into_iter()     T
//          Iter            iter()          &T
//          IterMut         iter_mut()      &mut T


//  Lesson: IntoIter
pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


//  Lesson: Iter
//  Lifetimes are a block/scope (region) for which something is valid.
//  'lifetime elision' is where the compiler determines lifetimes of variables.

//  <(when it is beneficially/necessary to provide lifetime parameters?)>

//  Equivalent: <(only 1 reference in input, output lifetime must be derived from input lifetime)>
//      fn foo(&A) -> &B;
//      fn foo<'a>(&'a A) -> &'a B;

//  Equivalent: <(many inputs, must be independent)>
//      fn foo(&A, &B, &C)
//      fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

//  Equivalent: <(output lifetime must be derived from 'self')>
//      fn foo(&self, &B, &C) -> D;
//      fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

//  fn foo<'a>(&'a A) -> &'a B
//  means output must live at least as long as input

//  '.as_deref()': Converts Option<T> to Option<&T::Target>

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> List<T> {

    //  'iter()' with explicit lifetimes:
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        //Iter { next: self.head.as_ref().map(|x| &**x) }
        Iter { next: self.head.as_deref(), }
    }

    //  'iter()' is eligable for lifetime elision:
    pub fn iter2(&self) -> Iter<T> {
        Iter { next: self.head.as_deref(), }
    }
    pub fn iter3(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref(), }
    }

}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|x| {
            //self.next = x.next.map(|y| &mut **y);                         //  invalid <(need to implement Deref for Node<T>?)>
            //self.next = x.next.as_ref().map::<&Node<T>, _>(|y| &y);       //  valid <(specifying the type lets the compiler know it needs to perform a deref coercion)>
            self.next = x.next.as_deref();
            &x.elem
        })
    }
}


//  Lesson: IterMut
//  Certain techniques are used to implement 'iter_mut()' without resorting to 'unsafe' 
//  (here we discard elements from the list as we return them with 'take()')

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut(), }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        //  call '.take()' so we have exclusive access to the mutable reference
        self.next.take().map(|x| {
            self.next = x.next.as_deref_mut();
            &mut x.elem
        })
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
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(vec![1]);
        list.push(vec![2]);
        list.push(vec![3]);
        assert_eq!(list.pop(), Some(vec![3]));
        assert_eq!(list.pop(), Some(vec![2]));
        list.push(vec![4]);
        list.push(vec![5]);
        assert_eq!(list.pop(), Some(vec![5]));
        assert_eq!(list.pop(), Some(vec![4]));
        assert_eq!(list.pop(), Some(vec![1]));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        //  <(invalid: deduced as '&mut (&mut T)')>
        //list.peek_mut().map(|&mut x| { x = 42 });

        //  <(our alternative)>
        //*list.peek_mut().unwrap() = 42;

        //  <((spell out the) rule that says |x| gets deduced as &mut T(?))>
        list.peek_mut().map(|x| { *x = 42 });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));

        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(vec![1]); list.push(vec![2]); list.push(vec![3]);
        assert_eq!(list.peek(), Some(&vec![3]));
        assert_eq!(list.peek_mut(), Some(&mut vec![3]));
        list.peek_mut().map(|x| { *x = vec![42] });
        assert_eq!(list.peek(), Some(&vec![42]));
        assert_eq!(list.peek_mut(), Some(&mut vec![42]));

    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(vec![1]); list.push(vec![2]); list.push(vec![3]);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(vec![3]));
        assert_eq!(iter.next(), Some(vec![2]));
        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), None);
        let mut list = List::new();
        list.push(vec![1]); list.push(vec![2]); list.push(vec![3]);
        let vals = vec![vec![3],vec![2],vec![1]];
        for (x,check) in list.into_iter().zip(vals.into_iter()) {
            assert_eq!(x, check);
        }
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let vals = vec![3,2,1];
        for (x,check) in list.into_iter().zip(vals.into_iter()) {
            assert_eq!(x, check);
        }
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        let mut list = List::new();
        list.push(vec![1]); list.push(vec![2]); list.push(vec![3]);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&vec![3]));
        assert_eq!(iter.next(), Some(&vec![2]));
        assert_eq!(iter.next(), Some(&vec![1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        let mut list = List::new();
        list.push(vec![1]); list.push(vec![2]); list.push(vec![3]);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut vec![3]));
        assert_eq!(iter.next(), Some(&mut vec![2]));
        assert_eq!(iter.next(), Some(&mut vec![1]));
        assert_eq!(iter.next(), None);
    }

}

