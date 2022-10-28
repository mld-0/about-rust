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
//  }}}
use std::iter::IntoIterator;
use std::iter::FromIterator;
use std::str::FromStr;
use std::collections::HashMap;

use std::iter::Peekable;
use std::iter::repeat;

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
//      std::iter::repeat(v)

//  Traits:
//      std::iter::Iterator         implemented by iterator type
//      std::iter::IntoIterator     implemented by containers that have iterator types
//      std::iter::FromIterator     <>

//  <(Most iterators are Sized)>

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
    //  map / filter / filter_map / flat_map / scan / take / take_while / skip / skip_while
    //  peekable / fuse / rev / next / next_back / 

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

    //  inspect(f)
    //  Used for debugging, applies closure 'f' to shared reference to each item, and passes it through
    let uc: String = "große".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(uc, "GROSSE");

    //  i1.chain(i2)
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

    //  enumerate()
    //  Attaches a running index to a sequence
    //  Given 'A,B,C' yields '(0,A),(1,B),(2,C)'
    //  (It returns a tuple, with the count being a usize)
    for (i,band) in bands.iter().enumerate() {
        println!("i=({}), band=({:?})", i, band);
    }

    //  i1.zip(i2)
    //  combines two iterators, producing tuples with an element from each
    //  (ends when shorter iterator ends)
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0,'A'),(1,'B'),(2,'C'),(3,'D')]);

    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(rhyme, vec![("going","once"), ("going","twice"), ("going","chicken soup with rice")]);

    let message = "To: jimb\r\nFrom: id\r\n\r\nOooooh, donuts!!\r\n";
    let mut lines = message.lines();

    //  by_ref()
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

    //  cloned()
    //  <>

    //  cycle(<>)
    //  <>

    println!("example_iterator_adaptors, DONE");
}

fn example_DoubleEndedIterator()
{
    //  std::iter::DoubleEndedIterator
    //  <>

    println!("example_DoubleEndedIterator, DONE");
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
    example_standard_library_iterators();
    example_iterator_adaptors();
    example_DoubleEndedIterator();
    example_consuming_iterators();
    example_custom_iterators();
}

