//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2023-01-19T20:27:43AEDT added to cheatsheet 'Rust' (not finished, update document when this is finished)
//  Ongoing: 2023-01-18T00:48:59AEDT 'Arc<T>' is like 'shared_ptr<T>' (implying 'Rc<T>' is not)) [...] (actually - 'shared_ptr<T>' is akin to 'Arc<Mutex<T>>'(?))
//  Ongoing: 2023-01-20T01:35:26AEDT move 'RefCell' before 'Cell' (after as a commit not involving changes to either(?))
//  Ongoing: 2023-01-20T01:36:11AEDT (I'm sure there are) missing (key) actions/operations for some wrappers(?)
//  Ongoing: 2023-01-20T01:38:18AEDT difference between Ref<T> and &T(?)
//  }}}
//  macro: get_func_name
//  {{{
macro_rules! get_func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}
//  }}}
use std::rc::Rc;
use std::cell::RefCell;

//  Continue: 2023-01-20T01:48:16AEDT programming-rust, wrapper-types, uses for each wrapper (with labeling '&mut T' (as used in RefCell)
//  Continue: 2023-01-20T01:48:51AEDT programming-rust, wrapper-types, 'Ref<T>'

//  LINK: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/


fn example_pointer_types()
{
    //  Box<T>
    //  Owned pointer, can hand out borrowed references (but is only owner) 
    //  <(Simplest)> way to allocate memory on heap, subject to compile time borrow checking
    //  Create:                 Box::new(object)
    //  Dereference:            *foo            foo.method()
    //  Borrow reference:       foo.as_ref()    foo.as_mut()
    //  Move-out:               x = *foo
    //  Move:                   x = foo
    //  Clone:                  foo.clone()
    let x = Box::new( Vec::<i32>::new() );


    //  &T and &mut T
    //  Shared/mutable reference
    //  Mutable references are exclusive
    //  Cannot be copied such that they outlive referenced objects


    //  *const T and *mut T
    //  Raw (C-like) pointers
    //  Can only be dereferenced in unsafe blocks


    //  Rc<T>
    //  Reference counted pointer
    //  Allows for multiple 'owning' pointers to the same data
    //  Used to share read-only data between objects (when it is not clear who will use data last)
    //  Guarantees:
    //      Data will not be destroyed until all references go out of scope
    //      Internal data is immutable
    //  Cost:
    //      Larger than Box<T>
    //      Reference counting overhead
    //      Not thread safe
    //      Allows cycles to be introduced
    //  Clone will increment reference count and return copy (instead of deep copying)
    //  Dereference:            *foo                                foo.method()
    //  Borrow reference:       foo.as_ref()                        <(cannot 'foo.as_mut()')>
    //  Clone-value:            x = foo.clone()                     
    //  Copy-value:             x = *foo 
    //  Move:                   x = foo
    //  Move-out:               <(impossible?)>



    //  Weak<T>
    //  Non-owning, non-borrowed smart pointer
    //  Returns None if internal data has been released
    //  <()>

    println!("{}: DONE", get_func_name!());
}

use std::cell::Cell;

fn example_cell_types()
{
    //  Cell types provide interior mutability
    //  (relaxes no aliasing with mutability restriction)

    //  Cell<T>
    //  Zero cost interior mutability for Copy types
    //  Each 
    //  Guarantees:
    //      Doesn't allow references to inside of a type
    //      <(Enums/structs are safe to be aliased mutably within)>
    //  Cost:
    //      <(each read/write requires performing a copy)>
    //      <(For large structs, it can be better to instead wrap individual fields in Cell<T>)>
    //      Can be used to violate Rust safety/invariants
    //      Shared mutability can cause logic errors
    //  <()>
    //  Example: x/y/z are immutable containers whose contents we can mutate
    let mut x: Cell<i32> = Cell::new(1);
    let y: &Cell<i32> = &x;
    let z: &Cell<i32> = &x;
    x.set(3);
    y.set(4);
    z.set(5);
    println!("x=({})", x.get());
    //  Dereference:        <(impossible?)>             
    //  Borrow value:       <(impossible?)>
    //  Borrow pointer:     foo.as_ptr()                <(no 'foo.as_mut_ptr()'?)>
    //  Copy-in:            foo.insert(x)
    //  Copy-out:           x = foo.get()
    //  Move-out:           x = foo.get_mut()


    //  RefCell<T>
    //  Interior mutability not restricted to Copy types
    //  (single threaded mutex, shifts RWLock checking from compile-time to runtime)
    //  Panics at runtime if programmer attempts to make a shared borrow while a mutable borrow is active
    //  <(Placing things in RefCell<T> can simplify a large program)>
    //  Best used only when simple solution is not possible with traditional references
    //  Guarantees:
    //      Provides compile-time aliased mutation restrictions at runtime
    //  Cost:
    //      Borrow state refcount overhead
    //      Not thread-safe
    //  Ref<'_,T>:          foo.borrow()                
    //  RefMut<'_,T>:       foo.borrow_mut()
    //  Result<&T>:         foo.try_borrow_unguarded()  [unsafe]
    //  &mut T:             foo.get_mut()
    //  *mut T:             foo.as_ptr()
    //  &T:                 unsafe { &*foo.as_ptr() }
    //  &mut T:             unsafe { &mut *foo.as_ptr() }
    //  <('borrow_mut()' vs 'get_mut()')>


    //  Ref<T>
    //  <()>

    println!("{}: DONE", get_func_name!());
}


fn example_syncronous_types()
{
    //  Allows sharing between threads
    //  (compiler will enforce that non-threadsafe types are not shared between threads)

    //  Arc<T>
    //  Atomic reference counted pointer
    //  Inner data is always mutable
    //  (use 'Arc<Mutex<T>>' to emulate 'shared_ptr<T>')
    //  (prefer '&' when sharing within a single thread where possible)
    //  Guarantees:
    //      Thread-safe
    //  Cost:
    //      Atomic reference counting overhead
    //      Allows cycles to be introduced


    //  Mutex<T>
    //  Provide mutual-exclusion via RAII guards
    //  Full lock for reading and writing
    //
    //  RwLock<T>
    //  Provide mutual-exclusion via RAII guards
    //  Efficient for multiple reads 
    //  Always safe to have multiple readers if there are no writers
    //
    //  Guarantees:
    //      Safe shared mutability across threads
    //  Cost:
    //      Atomic instruction overhead
    //      Can incur deadlocks


    //  UnsafeCell<T>
    //  'RefCell' with safety checks removed
    //  (compiler primitive used to implement 'Cell' / 'RefCell')

    println!("{}: DONE", get_func_name!());
}


fn example_wrapper_composition()
{
    //  Wrapper composition

    //  Rc<RefCell<T>>
    //  Provides shared ownership with mutability of 'T'

    //  Arc<Mutex<T>>
    //  Provides shared mutability and ownership.

    println!("{}: DONE", get_func_name!());
}


fn main() 
{
    example_pointer_types();
    example_cell_types();
    example_syncronous_types();
    example_wrapper_composition();
}

