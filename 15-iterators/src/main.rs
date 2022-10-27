//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-27T22:53:58AEDT equality not supported in Where clauses (book does so with its given definition of IntoIterator)
//  Ongoing: 2022-10-27T23:25:29AEDT 'iter()' vs 'to_iter()'? <('iter' iterator returns references, 'to_iter' iterator returns values?)> (calling 'into_iter' on a value produces values, calling it on a reference '(&v).into_iter()' produces references) [...] (equivalent 'v.iter()' / '(&v).into_iter()'?)
//  Ongoing: 2022-10-27T23:34:35AEDT how inefficent is it for a vector to drop values sequentially (are we not shuffling the vector at each stage?)
//  Ongoing: 2022-10-27T23:39:29AEDT assert eq against 'Some(OsStr::new("C:")))' is also correct for 'Some(OsStr::new(&"C:")))' [...] (difference between "abc" and &"abc"?)
//  Ongoing: 2022-10-27T23:54:18AEDT (how clever is the) default implementation of 'iter()' in terms of 'into_iter()'?
//  Ongoing: 2022-10-27T23:59:48AEDT 'print!()' (using std::fmt::Debug) into String (for 'dump_iterator')
//  Ongoing: 2022-10-28T00:05:51AEDT given an iterator that has taken ownership of a container by-val (can we get the container back?) (what about after iterating over container by-val?)
//  }}}

//  Half open range:        1..n        [1,n)
//  Inclusive range:        1..=n       [1,n]

//  Iterator related methods:
//      fold(self, init, f)             apply 'f' to each element and running total

//  Traits:
//      std::iter::Iterator         implemented by iterator type
//      std::iter::IntoIterator     implemented by containers that have iterator types

fn example_iterator_intro()
{
    //  An iterator is a value that produces a sequence of values, typically to be looped over
    //  Rust's standard library provides iterators for each container, as well as for input streams.

    fn triangle_i(n: i32) -> i32 {
        let mut result = 0;
        for i in 1..=n { result += i; }
        result
    }
    assert_eq!(28, triangle_i(7));

    //  '1..n+1' is a Range<i32>, producing values [1, n+1)
    //  'fold(init, f)' applies a closure 'f' to each element and the running total
    fn triangle_fold(n: i32) -> i32 {
        (1..=n).fold(0, |sum, item| sum + item)
    }
    assert_eq!(28, triangle_fold(7));

    println!("example_iterator_intro, DONE");
}

fn example_traits_Iterator_IntoIterator()
{
    //  An iterator is any value that implements 'std::iter::Iterator'
    trait Eg_Iterator {
        type Item;
        //  returns either (next value) Some(v), or None to indicate end of sequence
        fn next(&mut self) -> Option<Self::Item>;
        //  <(many) default methods>
    }

    //  A type which can be iterated over should implement 'std::iter::IntoIterator'
    trait Eg_IntoIterator 
    //    where Self::IntoIter::Item == Self::Item 
    {
        type Item;                  //  type of value produced by iterator
        type IntoIter: Iterator;    //  type of iterator
        fn into_iter(self) -> Self::IntoIter;
    }

    //  Using an iterator:
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    //  Equivalent:
    let mut s = String::new();
    for e in &v { s += e; s += ","; }
    println!("s=({})", s);
    //  or
    let mut s = String::new();
    let mut iter = (&v).into_iter();
    while let Some(e) = iter.next() { s += e; s += ","; }
    println!("s=({})", s);
    //  or
    let mut s = String::new();
    let mut iter = (&v).into_iter();
    for e in iter { s += e; s += ","; }
    println!("s=({})", s);

    //  for loops call 'to_iter()' on their operands <(then continuiously call 'next()')>

    //  <(Iterators automatically implement IntoIterator)>
    //  <(iterators provide 'to_iter()', which returns itself)>

    //  Terminology:
    //      iterator        any type that implements std::iter::Iterator
    //      iterable        any type that implements std::iter::IntoIterator
    //  An iterator produces values, which are items.
    //  The consumer is the code which recieves the items.

    println!("example_traits_Iterator_IntoIterator, DONE");
}

fn example_creating_iterators()
{
    //  Most collection types provide 'iter()' / 'iter_mut()' to return an iterator producing a reference to each item

    //  Manually using 'iter()' / 'next()'
    let v = vec![4,20,12,];
    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), None);
    assert!(v.len() > 0);
    let mut iter = v.into_iter();
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), Some(12));
    assert_eq!(iter.next(), None);
    //  Continue: 2022-10-27T23:33:37AEDT how to assert 'v' is dropped / 'iter' is empty? 
    //assert!(v.len() == 0);                //  error, 'v' has been dropped

    //  Iterating over a 'std::path::Path' produces the components of that path
    use std::ffi::OsStr;
    use std::path::Path;
    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iter = path.iter();
    assert_eq!(iter.next(), Some(OsStr::new("C:")));
    assert_eq!(iter.next(), Some(OsStr::new("Users")));
    assert_eq!(iter.next(), Some(OsStr::new("JimB")));
    assert_eq!(iter.next(), Some(OsStr::new("Downloads")));
    assert_eq!(iter.next(), Some(OsStr::new("Fedora.iso")));
    assert_eq!(iter.next(), None);

    //  Normally, use 'HashSet' (but insertion order is non-deterministic)
    //  <(BTreeSet has a deterministic insertion order)>
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string()); 
    favorites.insert("Liebesträume No. 3".to_string());
    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string())); 
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string())); 
    assert_eq!(it.next(), None);
    //assert!(favorites.len() == 0);        //  error, 'favorites' has been dropped

    //  collection.into_iter():
    //  Called on a shared reference, returns an iterator that produces shared references to its items
    //      for e in &v {}              for e in (&v).into_iter() {}
    //  Called on a mutable reference, returns an iterator that produces mutable references to its items
    //      for e in &mut v {}          for e in (&mut v).into_iter() {}
    //  Called on a value, returns an iterator that takes ownership of collection and returns items by value
    //      for e in v {}
    //  (not all types provide all 3 implementations)

    //  Mutable references should not be provided where they can be used to invalidate the container:
    //      HashSet, BTreeSet, BinaryHeap don't provide iterators of mutable references
    //      HashMap, BTreeMap provide iterators of mutable references for the values, but not the keys
    //      Slices [T] don't provide by-value iterators

    //  Equivalent:
    //      favorites.iter()
    //      (&favorites).into_iter()

    //  IntoIterator type bound
    use std::fmt::Debug;
    use std::iter::IntoIterator;
    fn dump_iterator<T,U>(t: T)
        where T: IntoIterator<Item=U>,
              U: Debug
    {
        for u in t { print!("{:?}, ", u); } 
        println!();
    }
    let v = vec![4,20,12,];
    //  (supports by-ref and by-val use)
    dump_iterator((&v).into_iter());
    dump_iterator(v.into_iter());

    ////  <(also valid)>
    //let v = vec![4,20,12,];
    //dump_iterator(&v);
    //dump_iterator(v);

    println!("example_creating_iterators, DONE");
}

fn example_drain_methods()
{
    println!("example_drain_methods, DONE");
}

fn example_other_iterator_sources()
{
    println!("example_other_iterator_sources, DONE");
}

fn example_iterator_adaptors()
{
    println!("example_iterator_adaptors, DONE");
}

fn example_consuming_iterators()
{
    println!("example_consuming_iterators, DONE");
}

fn example_custom_iterators()
{
    println!("example_custom_iterators, DONE");
}

fn main() 
{
    example_iterator_intro();
    example_traits_Iterator_IntoIterator();
    example_creating_iterators();
    example_drain_methods();
    example_other_iterator_sources();
    example_iterator_adaptors();
    example_consuming_iterators();
    example_custom_iterators();
}

