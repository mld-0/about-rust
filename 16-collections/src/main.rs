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
//  Ongoing: 2022-11-26T21:44:34AEDT What about the Rust rules of references say we can't borrow '&mut v[0]' and '&mut v[1]' at the same time(?)
//  Ongoing: 2022-11-26T22:23:42AEDT Rust has such comprehensive 'split' functions because we can't borrow multiple references to multiple elements simultaneously (without them)
//  Ongoing: 2022-11-26T22:26:07AEDT why 'RSplitN' is needed (why 'rsplitn()' can't just return 'SplitN')
//  Ongoing: 2022-11-26T22:27:40AEDT what actually happens when we split a <slice/Vector> - book implies one buffer becomes <two/multiple>(?)
//  Ongoing: 2022-11-26T22:28:33AEDT how to do what 'split()' does, without omitting separator elements
//  Ongoing: 2022-11-26T22:31:40AEDT many function <signatures> presented here are <simplified>
//  Ongoing: 2022-11-26T23:12:49AEDT why comparing slices container comparisons use PartialEq / PartialOrd ... (a type that is Ord is also PartialOrd)
//  Ongoing: 2022-11-26T23:26:30AEDT book example for shuffle uses thread_rng().shuffle() (which for us is an error)
//  }}}

//  Continue: 2022-11-26T23:44:54AEDT ordered vs unordered set/map (use-cases)

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
        //  slice.len()
        //  slice.is_empty()

        //  Vectors grow as needed, however this incurs a penalty that can be eliminated by pre-allocating capacity
        //
        //  Vec::with_capacity(n)
        //      New vector with capacity 'n'
        //
        //  vec.capacity()
        //      Vector current capacity
        //
        //  vec.reserve(n)
        //      Increase Vector capacity for at least 'n' more elements
        //
        //  vec.reserve_exact(n)
        //      Increase Vector capacity for 'n' more elements
        //
        //  vec.shrink_to_fit()
        //      Decrease capacity to length of Vector
        //

        //  Adding/removing elements
        //  (Only 'resize' Clones values - all other functions move values)
        //
        //  vec.push(value)
        //      Add <(move)> value to end of Vector
        //
        //  vec.pop() -> Option<T>
        //      Remove and return the last element
        //
        //  vec.insert(index, value)
        //      Insert 'value' at 'vec[index]' (Slow for long vectors)
        //
        //  vec.remove(index)
        //      Remove and return 'vec[index]' (Slow for long Vectors)
        //
        //  vec.resize(new_len, value)
        //      Add copies of 'value' to Vector until length is 'new_len'
        //      ('value' must implement 'Clone')
        //
        //  vec.truncate(new_len)
        //      Reduce the length of the Vector to 'new_len', dropping any elements in 'vec[new_len..]'
        //
        //  vec.clear()
        //      Remove all elements
        //
        //  vec.extend(iterable)
        //      Push each element in iterable to Vector
        //      (provided by 'std::iter::Extend')
        //
        //  vec.split_off(index) -> Vec<T>
        //      Remove elements 'vec[index..]', returning them as a new Vector
        //
        //  vec.append(&mut vec2)
        //      Move elements of 'vec2' into 'vec', leaving 'vec2' empty
        //
        //  vec.drain(range) -> Drain<T>
        //      Removes 'vec[range]' from Vector, returning an iterator of those values
        //
        //  vec.retain(test)
        //      Remove all elements for which 'test(element)' is False
        //
        //  vec.dedup()
        //      Remove repeated (adjacent) elements from Vector
        //
        //  vec.dedup_by(same)
        //      As per 'dedup', but use closure 'same' to determine whether two elements are equal
        //  
        //  vec.dedup_by_key(key)
        //      As per 'dedup' by treat elements as equal if key(&mut e1) == key(&mut e2)


        //  Remove duplicate values without sorting / losing order
        //  (Alternative to 'vec.dedup()')
        let mut byte_vec = b"Misssssssissippi".to_vec();
        let mut seen = HashSet::new();
        byte_vec.retain(|r| seen.insert(*r));
        assert_eq!(&byte_vec, b"Misp");

    }
    growing_and_shrinking();


    fn joining()
    {
        //  Methods where 'slices' is an array-of-arrays

        //  slices.concat()
        //      Return vector made by concatenating all slices
        assert_eq!( [[1,2],[3,4],[5,6]].concat(), vec![1,2,3,4,5,6] );

        //  slices.join(&seperator)
        //      As per 'concat()', but place (copy of) 'seperator' between slices
        assert_eq!( [[1,2],[3,4],[5,6]].join(&0), vec![1,2,0,3,4,0,5,6] );

    }
    joining();


    fn splitting()
    {
        //  We cannot have more than one mutable reference into Vector at a time
        let mut v = vec![1,2,3];
        //let a = &mut v[0];
        //let b = &mut v[1];                    //  error, cannot borrow more than 1 mutable reference
        //println!("a=({:?})", a);

        //  Instead, Rust provides methods that can return multiple mutable references into Vector
        //  <(These are safe as they split the Vector's data into non-overlapping regions)>

        //  These functions do not modify an array/slice/vector, they merely return references each part of the data inside
        //
        //  slice.iter()
        //  slice.iter_mut()
        //      Produce a reference/mutable-reference to each element of the slice
        //
        //  slice.split_at(index) -> (&[T], &[T])
        //  slice.split_at_mut(index) -> (&mut [T], &mut [T])
        //      Divide a slice, returning (&slice[..index], &slice[index..])
        //
        //  slice.split_first() -> (&T, &[T])
        //  slice.split_first_mut() -> (&mut T, &mut [T])
        //      <(Equivalent to 'split_at(0)')>
        //
        //  slice.split_last() -> (&[T], &T)
        //  slice.split_last_mut() -> (&mut [T], &mut T)
        //      As with 'split_first', but for the last element
        //  
        //  slice.split(is_sep) -> Split<T>
        //  slice.split_mut(is_sep) -> SplitMut<T>
        //      Split where 'is_sep(&element)' is True, returning an iterator over subslices
        //      (matched elements are not included in subslices)
        //
        //  slice.splitn(n, is_sep) -> SplitN<T>
        //  slice.splitn_mut(n, is_sep) -> SplitNMut<T>
        //      As per 'split()', but return at most 'n' subslices
        //
        //  slice.rsplitn(n, is_sep) -> RSplitN<T>
        //  slice.rsplitn_mut(n, is_sep) -> RSplitNMut<T>
        //      As per 'splitn()', but scanning/splitting the slice in reverse order
        //
        //  slice.chunks(n) -> Chunks<T>
        //  slice.chunks_mut(n) -> ChunksMut<T>
        //      Return an iterator over non-overlapping subslices of length 'n'
        //
        //  slice.windows(n) -> Windows<T>
        //      Return an iterator over all contiguous windows of length 'n'
        //      (subslices overlap, hence 'windows_mut' is not available)


        //  Example: 'windows(2)' is useful for finding the difference between values
        let temps = vec![21, 24, 17, 14, 16, 29, 23];
        let temps_delta = temps.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        println!("temps=({:?}), temps_delta=({:?})", temps, temps_delta);

    }
    splitting();


    fn swapping()
    {
        //  slice.swap(i, j)
        //      Swaps elements 'slice[i]' / 'slice[j]'

        //  vec.swap_remove(i)
        //      Removes 'vec[i]', replacing it with the Vectors last element
        //      (Fast removal of an item from the middle of a vector, changes the vector's order)

    }
    swapping();


    fn sorting_and_searching()
    {
        //  slice.sort()
        //      Stable sort Vector elements into increasing order
        //      (T must implement Ord)
        //
        //  slice.sort_by(cmp)
        //      Stable sort using comparison function 'cmp(e1, e2)'
        //      Compare tuples to sort by multiple fields
        //  
        //  slice.sort_by_key(key)
        //      Stable sort by 'key(element) -> K', where K implements Ord
        //      (note that 'key(element)' is not cached - function may be called many times)
        //      'key' functions which borrow a reference from the element cannot be used

        //  slice.reverse()
        //      Reverse in-place

        //  slice.contains(x) -> bool
        //      Does slice contain 'x'. 
        //      Performs linear search, 'binary_search()' will be faster for a sorted list

        //  slice.iter().position(|x| *x == value)
        //      Locate index of 'value' in slice

        //  A sorted slice can be efficently searched:
        //  (binary search of an unsorted list is not meaningful)
        //
        //  slice.binary_search(&value) -> Result<usize, usize>
        //      Binary search for a given element. If element is found, returns its index as Ok.
        //      If element is not found, return where it would be inserted as Err.
        //      <(If there are multiple matches, the index of any of them any be returned)>
        //
        //  slice.binary_search_by(&value, cmp) -> Result<usize, usize>
        //      As per 'binary_search()', using comparison function 'cmp(e1, e2)'
        //      
        //  slice.binary_search_by_key(&value, key)
        //      As per 'binary_search()', but use 'key(element) -> K', where K implements Ord

        //  <(Use the 'ord_subset' crate to support these methods on containers of f32/f64)>

    }
    sorting_and_searching();


    fn comparing_slices()
    {
        //  If T supports PartialEq, then so do [T; N] / [T] / Vec<T>

        //  If T supports PartialOrd, then so do [T; N] / [T] / Vec<T>

        //  slice.starts_with(values: &[T]) -> bool
        //  slice.ends_with(values: &[T]) -> bool
        //      Does the slice start/end with elements in 'values'
        assert!([1,2,3,4].starts_with(&[1,2]));
        assert!([1,2,3,4].ends_with(&[3,4]));

    }
    comparing_slices();


    fn random_elements()
    {
        let mut v1 = (0..=9).collect::<Vec<i32>>();

        use rand::thread_rng;
        use rand::seq::SliceRandom;

        //  rng.choose(&slice) -> Option<&T>
        //      Return reference to random element in 'slice'
        let x: &i32 = v1.choose(&mut thread_rng()).unwrap();

        //  rng.shuffle(&mut slice)
        //      Randomly re-order elements in 'slice'
        v1.shuffle(&mut thread_rng());
        //thread_rng().shuffle(&mut v1)         //  (invalid) (book example)

    }
    random_elements();


    fn invalidation_errors()
    {
        //  In most languages, modifying a container while iterating over it invalidates the iterator
        //  This is undefined behaviour in C++, (Java will throw an exception)

        //  Rust does not allow us to modify a container while a reference to an element in it exists

        //  This prevents us from writing certain algorithms (that we shouldn't have been writing anyway)
        //  instead use functions like Vec.filter() / Vec.retain() 

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

