//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-07T23:27:37AEDT what does it even mean for a struct (or its fields) to be private/public [...] (structs are private by default, visible only in the module where they’re declared. You can make a struct visible outside its module by prefixing its definition with pub) [...] (a Rust module is analogues to a file(?)) [...] examples of where this outside-module use of structs/their-elements possible(?) 
//  Ongoing: 2022-10-08T03:06:29AEDT semicolons are optional after some <items/definitions> (eg: after declaring a struct) (where/meaning-of an optional semicolon?)
//  Ongoing: 2022-10-08T03:11:47AEDT Queue (from two stacks) - is this here the same algorithm we used on this as a leetcode problem?
//  Ongoing: 2022-10-08T03:16:55AEDT Vector, (and other Rust types/containers) are designed to be swapped with '::std::mem::swap()' (what about using it on our own types (do we have to <specialise> it?)) [...] (consider the question again for (Queue<T>, which uses) (containers of) paramaterized types)
//  Ongoing: 2022-10-08T03:19:12AEDT ctors/dtors? (it is inconcievable that 'let mut q = Queue { older: Vec::new(), younger: Vec::new() }' is good design?)
//  Ongoing: 2022-10-08T03:30:55AEDT Rust, (and just not worrying about memory leaking?) (when/how it can/can't happen?) [...] (and the need to even declare dtors?)
//  Ongoing: 2022-10-08T03:40:18AEDT <significance/implications> (that we don't use mut T in Queue (either for 'Vec<mut T>' or 'push(c: mut T)' ... the things we push are non-mutable and passed by value (moved)))
//  Ongoing: 2022-10-08T03:49:25AEDT (the implication is) reference lifetime variables are just <> to type parameters (declare in the same <> prefixed with ')
//  Ongoing: 2022-10-08T03:50:58AEDT why book choses 'elt as its lifetime parameter
//  Ongoing: 2022-10-08T03:54:38AEDT 'Extrema' example with parameterized type and reference lifetime (also implement 'find_extrema' as a method of Extrema?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

//  Convention is to use CamelCase for types (including structs), and snake_case for fields/methods

//  Named-field struct:
fn example_named_field() 
{
    //  struct declaration
    struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize),
    }

    let width = 1024;
    let height = 576;
    let pixels = vec![0; width * height];
    let size = (width, height);

    //  struct expression:
    let img1 = GrayscaleMap { 
        pixels: vec![0; width * height],
        size: (width, height),
    };
    //  variables of the same name as fields can be used without labels
    let img2 = GrayscaleMap { pixels, size, };
    //  (in both cases, elements can be given in any order)

    //  access a structs field with the '.' operator
    assert_eq!(img1.size, (width,height));
    assert_eq!(img1.pixels.len(), width*height);

    //  structs are private by default, declare a struct public with 'pub'
    pub struct GrayscaleMapPub {
        pixels: Vec<u8>,
        size: (usize, usize),
    };
    //  struct elements are private by default, declare public elements with 'pub'
    struct GrayscaleMapElementsPub {
        pub pixels: Vec<u8>,
        pub size: (usize, usize),
    };
    //  (note the independence of struct/element publicness)

    //  Creating a struct requires all its fields to be visible (to the current scope)

    #[derive(Copy,Clone)]
    enum BroomIntent { FetchWater, DumpWater, };
    struct Broom { name: String, height: u32, health: u32, position: (f32,f32,f32), intent: BroomIntent, };

    //  A struct of the same type can be used to supply values for omitted fields
    //  if named fields are followed by '.. EXPR', then any fields not mentioned take their value from EXPR
    fn chop(b: Broom) -> (Broom,Broom) {
        let mut b1 = Broom { height: b.height/2, .. b };
        let mut b2 = Broom { name: b1.name.clone(), .. b1 };
        b1.name.push_str(" i"); b2.name.push_str(" ii");
        (b1, b2)
    }

    let hokey = Broom { name: "Hokey".to_string(), height: 60, health: 100, position: (100.0,200.0,0.0), intent: BroomIntent::FetchWater, };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey i");
    assert_eq!(hokey2.name, "Hokey ii");

    println!("example_named_field, DONE");
}


fn example_tupleLike_structs() 
{
    //  Declaration:
    struct Bounds(usize,usize);
    //  (this also implicitly defines a function: 'fn Bounds(e0: usize, e1: usize) -> Bounds {...}')

    //  <(Construction:)>
    let image_bounds = Bounds(1024,768);

    //  Access the elements of a tuple-like struct like those of a tuple
    assert_eq!(image_bounds.0 * image_bounds.1, 1024*768);
    //  (only constants, not variables, can be used as tuple indexes)

    //  Tuples may be public
    pub struct BoundsPub(usize,usize);
    //  and/or may their elements:
    struct BoundsElementsPub(pub usize, pub usize);

    //  newtypes: structs with a single element, used for stricter type checking / documentary value
    struct Ascii(Vec<u8>);
    //  (Passing around 'Ascii' adds documentary value and prevents any other Vec<u8> from being accepted)

    println!("example_tupleLike_structs, DONE");
}


fn example_unitLike_structs() 
{
    //  A unit-like struct is a struct with no elements
    struct Onesuch;
    //  Like the unit type '()', these occupy no memory

    //  <(Construction:)>
    let o = Onesuch;

    //  '..' is shorthand for the unit-like struct 'RangeFull'

    println!("example_unitLike_structs, DONE");
}


fn example_struct_memoryLayout() 
{
    //  <(Non-heap allocated struct elements are store contiguously in memory <on the stack>)>
    //  Unlike C/C++, Rust doesn't make promises about their order 
    //  Use '#[repr(C)]' to lay out structs in a C-compatible way
    println!("example_struct_memoryLayout, DONE");
}


fn example_defining_methods_impl() 
{
    //  Rust allows methods to be defined <for/on> any struct type
    //  Unlike C++, methods are defined in seperate impl blocks

    //  An impl block adds methods to the struct named at the top
    //  Rust calls methods 'associated functions' (as opposed to 'free functions')
    pub struct Queue {
        older: Vec<char>,
        younger: Vec<char>,
    }
    impl Queue {
        //  methods recieve a reference to 'self' (or mutable self) as their first argument 
        //  member variables must explicitly be referenced through 'self'
        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }
        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            self.older.pop()
        }
        //  Use a non-mutable 'self' reference if a mutable one is not required
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
        //  A method can take ownership of itself by recieving 'self' by-value
        pub fn split(self) -> (Vec<char>,Vec<char>) {
            (self.older, self.younger)
        }
        //  Static methods are methods that do not recieve 'self'
        //  These can be used to create constructor functions
        //  Convention is to call constructor functions 'new' (though this is not a requirement)
        pub fn new() -> Queue {
            Queue { older: Vec::new(), younger: Vec::new(), }
        }
        //  <(Rust provides <custom> destructors as 'drop(&mut self)')>
        //  <(default <implicit> dtor behaviour / need for dtors at all?)>
    }

    //  Custom creation
    let mut queue = Queue { older: Vec::new(), younger: Vec::new() };

    //  <ctor/factory-function> creation
    let mut queue = Queue::new();

    assert!(queue.is_empty());
    queue.push('0');
    queue.push('1');
    assert!(!queue.is_empty());
    assert_eq!(queue.pop(), Some('0'));
    queue.push('2');
    assert_eq!(queue.pop(), Some('1'));
    assert_eq!(queue.pop(), Some('2'));
    assert_eq!(queue.pop(), None);
    assert!(queue.is_empty());

    //  The '.' operator automatically dereferences its operands:
    //  Equivalent:
    //queue.push('1');
    //(&mut queue).push('1');

    //  Rust permits any type to have methods defined (including enums and primatives)
    //  (This is why Rust does not use the term 'object', instead calling everything a value)

    //  <(impl blocks are also used to implement traits)>

    println!("example_defining_methods_impl, DONE");
}


fn example_generic_structs() 
{
    //  Rust allows generic structs, that is templates with variable type(s)
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>,
    }
    impl<T> Queue<T> {
        pub fn push(&mut self, c: T) {
            self.younger.push(c);
        }
        pub fn pop(&mut self) -> Option<T> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            self.older.pop()
        }
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
        pub fn split(self) -> (Vec<T>,Vec<T>) {
            (self.older, self.younger)
        }
        pub fn new() -> Queue<T> {
            Queue { older: Vec::new(), younger: Vec::new(), }
        }
    }

    //  The type parameter can be supplied explicitly
    let mut queue = Queue::<char>::new();

    //  Or we can let Rust deduce it from how we use the variable in question
    let mut queue = Queue::new();
    assert!(queue.is_empty());
    queue.push('0');
    queue.push('1');
    assert!(!queue.is_empty());
    assert_eq!(queue.pop(), Some('0'));
    queue.push('2');
    assert_eq!(queue.pop(), Some('1'));
    assert_eq!(queue.pop(), Some('2'));
    assert_eq!(queue.pop(), None);
    assert!(queue.is_empty());

    println!("example_generic_structs, DONE");
}


fn example_struct_reference_lifetimeParameters() 
{
    //  If a struct contains references, we must name those references lifetime (see ch05)
    //struct S1 { x: &i32 }         //  invalid
    //  This is done with lifetime parameters
    struct S2<'a> { x: &'a i32 }

    struct Extrema<'a> { greatest: &'a i32, least: &'a i32, }
    fn find_extrema<'a>(slice: &'a [i32]) -> Extrema<'a> {
        let mut greatest = &slice[0];
        let mut least = &slice[0];
        for i in 1..slice.len() {
            if slice[i] < *least { least = &slice[i]; }
            if slice[i] > *greatest { greatest = &slice[i]; }
        }
        Extrema { greatest, least }
    }
    let a = [0,-3,0,15,48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    //  <(Alternatively, we can omit the lifetime reference parameter when there's one obvious candidate)>
    //  Also valid: 'fn find_extrem(slice: &[i32]) -> Extrema'
    //  <(So long as we didn't mean Extrema<'static>)>

    //  <(impl Extrema)>
    //  <(struct Extrema<T,'a>)>

    println!("example_struct_reference_lifetimeParameters, DONE");
}

fn example_deriving_common_struct_traits()
{
    println!("example_deriving_common_struct_traits, DONE");
}


fn main() 
{
    example_named_field();
    example_tupleLike_structs();
    example_unitLike_structs();
    example_struct_memoryLayout();
    example_defining_methods_impl();
    example_generic_structs();
    example_struct_reference_lifetimeParameters();
    example_deriving_common_struct_traits();
}

