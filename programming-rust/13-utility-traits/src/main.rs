//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-16T01:50:56AEDT (cannot create an unsized type directly), takeaway -> we cannot create an instance of an unsized type? (unsized types are only ever pointers to sized types?)
//  Ongoing: 2022-10-19T00:31:24AEDT passing a reference to T vs to &T (T can be <derived> as a reference type?)
//  Ongoing: 2022-10-19T00:34:31AEDT (a better explanation of type coercion)
//  Ongoing: 2022-10-19T23:04:47AEDT (rule for) when AsRef/AsMut get invoked automatically
//  Ongoing: 2022-10-19T23:09:35AEDT can AsRef<T> be implemented for multiple T for a give type(?) (how to chose which one is invoked by '.as_ref()' (other methods of invoking AsRef))
//  Ongoing: 2022-10-19T23:38:16AEDT how AsRef<Path> works for String
//  Ongoing: 2022-10-19T23:43:14AEDT AsRef<&str>
//  Ongoing: 2022-10-19T23:50:27AEDT String implements Borrow<str> (not Borrow<&str>?)
//  Ongoing: 2022-10-19T23:59:07AEDT significance of deriving a trait from Sized
//  Ongoing: 2022-10-20T00:00:11AEDT book uses 'fn from(T)' (ignoring varname) where T is the type parameter -> we cannot (not supply the varname)?
//  Ongoing: 2022-10-20T00:33:47AEDT how to specify function parameter bound 'MyType::From(T)' exists 
//  Ongoing: 2022-10-20T00:41:21AEDT (To/From are restricted to conversions that cannot fail) -> (can AsRef/Borrow fail?)
//  Ongoing: 2022-10-20T03:58:57AEDT (is it correct to say) '.to_owned()' can return a by-value type where Borrow could only provide a reference to that type?
//  Ongoing: 2022-10-20T04:05:47AEDT which traits (even/especially those with paths given here) do not need to be imported to be used
//  Ongoing: 2022-10-20T04:10:21AEDT put an '_' after 'Example' in each such named trait/type 
//  Ongoing: 2022-10-20T04:14:02AEDT (did we come across this syntax when looking at lifetime parameters?) '<'a, B: ?Sized + 'a>'
//  Ongoing: 2022-10-20T04:18:08AEDT significance of cast: <T as UtilityTrait> (and optionally with '::<something>' appended to it)
//  Ongoing: 2022-10-20T04:50:19AEDT how hard to implement Cow (copy-on-write) type
//  Ongoing: 2022-10-20T04:53:18AEDT clarify, (does 'to_owned()' ever return a reference?)
//  Ongoing: 2022-10-20T05:02:18AEDT (how to put something into) Cow<String> (vs Cow<str>)
//  Ongoing: 2022-10-20T05:33:25AEDT Cow, book example -> book use of 'Error' (something not visible to us?)
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
//  Continue: 2022-10-16T02:09:53AEDT when to make a type copy </clone>
//  Continue: 2022-10-20T04:02:01AEDT other useful traits
//  Continue: 2022-10-20T05:04:19AEDT Cow, how to implement for custom types (definition says we should implement 'ToOwned', but example with str uses '.into()')
//  Continue: 2022-10-20T05:06:52AEDT implement each trait mentioned not mentioned here for a custom type (as applicable)

//  Utility traits:
//      <>Sized                                 Mark type as fixed size known at compile time
//      <>Drop                                  Destructors
//      <>Copy                                  bitwise copy <(shallow copy)>
//      <>Clone                                 Cloning values <(deep copy)>
//      std::ops::{Deref/DerefMut}              dereference custom pointer type
//      std::default::Default                   Types with a 'default' value
//      std::convert::{AsRef/AsMut}             Borrowing references of one type from another
//      std::borrow::{Borrow/BorrowMut}         Like AsRef, but guaranteeing consistent ordering/equality/hashing
//      std::convert::{From/Into}               Converting one type to another (by-value), 1 parameter ctors
//      std::borrow::ToOwned                    Converting a reference to a (by-value) copy (of a borrowable type)

//  Other useful traits:
//      std::fmt::Display
//      <>Debug
//      <>


fn example_drop()
{
    //  Values are dropped when their owner goes away
    //  This is generally automatic when a variable goes out of scope
    //  It is usually only neccessary to implement Drop for resource management classes

    //  A type that implements 'Drop' cannot implement 'Copy'
    #[derive(Debug)]
    struct Appellation {
        name: String, nicknames: Vec<String>,
    }
    impl Appellation {
        fn new(name: String, nicknames: Vec<String>) -> Self {
            Appellation { name, nicknames }
        }
    }

    //  Defintion:
    trait ExampleDrop {
        fn drop(&mut self);
    }

    //  Customise how a given value is dropped by implementing 'std::ops::Drop'
    //  (this is Rust's equivalent of a destructor)
    impl Drop for Appellation {
        fn drop(&mut self) {
            println!("dropping Appellation=({:?})", self);
        }
    }

    //  We can explicitly drop a variable with the function 'drop()'
    //  (this function is trivial)
    fn ExampleDrop<T>(_x: T) {}

    let x = Appellation::new("Zeus".to_string(), vec!["cloud collector".to_string(), "king of the gods".to_string()]);
    drop(x);

    println!("example_drop, DONE");
}


fn example_sized()
{
    use std::fmt::Display;

    //  A 'sized type' is a type whose values all have the same size in memory
    //  (almost all types are sized)

    //  Unsized values cannot be stored in variables or passed as arguments, 
    //  they can only be <accessed> through pointers (which are themselves sized types).
    //  Pointers to unsized values are always fat pointers - they must contain the value's size.
    //  (and in the case of a pointer to a trait object, a pointer to a vtable of method implementations)

    //  (Only sized types can be returned from functions)

    //  String and array slices (str / [T]) are unsized 

    //  The referent of a trait object is unsized
    //  &std::io::Write and Box<std::io::Write> are pointers to some value that implements Write
    //  (this value may be any size)

    //  All sized types implement 'std::marker::Sized' (this is done automatically for custom types)
    //  (When used as a bound, 'Sized' requires the type's size to be known at compile time)
    //  (this is an example of a 'marker trait')

    //  Generic types are sized by default
    //  Equivalent:
    //      S<T>
    //      S<T: Sized>

    //  To specify a type that is questionably (not required to be) sized:
    //      S<T: ?Sized>

    //  Only the last field of a struct can be questionably sized:
    struct ExampleRcBox<T: ?Sized> {
        ref_count: usize,
        value: T,
    }
    //  (the resulting struct may be sized/unsized depending on type T)

    fn display(boxed: &ExampleRcBox<dyn Display>) { 
        println!("For your enjoyment: {}", &boxed.value) 
    }

    //  <(We cannot create an unsized type directly. Instead, create a sized value that implements the unsized type, and convert the sized reference to an unsized reference)>
    let boxed_lunch: ExampleRcBox<String> = ExampleRcBox { ref_count: 1, value: "lunch".to_string() };
    let boxed_displayable: &ExampleRcBox<dyn Display> = &boxed_lunch;
    //  (this conversion is implicit)

    display(&boxed_lunch);
    display(&boxed_displayable);

    println!("example_sized, DONE");
}

fn example_clone()
{
    //  Clone should construct an independent <deep> copy of self and return it
    //  Only sized types can implement clone

    //  Cloning can be expensive (hence why it must be performed explicitly)

    //  'clone_from()' permits optimisations that may not be possible with 'clone'

    //  Defintion:
    trait ExampleClone: Sized {
        fn clone(&self) -> Self;
        fn clone_from(&mut self, source: &Self) {
            *self = source.clone();
        }
    }

    //  Clone can be automatically implemented for a given type
    #[derive(Clone)]
    struct MyComplex<T> {
        re: T, im: T,
    }

    //  Types for which <deep copying> is meaningful should implement clone

    println!("example_clone, DONE");
}

fn example_copy()
{
    //  <(copy vs clone)>
    //  copy is a marker trait for types which can be shallow <bitwise> copied 
    //  A type that implements 'drop' cannot be copyable

    //  'std::marker::Copy'
    trait ExampleCopy: Clone { }

    //  Copy can be automatically implemented for a given type (if it is Cloneable)
    #[derive(Clone,Copy)]
    struct MyComplex<T> {
        re: T, im: T,
    }

    //  <(Later changing whether a type is copyable is problematic)>

    //  <(A type should be made copy if <>)>

    println!("example_copy, DONE");
}


fn example_Deref_DerefMut()
{
    use std::ops::{Deref,DerefMut};
    use std::fmt::Display;
    //  The dereferencing operators <are/include> '*' / '.'
    //  We can specify how these operate through the traits 'std::ops::Deref' / 'std::ops::DerefMut'

    //  Definition:
    trait ExampleDeref {
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }
    trait ExampleDerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }
    //  ('self' remains borrowed for as long as the returned reference lives

    //  <(deref coercions: if inserting a deref would prevent a type mis-match, Rust inserts one for you)>
    //  eg:
    //      r.find('?')         (*r).find('?')

    struct Selector<T> {
        elements: Vec<T>,
        current: usize,
    }
    impl<T> Deref for Selector<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.elements[self.current]
        }
    }
    impl<T> DerefMut for Selector<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.elements[self.current]
        }
    }

    let mut s = Selector { elements: vec!['x','y','z',], current: 2 };
    assert_eq!(*s, 'z');
    *s = 'w';
    assert_eq!(s.elements, ['x','y','w']);

    //  deref coercion:
    assert!(s.is_alphabetic());

    //  (Do not implement Deref/Derefmut for a type just to make another types methods visible)

    let mut s = Selector { elements: vec!["abc", "def", "hij"], current: 1 };

    //  <(type coercion does not work for generic functions)>:
    fn show_it(thing: &str) { println!("{}", thing); }
    fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }
    show_it(&s);
    //show_it_generic(&s);              //  <(error)>
    show_it_generic(&s as &str);


    println!("example_Deref_DerefMut, DONE");
}

fn example_default()
{
    //  'std::default::Default' is for types that have an obvious default value
    use std::default::Default;
    use std::collections::HashSet;

    //  Definition:
    trait ExampleDefault {
        fn default() -> Self;
    }

    impl ExampleDefault for String {
        fn default() -> Self {
            Self::new()
        }
    }

    //  For Rust's default container types, Default returns an empty collection
    //  <>

    //  If type T implements Default, then Default for the following is implemented automatically:
    //  Rc<T>, Arc<T>, Box<T>, Cell<T>, RefCell<T>, Cow<T>, Mutex<T>, RwLock<T>

    //  If all types of a tuple type implement Default, then that tuple type automatically does as well

    //  If all fields of struct implement Default, then Default can be derived for that struct
    #[derive(Default)]
    struct Foo { a: i32, b: f32, d: bool, }

    //  The default of Option<T> is None

    println!("example_default, DONE");
}

fn example_AsRef_AsMut()
{
    use std::fs::File;
    use std::path::Path;
    use std::io;

    //  When a type implements AsRef<T>, then &T can be borrowed from it efficently
    //  (same for AsMut<T> and &mut T)

    //  Definition:
    trait ExampleAsRef<T: ?Sized> {
        fn as_ref(&self) -> &T;
    }
    trait ExampleAsMut<T: ?Sized> {
        fn as_mut(&mut self) -> &mut T;
    }

    //  AsRef allows functions to be more flexible in the argument types they accept
    fn ExampleOpen<P: AsRef<Path>>(path: P) -> Result<File,io::Error> { 
        File::open(path)
    }
    //  (anything that implements AsRef<Path> (which includes String and str))
    let p: &Path = "/tmp/abc.txt".as_ref();
    println!("p=({:?})", p);

    //  If, for any types T and U, if T implements AsRef<U>, then so does &T

    //  String provides AsRef<u8>, but not AsMut<u8> (as there is no way to guarantee invalid characters aren't written)

    //  Avoid defining an 'AsFoo' trait when 'AsRef<Foo>' is adequate

    fn RecieveBytes<T: AsRef<[u8]>>(b: T) {
        let x: &[u8] = b.as_ref();
        println!("x=({:?})", x);
    }
    let b1 = b"abc";
    let b2: Vec<u8> = vec![1,2,3];
    RecieveBytes(b1);
    RecieveBytes(b2);

    println!("example_AsRef_AsMut, DONE");
}

fn example_Borrow_BorrowMut()
{
    //  When a type implements 'std::borrow:Borrow', then its borrow method efficently borrows &T from it
    //  A type should only implement Borrow<T> when &T hashes/compares the same way as it
    //  <(Rust *doesn't* enforce this)>

    //  Every type T automatically implements Borrow<T>

    //  String implements Borrow<&str>, but not Borrow<[u8]> or Borrow<Path>
    //  (since only &str has the same hashing/comparison behaviour as String)

    //  Definition:
    trait ExampleBorrow<T: ?Sized> {
        fn borrow(&self) -> &T;
    }
    trait ExampleBorrowMut<T: ?Sized> {
        fn borrow_mut(&mut self) -> &mut T;
    }

    //  HashMap allows type Q, from which K can be borrowed, to be used as a key
    //impl ExampleHashMap<K,V>
    //    where K: Eq + Hash 
    //{
    //    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    //        where K: Borrow<Q>
    //    {
    //    }
    //}

    //  All standard library types use Borrow to decide which types can be passed to their lookup functions
    //      Vec<T> and [T: N] implement Borrow<[T]>
    //      String implements Borrow<str>
    //      <>

    println!("example_Borrow_BorrowMut, DONE");
}


fn example_From_Into()
{
    //  'std::convert::From' and 'std::convert::Into' represent conversions that consume one a value of one type and return a value of another 
    //  This conversion may or may not be efficent: because they take ownership of the origional variable, that variable can be used to create the converted type.
    //  (they are used only for conversions that cannot fail (those should return a Result type))

    //  Every type T automatically implements Into<T> and From<T>

    //  Definition:
    trait ExampleInto<T>: Sized {
        fn into(self) -> T;
    }
    trait ExampleFrom<T>: Sized {
        fn from(val: T) -> Self;
    }

    //  <(This is the generic (default) impementation for 'Into' in terms of 'From'(?) (and it is incomplete) ... (this is implicit?))> 
    //impl<T,U> Into<U> for T where U: From<T> {
    //  <...>
    //}

    //  'Intro' is used to make functions more flexible in the types they accept
    use std::net::Ipv4Addr;
    fn ping<T>(address: T) -> std::io::Result<bool>
        where T: Into<Ipv4Addr> 
    {
        let ipv4 = address.into();      //  T only supports one Into<> type -> result is unambiguous
        println!("ping ipv4=({})", ipv4);
        Ok(true)
    }
    ping( Ipv4Addr::new(66,146,219,98) );
    ping( [66,146,219,98] );
    ping( 0xd076eb94_u32 );

    let addr1 = Ipv4Addr::from( [66,146,219,98] );

    struct Foo { a: i32, };
    impl From<i32> for Foo {
        fn from(a: i32) -> Self {
            Foo { a }
        }
    }

    //  Single argument constructors should be written as implementations of From<>
    let f1 = Foo::from(53);

    //  <(implementing From<T> also provides To<T>)>
    let f2: Foo = 53.into();

    println!("example_From_Into, DONE");
}


fn example_ToOwned()
{
    use std::borrow::Borrow;

    //  'std::clone::Clone' by definition returns T when invoked on &T
    //  'std::borrow::ToOwned' provides a similar conversion, but allows the return type to be anything that can be Borrowed from T
    //  ('ToOwned' is a generalization of 'Clone' to borrowed data)

    //  Definition:
    trait ExampleToOwned {
        type Owned: Borrow<Self>;
        fn to_owned(&self) -> Self::Owned;
    }

    let a1: &[i32] = &[1,2,3,4];
    let v1: Vec<i32> = a1.to_owned();

    let s1: &str = "asdf";
    let S1: String = s1.to_owned();

    //  <(Implement for custom type)>
    //  <>

    println!("example_ToOwned, DONE");
}


fn example_Cow_Borrow_ToOwned_And_Ownership()
{
    use std::borrow::Cow;
    use std::borrow::Borrow;
    use std::path::PathBuf;

    //  Cow: clone on write (allows borrowing or ownership (by-ref and by-val))
    //  Cow<B> either borrows a shared reference to B, or owns a value from which we can borrow such a reference.

    //  It contains a reference to an existing object, until we try and modify that existing object at which point it is copied and all subsiquent references are to the copy.

    //  Deref<Cow> returns '&Cow::Owned' if it exists, otherwise 'Cow::Borrowed'
    //  DerefMut<Cow> returns a mutable reference to Cow::Owned
    //  Upon calling DerefMut, if the value is Cow::Borrowed, we create 'Cow::Owned = Cow::Borrowed::to_owned()' 

    //  Definition:
    enum ExampleCow<'a, B: ?Sized + 'a>
        where B: ToOwned
    {
        Borrowed( &'a B ),
        Owned( <B as ToOwned>::Owned ),
    }

    //  <(Use with builtin types)>
    let x: Cow<str> = "asdf".into();

    struct Foo { a: i32, b: i32, c: i32, };
    impl ToOwned for Foo {
        type Owned = Foo;
        fn to_owned(&self) -> Self::Owned {
            Foo { a: self.a, b: self.b, c: self.c, }
        }
    }
    //  {{{
    //  <(no?)>
    //impl From<&Foo> for Foo {
    //    fn from(a: &Foo) -> Self {
    //        Foo { a: a.a, b: a.b, c: a.c, }
    //    }
    //}
    //  }}}
    //  <(what is missing for (see below) to work?)>
    let y: Cow<Foo> = Foo { a: 5, b: 7, c: 9 }.into();


    //  <(Book example:)>
    //fn describe(error: &Error) -> Cow<'static, str> {
    //    match *error {
    //        Error::OutOfMemory => "out of memory".into(),
    //        Error::StackOverflow => "stack overflow".into(),
    //        Error::MachineOnFire => "machine on fire".into(),
    //        Error::Unfathomable => "machine bewildered".into(),
    //        Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
    //    }
    //}
    //println!("Disaster has struck: {}", describe(&error));
    //let mut log: Vec<String> = Vec::new(); 
    //log.push(describe(&error).into_owned());

        println!("example_Cow_Borrow_ToOwned_And_Ownership, DONE");
    }


fn main() 
{
    example_drop();
    example_sized();
    example_clone();
    example_copy();
    example_Deref_DerefMut();
    example_default();
    example_AsRef_AsMut();
    example_Borrow_BorrowMut();
    example_From_Into();
    example_ToOwned();
    example_Cow_Borrow_ToOwned_And_Ownership();
}

