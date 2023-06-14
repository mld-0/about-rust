//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-07T01:25:42AEDT 'pub' is a module-level (and not an object-level) concept(?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

//  Rust programs are made of crates
//  Each crate is a Rust project

//  Use '--verbose' when building a project for a complete view of included crates

//  Use the 'extern' keyword to tell Rust that a crate is an external library
//extern crate num;
//extern crate image;
//extern crate crossbeam;
//  (specify in 'Cargo.toml' which versions should be used)

//  Rust uses '--crate-type lib' to compile libraries (as opposed to '--crate-type bin')
//  This produces an '*.rlib' file containing the compiled library (including type information)
//  These will be statically linked into the final executable

//  Build profiles:
//  <()>


//  Modules are Rust's namespaces. 
//  Crates are for code sharing between projects, modules are for code organization within a project.
//mod spores {
//    use cells::Cell;
//    pub struct Spore {}
//    pub fn produce_spore(factory: &mut Sporangium) -> Spore {}
//    fn recombine(parent: &mut Cell) {}
//}
//  'pub' denotes an item should be accessible outside a module (anything else is private)
//  Modules can be nested. We can declare a hierachy of modules in a single source file 
//  (but Rust provides a better alternative)

//  <(Import a module from another file ('spores.rs' or 'spores/mod.rs'))>
//mod spores;

//  Modules are not compiled seperately (even if in a seperate file), building a Rust crate builds all its modules.

//  The '::' operator is used to access features of a module.
//  To access the standard library (from anywhere), use '::std'

//  To prevent the need for this absolute path every time, make the module available:
fn example_import() {
    //  Declares 'mem' as a local alias for '::std::mem'
    //  Importing the containing module rather than the actual function is better style
    use std::mem;       
    let mut s1 = 5;
    let mut s2 = 7;
    if s1 > s2 { mem::swap(&mut s1, &mut s2); }

    //  Several modules can be imported at once:
    use std::collections::{HashMap,HashSet};
    use std::io::prelude::*;

    //  Modules do not automatically inherit names from their parent modules
    //  (they must import everything they use, even from their parents)

    //  Within an import, 'super' is an alias for the parent module, and 'self' is an alias for the current module
    //  Use these to write relative import paths

    //  Submodules can access private items in their parent module, but they must import each one by name
    //  (Wildcard 'self::AminoAcid::*' imports only import public items)

    //  <(Modules are not the same things as files, but they serve as an organisational analogy)>

    //  Each module starts with an implicit:
    extern crate std;
    use std::prelude::v1::*;
    //  (recall, naming a module 'prelude' denotes it should be imported with '*')

    println!("example_import, DONE");
}


//  A module is made up of items:
//      Functions
//      Types
//      Type aliases
//      impl blocks
//      constants
//      (sub)modules
//      imports
//      extern blocks (functions in another language)

//  Rust warns about items that are declared but never used 
//  <(unless they, and all enclosing modules, are marked public)>
//  (unless given '#![allow(unused)]')


fn main() 
{
    example_import();
}

