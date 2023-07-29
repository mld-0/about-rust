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


    //  Creation:
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
}


//  RefCell<T>
fn RefCell_T()
{
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
    Option_T();
    Box_T();
    Option_Box_T();
    Rc_T();
    RefCell_T();
    Rc_RefCell_T();
    println!("main(), DONE");
}

