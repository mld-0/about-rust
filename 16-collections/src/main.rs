//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-11-25T23:34:39AEDT LinkedLists are (not cache friendly) (usually bad) - what useful purpouse(s) for which is it being provided as a library container? (what do the provided methods say about the <properties> of a linked list?)
//  Ongoing: 2022-11-26T00:00:56AEDT BinaryHeap (queue) always efficient to find/remove max value - (instead of doing trick of making everything negative), can we configure it to use min value instead? - (BinaryHeap requires and ordered type - does every ordered type have an equivalent of making values negative(?))
//  Ongoing: 2022-11-26T00:21:01AEDT Hash[Map|Set] vs BTree[Hash|Set] - what does it even mean for our keys to be sorted?
//  Ongoing: 2022-11-26T01:24:30AEDT meaning of 'Vec<&str, Global>' -> type of vec! with string literals ... (this is the Ycm GetType even if we manually give type as 'Vec<&str>') ... (something to do with reference lifetime(?) (don't lifetimes come first?)) [...] vector of Strings / i32s both also have 'Global' (is it something to do with the use of a macro(?)) [...] (also present for collect()-ed vector) [...] (not to do with mutability either) [...] (what is it even called - search for 'rust global' yields discussion of global variables) ... (introspection 'get_type_of()' function yields 'alloc::vec::Vec<i32>')
//  Ongoing: 2022-11-26T01:33:22AEDT collecting into a specific type vector is '(0..9).collect::<Vec<i32>>()' ((and) not '(0..9).collect::Vec::<i32>>()' or '(0..9).collect::Vec::i32()')
//  Ongoing: 2022-11-26T01:48:31AEDT HashSet::from() doesn't work with 'vec_of_strings![]' [...] 'vec!["".to_string()]' doesn't work either [...] (same error for BTreeSet) [...] (clarifying type of HashSet not helpful) [...] (neither is calling '.iter()'/'.into_iter()' on that macro-vector) [...] (figure out how to convert Vec -> HashSet (and work backwards from there)?)
//  Ongoing: 2022-11-26T01:56:59AEDT (is) '&str' (whatabout 'str') is <always?> a fat pointer - <(a pointer to a string literal, and a length of that string)> 
//  Ongoing: 2022-11-26T02:30:08AEDT assigning from (moving out of) a vector by assignment requires the type be (is it) Clone-able / Copy-able(?) [...] (can't be 'Copy' without also being 'Clone' (but Clone is the deep-copy/customizable one?) (this is true for deriving traits - is it true generally?))
//  Ongoing: 2022-11-26T02:36:55AEDT (book: use '.to_vec()' to get a copy (requires Clone)) do we get '.to_vec()' for free when we implement Clone? [...] (or is '.to_vec()' provided by Slice?)
//  Ongoing: 2022-11-26T02:48:32AEDT 'get(index) -> Option<&T>' is a lie - (the truth is one of those far-more-horrifying type of things) (doesn't cover range results) [...] (clarify, when a range is returned, it is as a reference(?)) [...] that also makes 'get_mut(index) -> Option<&mut T>' a lie ... are first/last -> Option<&T> correct (if get is a lie)(?)
//  Ongoing: 2022-11-26T02:56:12AEDT panic-ing while halfway through iterating over a vector by-value (moving elements out of it) ... (vector is deamed to be consumed as soon as we <start/finish> iterating over it(?))
//  Ongoing: 2022-11-26T03:00:13AEDT '.iter()' vs 'into_iter()'
//  }}}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

use std::collections::{HashSet,BTreeSet,HashMap,BTreeMap};

//  Rust uses Option where another language would use null.

//  Rust's compile-time borrow checker prevents invalidation errors - when a pointer to something inside the container is made invalid because the container is resized.

fn overview()
{
    //  Standard collections                Description                 C++ Equivalent          Python
    //      Vec<T>                          Growable array              vector                  list
    //      VecDeque<T>                     Double-ended queue          deque                   c.deque
    //      LinkedList<T>                   Doubly-linked list          list                    
    //      BinaryHeap<T: Ord>              Max heap                    priority_queue          heapq
    //      HashMap<K: Eq + Hash, V>        Dictionary                  unordered_map           dict
    //      BTreeMap<K: Ord, V>             <(Sorted Dictionary)>       map
    //      HashSet<T: Eq + Hash>           Hash table <(Set)>          unordered_set           set
    //      BTreeSet<T: Ord>                <(Sorted)> Set              set

    //  Book, the most common containers: Vec<T>, HashMap<K,V>, HashSet<T>
    //  (the rest having niche uses)

    //  Vec<T>
    //  Vector, a growable, heap-allocated array of T values
    //  (Can be considered the default container / a better alternative to the primitive array)

    //  VecDeque<T>
    //  Vector, optimised for insertion/removal from the ends

    //  LinkedList<T>
    //  Fast access to front/back, fast <insertion/>concatenation
    //  (Generally slower than Vec/VecDeque)

    //  BinaryHeap<T>
    //  Priority Queue, always efficient to find/remove the max value
    //  <(niche use - getting max 'n' values faster than the entire list can be sorted)> <(which leetcode problem demonstrated this trick?)>
    //  <(min value instead? (where we can't use make everything negative))>

    //  HashMap<K,V>
    //  Key-value pairs, with fast lookup by key.
    //  (Key must be hashable)

    //  BTreeMap<K,V>
    //  Like HashMap, but keep entries sorted by key.
    //  (Slower than HashMap)

    //  HashSet<T>
    //  Set of T values. Fast to add/remove values, and to check membership.

    //  BTreeSet<T>
    //  Like BTreeSet, but keep elements sorted.
    //  (Slower than HashSet)

    println!("overview, DONE");
}

fn example_Vec()
{
    //  Vector is a structure containing a length, a capacity, and a pointer to heap allocated storage
    //  (No heap storage is allocated when the vector is empty)

    //  Create using the 'vec!' macro
    let mut v1: Vec<i32> = vec![1,2,3,4,5];

    //  Type can be inferred
    let mut v2 = vec!["abc".to_string(), "def".to_string(), "hij".to_string()];

    //  Vec implements 'std::iter::FromIter', allowing us to use 'collect()' to create a vector from any iterator
    let mut v3: Vec<i32> = (0..=9).collect();
    //  or
    let mut v3 = (0..=9).collect::<Vec<i32>>();

    let s1 = HashSet::from(["abc".to_string(), "def".to_string(), "hij".to_string()]);
    //let s1: HashSet<String> = HashSet::from(vec_of_strings!["abc", "def", "hij"]);

    //  Convert another container (type supporting '.into_iter()') into a vector
    let mut v4 = s1.into_iter().collect::<Vec<String>>();

    //  Each element of 'Vec<&str>' contains a string length and pointer to string literal


    fn accessing_elements() 
    {
        //  Vectors will panic if given an out-of-bounds index
        //  Vector lengths/indices must be usize
        //  <(cannot assign-from (move out of) vector of non-Clone type - cannot move out of elements)>

        let lines: Vec<String> = vec_of_strings!["abc", "def", "hijk", "lmno", "pqrs", "tuv", "wxyz"];
        let numbers = (0..=9).collect::<Vec<i32>>();

        //  Reference to an element
        let first_line = &lines[0];

        #[derive(Clone,Copy)]
        struct Imaginary { im: f64, re: f64, };

        //  Assigning from a vector 
        //  (only possible for Clone types)
        let fifth_number = numbers[4];          //  requires Clone
        let second_line = lines[1].clone();     //  requires Clone

        //  Reference to a slice
        let r = &numbers[2..5];

        //  Copy of a slice (as Vector)
        let v = numbers[2..5].to_vec();


        //  Vec<T> has a lot of methods, and implements a lot of traits (see below)
        //  LINK: https://doc.rust-lang.org/std/vec/struct.Vec.html

        //  first() -> Option<&T>
        //  last() -> Option<&T>
        //      Get reference to first/last element
        //
        //  get(index) -> Option<&T>
        //      Get reference to element by index, or <(reference to?)> subslice by range
        //      Returns None if index/range out-of-bounds
        //
        //  first_mut() -> Option<&mut T>
        //  last_mut() -> Option<&mut T>
        //  get_mut(index) -> Option<&mut T>
        //      Like first/last/get, but returns a mutable reference
        //
        //  to_vec() -> Vec<T>
        //      Clone a slice, returning it as a vector (T must implement Clone)

    }
    accessing_elements();

    fn iteration()
    {
        //  Vectors and slices are both iterable by-ref / by-value
        //  <(see: item?)>
        //  <(actually: are slices iterable by-value?)>

        //  Iterating over 'Vec<T>' produces 'T'
        //  <(The vector is consumed)>

        //  Iterating over '&[T;N]' / '&[T]' / '&Vec<T>' produces '&T'

        //  Iterating over '&mut [T;N]' / '&mut [T]' / '&mut Vec<T>' produces '&mut T'

        //  Arrays, slices, and vectors all implement '.iter()' / '.iter_mut()'

        //  <('.iter()' vs '.into_iter()'?)>
    }
    iteration();

    fn growing_and_shrinking()
    {
    }
    growing_and_shrinking();

    fn joining()
    {
    }
    joining();

    fn splitting()
    {
    }
    splitting();

    fn swapping()
    {
    }
    swapping();

    fn sorting_and_searching()
    {
    }
    sorting_and_searching();

    fn comparing_slices()
    {
    }
    comparing_slices();

    fn random_elements()
    {
    }
    random_elements();

    fn invalidation_errors()
    {
    }
    invalidation_errors();

    println!("example_Vec, DONE");
}


fn example_VecDeque()
{
    println!("example_VecDeque, DONE");
}


fn example_LinkedList()
{
    println!("example_LinkedList, DONE");
}


fn example_BinaryHeap()
{
    println!("example_BinaryHeap, DONE");
}


fn example_HashMap_and_BTreeMap()
{
    println!("example_HashMap_and_BTreeMap, DONE");
}


fn example_HashSet_and_BTreeSet()
{
    println!("example_HashSet_and_BTreeSet, DONE");
}


fn example_Hashing()
{
    println!("example_Hashing, DONE");
}


fn example_CustomContainers()
{
    println!("example_CustomContainers, DONE");
}


fn main() 
{
    overview();
    example_Vec();
    example_VecDeque();
    example_LinkedList();
    example_HashMap_and_BTreeMap();
    example_Hashing();
    example_CustomContainers();
}

