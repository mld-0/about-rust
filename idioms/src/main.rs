//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  }}}
//#![allow(unused)]
#![allow(non_snake_case)]


fn match_blocks() 
{
}


fn if_let()
{
}


fn while_let()
{
}


fn about_enums()
{
}


fn automatic_dereferencing()
{
    //  [{automatic dereferencing == implicit dereferencing == deref coercions?}]
}


//  Option<T>
fn Option_T()
{
    //  LINK: Module std::option (see below)
    //  https://doc.rust-lang.org/std/option/

    //  is_some()           
    //  is_none()

    //  Adapters for working with references:
    //  as_ref():               &Option<T> -> Option<&T>
    //  as_mut()                &mut Option<T> -> Option<&mut T>
    //
    //  as_deref():             &Option<T> -> Option<&T::Target>
    //  as_deref_mut():         &mut Option<T> -> Option<&mut T::Target>


    //  Extract contained value:
    //  expect():               <>
    //  unwrap():               <>
    //  unwrap_or():            <>
    //  unwrap_or_default()     <>
    //  unwrap_or_else()        <>

    //  Transform contained value:
    //  <>

}


//  Box<T>
fn Box_T()
{
}

//  Option<Box<T>>
fn Option_Box_T()
{
}


//  Rc<T>
fn Rc_T()
{
    use std::rc::Rc;
    //  std::rc::Rc is a reference counting pointer
    //  (alternatives include `std::rc::Arc` for a threadsafe counting pointer, and `std::rc::Weak` for a non-owning pointer)

    //  Generally it does not allow mutation of its contents (see `Rc_RefCell_T`)

    //  Functions associated with Rc are called as `Rc::get_mut()` instead of `x.get_mut()` to avoid any conflicts with the methods of the inner type T


    //  Rc::new() 
    //  Creation
    let s: Rc<String> = Rc::new("abc".to_string());


    //  Rc<T> automatically derefs to T 
    //  (Weak<T> does not, since the inner value may have been dropped)
    let x: &str = &s;


    //  Rc::clone(): copies the pointer, increasing the reference count 
    //  (direct assignment moves the pointer instead)
    //  The following are equivalent:
    let s2: Rc<String> = s.clone();
    let s3: Rc<String> = Rc::clone(&s);


    //  Reference count is decremented when each Rc is dropped
    println!("reference_count(s)=({})", Rc::strong_count(&s));
    drop(s2);
    println!("reference_count(s)=({})", Rc::strong_count(&s));
    drop(s3);
    println!("reference_count(s)=({})", Rc::strong_count(&s));


    //  as_ptr() / as_mut_ptr():
    //  obtain a raw pointer to underlying value, without consuming owning pointer
    //  pointer remains valid so long as the reference count remains above 0
    let p_s = s.as_ptr();


    //  into_raw() / from_raw():
    unsafe {
        let y = Rc::new("abc".to_owned());
        //  obtain a raw pointer to the underlying value, consuming the owning pointer
        let p_y = Rc::into_raw(y); 
        //  this value *must* be converted back to an owning pointer to avoid a memory leak
        let y = Rc::from_raw(p_y);
        //  `Rc::from_raw` is *only* valid when used on a pointer obtained from `Rc::into_raw`
    }

    //  Rc::downgrade()
    //  Obtain a Weak<T> (non-owning) pointer to the underlying value
    let w_s = Rc::downgrade(&s);


    //  Rc::get_mut()
    //  Returns a mutable reference to underlying value as Some(&T) if reference count is 1, or None otherwise
    //  (only valid if Rc is declared as mutable)
    let mut x = Rc::new(3);
    let mut_ref_x = Rc::get_mut(&mut x).unwrap();
    *mut_ref_x = 4;
    //  (this mutable reference is dropped when we attempt to use the Rc pointer again)
    assert_eq!(*x, 4);
    //  (cannot get mutable reference into Rc when it is owned by more than one owning pointer)
    let x2 = x.clone();
    assert_eq!(Rc::get_mut(&mut x), None);
    drop(x2);
    assert_eq!(*Rc::get_mut(&mut x).unwrap(), 4);


    //  [{presumedly, using raw pointers to mutate an Rc with multiple owners is UB?}]


    //  Rc::make_mut(&mut Rc<T>) -> &mut T
    //  <>


    //  Rc and ownership cycles:
    //  <>


    //  Trait implementations:
}


//  RefCell<T>
fn RefCell_T()
{
    use std::cell::RefCell;
    //  std::cell:RefCell provides 'interior mutability' - the ability to mutate data stored in an immutable variable
    //  (alternatives include `Cell<T>` [{for copy types?}], and `RwLock<T>` for threadsafe applications)

    //  `RefCell` represents single ownership over the data it holds (unlike `Rc` which allows multiple ownership)

    //  `RefCell<T>` enforces the same borrowing rules as `Box<T>` (any number of immutable borrows, or a single mutable borrow, but not both) - however, it enforces them at runtime instead of at compile-time (and will panic at runtime if we attempt to violate them)
    //  The Rust borrow checker is inherently conservative, it rejects certain memory-safe borrows. Using `RefCell<T>` allows us to implement such algorithms that would be rejected by the compiler.

    //  [{`RefCell` can be used for operations that are logically immutable, but implementation details requires mutation in the implementation, eg: caching, mock-objects}]

    //  (`Rc<RefCell<T>>` is a common Rust idiom for circumventing the immutability of `Rc`, see `Rc_RefCell_T()`)


    //  RefCell::new()
    //  Creation
    let rc = RefCell::new(29);


    //  borrow(&self) -> Ref<'_, T>
    //  Immutable borrows the wrapped value
    //  The borrow lasts until the returned `Ref` exists the current scope
    //  Panics if a mutable borrow of the wrapped value exists
    //  (note the returned value is a `Ref` object, not `&T`)


    //  borrow_mut(&self) -> RefMut<'_, T>
    //  Mutably borrows the wrapped value
    //  The borrow lasts until the returned `RefMut` exists the current scope
    //  Panics if any borrow of the wrapped value exists
    //  (note the returned value is a `RefMut` object, not `&T`)


    //  try_borrow(&self) -> Result<Ref<'_, T>, BorrowError>
    //  Non-panicking variant of `borrow()`


    //  try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError>
    //  Non-panicking variant of `borrow_mut()`


    //  Example: attempting to borrow_mut from a RefCell while another reference exists will not be rejected by the compiler, but will panic at runtime
    //  (note that we must explicitly drop the immutable reference - unlike the borrow-checking of the compiler, references obtained from a RefCell are not dropped at the first possible opportunity)
    let rc = RefCell::new(53);
    let r1_rc = rc.borrow();
    //let mut mr1_rc = rc.borrow_mut();     //  will panic, immutable reference already exists
    drop(r1_rc);
    let mut mr1_rc = rc.borrow_mut();       //  valid, no other reference exists
    //let mut mr2_rc = rc.borrow_mut();     //  will panic, mutable reference already exists
    drop(mr1_rc);


    //  into_inner(self) -> T
    //  Consumes the RefCell, returning the owned value
    let x = 53;
    let rc = RefCell::new(x);
    let x = rc.into_inner();


    //  replace(&self, t: T) -> T
    //  Replace the wrapped value with a new one, returning the old value
    //  (corresponds to `std::mem::replace()`)


    //  replace_with(&self, f: F) -> T
    //  Like `replace`, but `f` is a function to be applied to the old value
    //  [{owned value must be copy?}]


    //  swap(&self, other: &RefCell<T>)
    //  Swap the contents of two RefCells
    //  (corresponds to `std::mem::swap`)


    //  as_ptr(&self) -> *mut T
    //  Returns a mutable raw pointer to the underlying value
    //  [{use of `as_ptr()` raw pointer and UB?}]


    //  get_mut(&mut self) -> &mut T
    //  Returns an actual mutable reference to the underlying data
    //  Since this method borrows RefCell mutably, the compiler can check whether any other borrows exist, just as it does with `Box<T>` (making runtime checks unnecessary)


    //  take(&self) -> T
    //  Return the wrapped value, consuming the RefCell
    //  (panics if the wrapped value is currently borrowed)


    //  Trait implementation:
    //  {{{
    //  The following traits are automatically implemented for `RefCell<T>` if `T` implements them:
    //      Clone (panics if a mutable borrow exists of the wrapped value)
    //      Debug
    //      Default
    //      From
    //      Ord (panics if a mutable borrow exists for either wrapped value)
    //      PartialEq (panics if a mutable borrow exists for either wrapped value)
    //      PartialOrd (panics if a mutable borrow exists for either wrapped value)
    //      CoerceUnsized
    //      Eq
    //      Send
    //      !Sync

    //  Auto-trait implementations: 
    //      !RefUnwindSafe
    //      Unpin
    //      UnwindSafe
    
    //  Blanket implementations
    //      Any
    //      Borrow
    //      BorrowMut
    //      From
    //      Into
    //      ToOwned
    //      TryFrom
    //      TryInto
    //  }}}
}


//  Rc<RefCell<T>>
fn Rc_RefCell_T()
{
}


//  Sized?
//  <>


//  mem::replace()
//  <>


//  take()
//  <>


//  [{&T vs T::Ref}]
//  <>


fn main() 
{
    match_blocks();
    about_enums();
    if_let();
    while_let();
    automatic_dereferencing();
    Option_T();
    Box_T();
    Option_Box_T();
    Rc_T();
    RefCell_T();
    Rc_RefCell_T();
    println!("main(), DONE");
}

