//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-07T23:27:37AEDT what does it even mean for a struct (or its fields) to be private/public [...] (structs are private by default, visible only in the module where they’re declared. You can make a struct visible outside its module by prefixing its definition with pub) [...] (a Rust module is analogues to a file(?)) [...] examples of where this outside-module use of structs/their-elements possible(?)
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
}


fn main() 
{
    example_named_field();
    example_tupleLike_structs();
    example_unitLike_structs();
    example_struct_memoryLayout();
    example_defining_methods_impl();
}

