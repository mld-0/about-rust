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
//  Ongoing: 2022-10-28T08:48:03AEDT HashMap, no .items() (k,v) iterator?
//  Ongoing: 2022-10-28T11:16:57AEDT the 'major_cities' example is hideously ugly (is there / create a macro to create a vector of Strings (without such '.iter().map(|x| x.to_string()).collect::<Vec<String>>()' nonsense?)) [...] 'vec_of_strings' - a beautiful solution
//  Ongoing: 2022-10-29T01:06:57AEDT equality between Vector and array (in 'assert_eq' and outside it)
//  Ongoing: 2022-10-29T01:27:39AEDT a bigger challenge 'vec_of_strings![("going","once"), ("going","twice"), ("going","chicken soup with rice")]'
//  Ongoing: 2022-10-31T20:21:18AEDT 'cloned() takes an iterator that produces references' (it's not defined for an iterator returning values?)
//  Ongoing: 2022-10-31T20:24:47AEDT support for clone, (by-reference vs by-value (given 'cloned()' only supports iterators returning references))
//  Ongoing: 2022-10-31T20:48:50AEDT max/min require 'std::cmp::Ord' (not <'std::cmp::PartialOrd'>)(?) [...] (this is explicitly a decision to exclude floats, with their ambigiously ordered NaNs)
//  Ongoing: 2022-10-31T20:56:24AEDT 'max_by_key_by()' function?
//  Ongoing: 2022-10-31T23:43:59AEDT can rposition (reverse) a bytestring (but not a &str string)
//  Ongoing: 2022-10-31T23:45:44AEDT 'position' (example) closure is by-val, 'rposition' (example) closure is by-ref
//  Ongoing: 2022-11-01T00:02:06AEDT 'fold' example 'pangram' (works the same with closure <paramater/argument> 'w' passed by-val/by-ref) [...] (learn how String/str (and references) work)
//  Ongoing: 2022-11-01T00:55:28AEDT a single example function for each adaptor function(?)
//  Ongoing: 2022-11-01T01:02:24AEDT (are?) IntoIter / IntoIterable both (different) things?
//  Ongoing: 2022-11-01T01:03:46AEDT 'extend(&[...])' vs 'extend([...])'(?)
//  Ongoing: 2022-11-01T21:27:40AEDT (what rule says) iterating over a struct-literal requires that we place it in parentheses eg: 'for k in (Eg_I32Range { start: 0, end: 14 }) { }'
//  Ongoing: 2022-11-01T21:29:43AEDT our calculation of pi with a loop (how to do the same) (using our custom iterator) with iterator adaptors
//  Ongoing: 2022-11-01T22:08:44AEDT implementing 'iter()' for BinaryTree (we are doing so in a regular template-impl block (not by implementing a trait?) (what normally provides 'iter'?))
//  Ongoing: 2022-11-01T22:26:32AEDT implementing 'BinaryTree' iterator -> our 'iter()' / 'into_iter()' functions do the same thing(?) <('into_iter()' should be by-value? (which we have not implemented?))>
//  }}}

//  Continue: cleanup 'Iterator related methods' split into iterator-adapters / consuming-iterators / <other-categories> (and complete)
//  Continue: implementing an iterator, a more Rust-ian example (our example supports only iterating over references?)

use std::iter::IntoIterator;
use std::iter::DoubleEndedIterator;
use std::str::FromStr;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::iter::repeat;
use std::iter::once;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

//  Half open range:        1..n        [1,n)
//  Inclusive range:        1..=n       [1,n]

//  Iterator related methods:
//      fold(self, init, f)             apply 'f' to each element and running total
//      into_iter()                     returns an iterator returning references/values (as per type 'self')
//      iter()                          returns an iterator returning references
//      drain(range)                    returns an iterator returning values (does not consume container)
//      from_iter(iter)                 <(create container from sequence)>
//      map(f)                          Apply 'f' to each element
//      filter(p)                       returns an iterator only containing items for which 'p' is true
//      collect()                       return a container containing values from iterator
//      rev()                           reverse an iterator
//      skip(n)                         returns an iterator that skips the first 'n' elements
//      take(n)                         returns an iterator containing at most 'n' elements
//      take_while(p)                   returns an iterator containing values up until closure 'p' is false
//      filter_map(f)                   returns an iterator containing only values for which 'f' returns Some(value)
//      enumerate()                     returns an iterator which gives current iteration count as well as value
//      all(f)                          is 'f' true for all elements
//      any(f)                          is 'f' true for any element
//      find(f)                         return first element for which 'f' is true
//      copied()                        returns an iterator which returns copies of container values
//      cloned()                        returns an iterator which returns clones of container values
//      cycle()                         repeat an iterator endlessly
//      sum()                           returns sum of elements
//      product()                       returns product of elements
//      max() / min()                   max / min element of iterator
//      nth(n)                          nth element of iterator
//      reduce(f)                       Reduce elements to a single one by repeatedly applying 'f'
//      peekable()                      create an iterator which can use peek/peek_mut
//      position(f)                     return index of first value for which 'f' is true
//      rposition(f)                    starting from right, return index of first value for which 'f' is true
//      zip(u)                          combine two iterators
//      unzip()                         seperate iterators combined with 'zip'
//      scan(init,f)                    Like map, (but also accumulates like 'fold'), can terminate sequence early
//      std::iter::empty()              Returns None immediately
//      std::iter::once(v)              Produces 'v', then None
//      std::iter::repeat(v)            Repeat given value forever
//      <()>

//  Traits:
//      std::iter::Iterator         implemented by iterator type
//      std::iter::IntoIterator     implemented by containers that have iterator types
//      std::iter::FromIterator     implemented by containers that can be created from iterators
//      <()>

//  By implementing 'std::iter::Iterator', 'std::iter::IntoIterator' is automatically implemented

//  <(Most(?) iterators are Sized)>

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
    //  'drain()' takes a mutable reference to the collection and returns an iterator that yields elements by-value
    //  (unlike 'into_iter()' it does not consume the collection itself)

    //  On types that can be indexed by range, 'drain()' takes a range of elements to remove

    let mut v1 = vec![0,1,2,3,4,5,6,7,8,9];
    let v2 = Vec::from_iter(v1.drain(1..=4));
    assert_eq!(v1, vec![0,5,6,7,8,9,]);
    assert_eq!(v2, vec![1,2,3,4,]);

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    //  Use 'drain(..)' to drain whole sequence
    let mut v1 = Vec::<i32>::from_iter(0..=9);
    let v2 = Vec::from_iter(v1.drain(..));
    assert_eq!(v1, Vec::<i32>::new());
    assert_eq!(v2, vec![0,1,2,3,4,5,6,7,8,9]);

    println!("example_drain_methods, DONE");
}

fn example_standard_library_iterators()
{
    //  Iterators in standard library:
    //      std::ops::Range             1..10
    //      std::ops::RangeFrom         1..
    //      Option<T>                   Some(10).iter()
    //      Result<T,E>                 Ok("blah").iter()

    //      Vec<T>, &[T]                v.windows(16)               Every contiguous slice of the given length
    //                                  v.chunks(16)                Non-overlapping contiguous slices of given length
    //                                  v.chunks_mut(1024)          
    //                                  v.split(|b| b & 1 != 0)     Slices seperated by elements that match predicate
    //                                  v.split_mut(...)            
    //                                  v.rsplit(...)               split, R->L
    //                                  v.splitn(n, ...)            split, at most 'n' slices

    //      String, &str                s.bytes()                   utf8 bytes
    //                                  s.chars()                   chars (utf8 encoded)
    //                                  s.split_whitespace()        split by whitespace
    //                                  s.lines()                   split by newline
    //                                  s.split(pattern)            split by pattern
    //                                  s.matches(...)              slices matching given pattern

    //      HashMap                     map.keys()
    //      BTreeMap                    map.values()
    //                                  map.values_mut()

    //      HashSet                     set1.union(set2)
    //      BTreeSet                    set2.intersection(set2)

    //      std::sync::mpsc::Reciever   recv.iter()                 values send from another thread

    //      std::io::Read               stream.bytes()
    //                                  stream.chars()

    //      std::io::BufRead            bufstream.lines()           Parse stream as utf8, produces lines as Strings
    //                                  bufstream.split(n)          split stream on given byte, produces Vec<u8>

    //      std::fs::ReadDir            std::fs::read_dir(path)     
    //      std::new::TcpListener       listener.incoming()

    //      std::iter                   std::iter::empty()          returns None immediately
    //                                  std::iter::once(val)        produce given value, then None
    //                                  std::iter::repeat(val)      repeat given value forever

    println!("example_standard_library_iterators, DONE");
}

fn example_iterator_adaptors()
{
    //  <(organize/categorize?)>
    //  Method for creating one iterator from another
    //      map / filter / filter_map / flat_map / scan / take / take_while / skip / skip_while
    //      peekable / fuse / rev / next / next_back / inspect / chain / enumerate / zip 
    //      by_ref / cloned / cycle / 

    //  iterator adapters are zero overhead abstractions

    let text = " ponies \n giraffes\niguanas \nsquid".to_string();

    //  'map(f)' allows a closure to be applied to the items of an iterator
    //  'filter(f)' allows items to be removed from an iterator as per a closure
    let v1: Vec<&str> = text.lines().map(str::trim).collect();
    let v2: Vec<&str> = text.lines().map(str::trim).filter(|s| *s != "iguanas").collect();
    assert_eq!(v1, vec!["ponies", "giraffes", "iguanas", "squid"]);
    assert_eq!(v2, vec!["ponies", "giraffes", "squid"]);

    //  <(Definitions:)>
    //fn Eg_map<B,F>(self, f: F) -> Some(Iterator<Item=B>)
    //    where Self: Sized, F: FnMut(Self::Item) -> B;
    //fn Eg_filter<P>(self, predicate: P) -> Some(Iterator<Item=Self::Item>)
    //    where Self: Sized, P: FnMut(&Self::Item) -> bool;

    //  (iterators do not do any work unless some function is called which steps through them)
    //  (note: this expresison has no output)
    ["earth", "water", "air", "fire"].iter().map(|x| println!("x=({})", x));


    //  'filter_map(f)' acts as a combination of filter and map
    //  'f' returns value as Some(value), or None to indicate value should be excluded

    //  <(Definition:)>
    //fn filter_map<B,F>(self, f: F) -> Some(Iterator<Item=B>)
    //    where Self: Sized, F: FnMut(Self::Item) -> Option<B>

    //  Extract floats from string:
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    let v: Vec<f64> = text.split_whitespace().filter_map(|w| f64::from_str(w).ok()).collect();
    assert_eq!(v, vec![1.0,0.25,289.0,3.1415]);


    //  'flat_map(f)' allows 0, 1, or multiple values to be returned by 'f' for each item 
    //  (the resulting sequences are concatenated into a single iterator)

    //  <(Definition:)>
    //fn flat_map<U,F>(self, f: F) -> Some(Iterator<Item=U::Item>)
    //    where F: FnMut(Self::Item) -> U, 
    //          U: IntoIterator

    let mut major_cities = HashMap::<&str,Vec<String>>::new();
    //major_cities.insert("Japan", vec!["Tokyo".to_string(), "Kyoto".to_string()]);
    major_cities.insert("Japan", vec_of_strings!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec_of_strings!["Portland", "Nashville"]); 
    major_cities.insert("Brazil", vec_of_strings!["São Paulo", "Brasília"]); 
    major_cities.insert("Kenya", vec_of_strings!["Nairobi", "Mombasa"]); 
    major_cities.insert("The Netherlands", vec_of_strings!["Amsterdam", "Utrecht"]); 
    let countries = ["Japan", "Brazil", "Kenya"];
    let v = countries.iter().flat_map(|x| major_cities[x].clone()).collect::<Vec<String>>();
    assert_eq!(v, vec_of_strings!["Tokyo","Kyoto", "São Paulo", "Brasília", "Nairobi", "Mombasa"]);


    //  'scan(init, f)' 
    //  scan is like map, except is has a mutable initial value, can terminate the iterator early by returning None
    let v = (0..10).scan(0, |count,item| {
            *count += 1;
            if *count > 5 { None } else { Some(item * item) }
        }).collect::<Vec<i32>>();
    assert_eq!(v, vec![0,1,4,9,16]);

    //  <(String.lines() does not consume the String from which it is created)>


    //  'take(n)'
    //  Returns iterator with at most 'n' elements


    //  'take_while(p)'
    //  Returns iterator with elements until first p(item) == false

    let message = "To: jimb\r\n\
    From: superego <editor@oreilly.com>\r\n\
    \r\n\
    Did you get any writing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";
    let v = message.lines().take_while(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>();
    println!("v=({:?})", v);


    //  'skip(n)'
    //  Return iterator with 'n' items from beginning removed


    //  'skip_while(p)'
    //  Return iterator with items before p(item) == <true/false> removed

    //  <(Definition:)>
    //fn Eg_take(self, n: usize) -> some Iterator<Item=Self::Item> 
    //      where Self: Sized;
    //fn Eg_take_while<P>(self, predicate: P) -> some Iterator<Item=Self::Item> 
    //      where Self: Sized, 
    //            P: FnMut(&Self::Item) -> bool;

    let v = message.lines().skip_while(|x| !x.is_empty()).skip(1).map(|x| x.to_string()).collect::<Vec<String>>();
    println!("v=({:?})", v);

    //  A common use case for 'skip()' is to skip the command name when iterating over cli args
    for arg in std::env::args().skip(1) {
        println!("arg=({:?})", arg);
    }


    //  'peekable()'
    //  A peekable iterator lets us peek at the next item without consuming it. 
    //  Turn any iterator into a peekable iterator by calling 'peekable()'

    //  <(Definition:)>
    //fn Eg_peekable(self) -> std::iter::Peekable<Self>
    //      where Self: Sized;

    //  Example: extract continuous digits in a string
    fn parse_number<T>(tokens: &mut Peekable<T>) -> u32
        where T: Iterator<Item=char>
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n
            }
            tokens.next();
        }
    }
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    //  <(performance implications of making a stream peekable?)>


    //  'fuse()'
    //  Once an Iterator has returned None, the trait doesn't specify what it must do next time it is called.
    //  fuse returns an iterator that can only return None after first returning None

    struct Flaky(bool);
    impl Iterator for Flaky {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some(1)
            } else {
                self.0 = true;
                None
            }
        }
    }
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some(1));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some(1));
    let mut flaky = Flaky(true).fuse();
    assert_eq!(flaky.next(), Some(1));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), None);


    //  'inspect(f)'
    //  Used for debugging, applies closure 'f' to shared reference to each item, and passes it through
    let uc: String = "große".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(uc, "GROSSE");


    //  'i1.chain(i2)'
    //  appends on iterator to another

    //  <(Definition:)>
    //fn Eg_chain<U>(self, other: U) -> Some(Iterator<Item=Self::Item>>)
    //    where Self: Sized, 
    //          U: IntoIterator<Item=Self::Item>;

    let v: Vec<i32> = (1..=4).chain(vec![20,30,40]).collect();
    assert_eq!(v, [1,2,3,4,20,30,40]);

    //  A chain iterator is reverseable if both its underlying iterators are
    let v: Vec<i32> = (1..=4).chain(vec![20,30,40]).rev().collect();
    assert_eq!(v, [40,30,20,4,3,2,1]);

    let cols: usize = 5;
    let rows: usize = 5;
    let mut pixels: Vec<u8> = (0..(cols*rows)).map(|x| (x/10) as u8).collect();
    let threads = 2;
    let band_rows = rows / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(band_rows * cols).collect();


    //  'enumerate()'
    //  Attaches a running index to a sequence
    //  Given 'A,B,C' yields '(0,A),(1,B),(2,C)'
    //  (It returns a tuple, with the count being a usize)
    for (i,band) in bands.iter().enumerate() {
        println!("i=({}), band=({:?})", i, band);
    }


    //  'i1.zip(i2)'
    //  combines two iterators, producing tuples with an element from each
    //  (ends when shorter iterator ends)
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0,'A'),(1,'B'),(2,'C'),(3,'D')]);

    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(rhyme, vec![("going","once"), ("going","twice"), ("going","chicken soup with rice")]);

    let message = "To: jimb\r\nFrom: id\r\n\r\nOooooh, donuts!!\r\n";
    let mut lines = message.lines();


    //  'by_ref()'
    //  Borrows a mutable reference to the iterator
    //  Allows adaptors to be used on iterators without consuming them
    for h in lines.by_ref().take_while(|x| !x.is_empty()) {
        print!("{}, ", h);
    }
    println!();
    for h in lines {
        print!("{}, ", h);
    }
    println!();


    //  'cloned()'
    //  Returns an iterator that produces values cloned from the references recieved
    //  (not callable on iterator returning by-value)
    //  (Referent type must implement Clone)
    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));
    //assert_eq!(a.into_iter().cloned().next(), Some('1'));         //  error, 'cloned' not available for by-val iter


    //  'cycle()'
    //  Returns an iterator that endlessly repeats the sequence produced by the underlying iterator
    //  (Underlying iterator must implement Clone)
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizzes_buzzes)
        .map(|x| match x {
                 (i, ("", "")) => i.to_string(),
                 (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
            }).collect::<Vec<String>>();
    println!("fizz_buzz=({:?})", fizz_buzz);

    println!("example_iterator_adaptors, DONE");
}

fn example_DoubleEndedIterator()
{
    //  std::iter::DoubleEndedIterator
    //  An iterator that allows items to be taken from either end
    //  <(<some/most> standard library ordered collections are double ended)>

    //  <(Definition:)>
    trait Eg_DoubleEndedIterator: Iterator {
        fn next_back(&mut self) -> Option<Self::Item>;
    }

    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    //  'rev()'
    //  Reverse a double ended iterator

    //  <(Definition:)>
    //fn Eg_rev(self) -> Some(Iterator<Item=Self>) 
    //    where Self: Sized + DoubleEndedIterator;

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next_back(), Some(&"breakfast"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    //  many adaptors, (including 'map()' and 'filter()') preserve reversibility

    println!("example_DoubleEndedIterator, DONE");
}

fn example_consuming_iterators()
{
    use std::io::prelude::*;
    use std::cmp::{PartialOrd,Ordering};

    //  <(organize/categorize?)>
    //  Methods for consuming iterators
    //  (recall that iterators are not executed until they are consumed)
    //      count / sum / product / max / min / max_by / min_by / max_by_key / min_by_key 
    //      any / all / position / rposition / fold / nth last / find / partition
    //      lt / le / gt / ge / eq / ne / cmp / partial_cmp
    //      collect / from_iter / extend 

    //  'count()'
    //  Draws from iterator until it returns None, returning the number of non-None elements
    let c = (1..10).count();
    println!("c=({})", c);


    //  'sum()'
    //  Sum iterators items
    //  (implemented by 'std::iter::Sum')
    fn triangle(n: u64) -> u64 { 
        (1..=n).sum() 
    }
    assert_eq!(triangle(20), 210);


    //  'product()'
    //  Multiply iterators items
    //  (implemented by 'std::iter::Product)
    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
    assert_eq!(factorial(20), 2432902008176640000);


    //  'max()' / 'min()'
    //  Returns the max/min value produced by the iterator.
    //  (Iterator's underlying type must implement 'std::cmp::Ord')
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    //  Custom comparison function - will panic if given NaN
    //  <(use of references-to-references?)>
    fn Eg_cmp(l: &&f64, r: &&f64) -> Ordering { l.partial_cmp(r).unwrap() }


    //  'max_by(f)' / 'min_by(f)'
    //  Returns the max/min value produced by the iterator, as determied by comparison function 'f'
    let numbers = [1.0,4.0,2.0,];
    assert_eq!(numbers.iter().max_by(Eg_cmp), Some(&4.0));
    assert_eq!(numbers.iter().min_by(Eg_cmp), Some(&1.0));


    //  'max_by_key(f)' / 'min_by_key(f)'
    //  Returns max/min value produced by 

    //  <(Definition:)>
    //fn Eg_min_by_key<B: Ord, F>(self f: F) -> Option<Self::Item>
    //    where Self: Sized, 
    //          F: FnMut(&Self::Item) -> B;

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);
    assert_eq!(populations.iter().max_by_key(|&(_k,v)| v), Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_k,v)| v), Some((&"Greenhorn", &2)));


    //  'any(f)' / 'all(f)'
    //  Apply a closure to the iterator, and return whether it returns true for any / all items
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));


    //  'std::iter::ExactSizeIterator'
    //  <(an iterator with a known number of items)>
    //  <(Definition:)>
    pub trait Eg_ExactSizeIterator: Iterator {
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
    }


    //  'position(p)' / 'rposition(p)'
    //  Apply closure 'p' to each item, returning the index of the first item for which the result is True
    //  (rposition searches from the right (although result index is from the left))
    let t = "Xerxes";
    assert_eq!(t.chars().position(|x| x == 'e'), Some(1));
    assert_eq!(t.chars().position(|x| x == 'X'), Some(0));
    assert_eq!(t.chars().position(|x| x == 'z'), None);

    //  rposition requires a reversible and exactsize iterator
    let b = b"Xerxes";
    assert_eq!(b.iter().rposition(|&x| x == b'e'), Some(4));
    assert_eq!(b.iter().rposition(|&x| x == b'X'), Some(0));


    //  'fold(init, f)'
    //  Applys 'f' to 'init' and each item to produce a single final value
    let a = [5,6,7,8,9,10];
    assert_eq!(a.iter().fold(0, |x, _| x+1), 6);                                    //  count
    assert_eq!(a.iter().fold(0, |x, i| x+i), 45);                                   //  sum
    assert_eq!(a.iter().fold(1, |x, i| x*i), 151200);                               //  product
    assert_eq!(a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m,i)), 10);    //  max

    let a = ["Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs"];
    //  (Accumulators are moved into/out-of closure - so fold can be used with non-copy types)
    let pangram = a.iter().fold(String::new(), |mut x, &w| { x.push_str(w); x});
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs");

    //  <(Definition)>
    //fn Eg_fold<A,F>(self, init: A, f: F) -> A
    //    where Self: Sized,
    //          F: FnMut(A, Self::Item) -> A;


    //  'nth(n)'
    //  Returns the n-th element of the iterator (0-indexed) (returned item, and items before it, are consumed)
    //  (doesn't take ownership of iterator)
    //  ('.nth(0)' is equivalent to '.next()')
    let mut s = (0..=9).map(|x| x * x);
    assert_eq!(s.nth(4), Some(16));
    assert_eq!(s.nth(0), Some(25));
    assert_eq!(s.nth(6), None);
    assert_eq!(s.nth(0), None);

    //  <(Definition)>
    //fn Eg_nth(&mut self, n: usize) -> Option<Self::Item>
    //    where Self: Sized;


    //  'last()'
    //  Consumes items until iterator returns None, returning last non-None item
    //  (consumes all iterator items (even if iterator is reverseable))
    let mut s = (0..=9).map(|x| x * x);
    assert_eq!(s.last(), Some(81));

    //  (to get the last item of a reverseable iterator without consuming all items:)
    let mut s = (0..=9).map(|x| x * x);
    assert_eq!(s.rev().next(), Some(81));

    //  <(Definition:)>
    //fn Eg_last(self) -> Option<Self::Item>;



    //  'find()'
    //  Given a closure, return the first item for which the closure is true
    //  (consumes iterator items up to the and including the returned item)
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);
    assert_eq!(populations.iter().find(|&(_k, &v)| v > 1_000_000), None);
    assert_eq!(populations.iter().find(|&(_k, &v)| v > 500_000), Some((&"Portland", &583_776)));

    //  <(Definition:)>
    //fn Eg_find<P>(&mut self, predicate: P) -> Option<Self::Item>
    //    where Self: Sized,
    //          P: FnMut(&Self::Item) -> bool;


    //  'partition(f)'
    //  Divide the items of an iterator among two collections, according to a closure
    //  (Requires 'std::default::Default' and 'std::Default::Extend')
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    let (living, nonliving): (Vec<&str>, _) = things.iter().partition(|x| x.as_bytes()[0] & 1 != 0);
    assert_eq!(living, ["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, ["doorknob", "noodle"]);

    //  <(partition builds two containers, rather than two iterators, as splitting a single iterator into two violates Rust's rules about multiple mutable references to the same item)>

    //  <(Definition:)>
    //fn partition<B,F>(self, f: F) -> (B, B)
    //    where Self: Sized,
    //          B: Default + Extend<Self::Item>,
    //          F: FnMut(&Self::Item) -> bool;

    println!("example_consuming_iterators, DONE");
}

fn example_comparing_item_sequences()
{
    //  Use comparison operators ('<', '==', ect) to compare strings, vectors, and slices
    //  (assuming their indervidual elements support these operators)

    //  Iterators do not support these comparison operators, but they provide methods to do the same job:
    //  'lt()' / 'le()' / 'gt()' / 'ge()'
    //  'eq()' / 'ne()' 
    //  'cmp()' / 'partial_cmp()'

    let packed = "Helen of Troy";
    let spaced = "Helen    of    Troy";
    let obscure = "Helen of Sandusky";
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    //  <(Equivalent?)>
    assert!(spaced < obscure);
    assert!(spaced.chars().lt(obscure.chars()));

    println!("example_comparing_item_sequences");
}

fn example_buildingCollections_collect_FromIter()
{
    //  'collect()'
    //  (Builds a standard library collection from the items of an iterator)

    //  (must provide the type of the resulting container):
    let a: Vec<i64> = (0..=10).collect();
    let a = (0..=10).collect::<Vec<i64>>();

    //  <(use something other than 'std::env::args()')>
    //use std::collections::{HashSet,BTreeSet,LinkedList,HashMap,BTreeMap};
    //let args: HashSet<String> = std::env::args().collect();
    //let args: BTreeSet<String> = std::env::args().collect();
    //let args: LinkedList<String> = std::env::args().collect();
    //let args: HashMap<String,usize> = std::env::args().zip(0..).collect();
    //let args: BTreeMap<String,usize> = std::env::args().zip(0..).collect();

    //  Implemented by trait 'std::iter::FromIterator'
    //  <(Definition:)>
    trait Eg_FromIterator<A>: Sized {
        fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self;
    }
    //  <('collect()' (default version) is implemented in terms of 'from_iter()'?)>

    //  collect can use information like 'Iterator::size_hint()' to be more efficent
    //  (eg: preallocating vectors)>

    println!("example_buildingCollections_collect_FromIter");
}

fn example_Extend()
{
    //  'c.extend(v)'
    //  Add items from iterable 'v' to collection 'c'
    let mut v: Vec<i32> = (0..5).map(|x| 1 << x).collect();
    v.extend(&[31,57,99,163]);
    v.extend([3,6,9,7]);
    assert_eq!(v, [1,2,4,8,16,31,57,99,163,3,6,9,7]);

    //  Implemented by trait 'std::iter::Extend' 
    //  <(Definition:)>
    trait Eg_Extend<A> {
        fn extend<T>(&mut self, iter: T)
            where T: IntoIterator<Item=A>;
    }

    //  (many implementations of 'from_iter' are in terms of 'extend')

    println!("example_Extend");
}

fn example_custom_iterators()
{
    //  Iterator traits:
    //      std::iter::Iterator         implemented by iterator type
    //      std::iter::IntoIterator     implemented by containers that have iterator types
    //      std::iter::FromIterator     implemented by containers that can be created from iterators

    struct Eg_I32Range { start: i32, end: i32, }

    impl std::iter::Iterator for Eg_I32Range {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            if self.start >= self.end {
                None
            } else {
                let value = self.start;
                self.start += 1;
                Some(value)
            }
        }
    }

    //  <(By implementing 'Iterator', 'IntoIterator' is automatically implemented)>
    //  (that is, implementing 'Iterator' is all that is required to loop over our type)
    let mut iter = Eg_I32Range { start: 0, end: 14 };
    let mut pi = 0.0;
    let mut numerator = 1.0;
    for k in iter { 
        pi += numerator / (2*k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);
    //  (IEEE 754 specifies this result exactly)
    assert_eq!(pi as f32, std::f32::consts::PI);


    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }
    struct TreeNode<T> {
        val: T, 
        left: BinaryTree<T>, 
        right: BinaryTree<T>,
    }

    //  Constructing an iterator that traverses BinaryTree requires us to keep a stack of our tree traverse
    struct TreeIter<'a, T: 'a> {
        unvisited: Vec<&'a TreeNode<T>>
    }
    impl<'a, T: 'a> TreeIter<'a, T> {
        fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
            while let BinaryTree::NonEmpty(ref node) = *tree {
                self.unvisited.push(node);
                tree = &node.left;
            }
        }
    }
    impl<T> BinaryTree<T> {
        fn iter(&self) -> TreeIter<T> {
            let mut iter = TreeIter { unvisited: Vec::new() };
            iter.push_left_edge(self);
            iter
        }
    }
    //  Required to iterate over reference to container
    //  <(iterating over a container makes an implicit call to 'into_iter()')>
    impl<'a, T: 'a> std::iter::IntoIterator for &'a BinaryTree<T> {
        type Item = &'a T;
        type IntoIter = TreeIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    impl<'a, T> Iterator for TreeIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            let node = match self.unvisited.pop() {
                None => return None,
                Some(n) => n,
            };
            self.push_left_edge(&node.right);
            Some(&node.val)
        }
    }
    fn make_node<T>(val: T, left: BinaryTree<T>, right: BinaryTree<T>) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(TreeNode { val, left, right }))
    }

    let subtree_l = make_node("abc", BinaryTree::Empty, BinaryTree::Empty);
    let subtree_rl = make_node("hij", BinaryTree::Empty, BinaryTree::Empty);
    let subtree_r = make_node("klm", subtree_rl, BinaryTree::Empty);
    let tree = make_node("def", subtree_l, subtree_r);

    let mut v = Vec::new();
    for x in &tree { v.push(*x); }
    //  or
    //for x in tree.iter() { v.push(*x); }
    //for x in tree.into_iter() { v.push(*x); }
    println!("v=({:?})", v);
    assert_eq!(v, ["abc", "def", "hij", "klm"]);

    let v = tree.iter().map(|x| format!("mega-{}", x)).collect::<Vec<_>>();
    assert_eq!(v, ["mega-abc", "mega-def", "mega-hij", "mega-klm"]);

    println!("example_custom_iterators, DONE");
}

fn main() 
{
    example_iterator_intro();
    example_traits_Iterator_IntoIterator();
    example_creating_iterators();
    example_drain_methods();
    example_standard_library_iterators();
    example_iterator_adaptors();
    example_DoubleEndedIterator();
    example_consuming_iterators();
    example_comparing_item_sequences();
    example_buildingCollections_collect_FromIter();
    example_Extend();
    example_custom_iterators();
}

