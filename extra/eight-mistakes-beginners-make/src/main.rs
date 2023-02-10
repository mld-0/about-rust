//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2023-02-09T22:44:03AEDT when are enums Copy?
//  Ongoing: 2023-02-09T22:44:43AEDT match 'or' uses single '|'?
//  Ongoing: 2023-02-09T23:33:38AEDT 'array_window()' <of/over> N elements?
//  Ongoing: 2023-02-10T21:56:23AEDT 'open_and_parse_file()' -> what is the idomatic way to handle multiple error types and the '?' operator?
//  Ongoing: 2023-02-10T22:01:30AEDT implicit conversion (when does 'std::convert::From' get used implicitly?)
//  Ongoing: 2023-02-10T22:08:29AEDT rewriting 'Point::from_str()' to have more meaningful error messages for incorrect input ... (what is the current error for that thing you gave me doesn't have enough components when split by ',' ... given we use 'let y = coords[1].parse::<i32>()?' are we handling it at all (does out of bounds exception get propogated by '?'?))
//  Ongoing: 2023-02-10T22:12:54AEDT Need to 'use std::str::FromStr' before we can call 'Point::from_str()' -> anyway to call it without the former?
//  Ongoing: 2023-02-10T23:27:51AEDT (lambdas and) keyword 'move'?
//  Ongoing: 2023-02-10T23:42:46AEDT cleaner alternative to 'self.callback.as_ref().unwrap()(damage_recieved)'?
//  Ongoing: 2023-02-10T23:46:06AEDT clippy has no suggestions for either of our Monsters examples?
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//  Allow 'array_windows()'
#![feature(array_windows)]

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

//  Continue: 2023-02-09T23:07:32AEDT complete 'Error' implementation for custom error (manually / "This Error" crate)

//  8 deadly mistakes beginner Rust developers make
//  LINK: https://www.youtube.com/watch?v=PbR4ECFIckg 

//  (Blog post from which video is appropriated)
//  LINK: https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/
//  {{{

//  Avoid hungarian notation:
//  (don't be afraid to use shadowing / reuse varnames when converting same data between types)
//  (contention: shadowing isn't a problem in statically typed languages)

//  Don't use wrong integer type:
//  (Rust <strongly> prefers usize over isize for indexing (and so should you))

//  Unsafe:
//  Not a magic escape hatch to allow C programs with rust syntax
//  Should not be used without a good understanding of Rust's memory model

//  Not using namespaces:
//          Bad         rune_wasmer_runtime_load()
//          Good        rune::wasmer::Runtime::load()

//  Overusing slice indexing:
//  Creates opertunities for mistakes
//  Iterators can oftern be better optimized

//  Overusing iterators:
//  Sometimes, indexing makes for a more readable solution

//  Initalize during construction:
//  The constructor output should be a usable object - make invalid states unrepresentable
//  Bad:
//          let mut dict = Dictionary::new()
//          dict.load_from_file("./words.txt")
//  Good:
//          let dict = Dictionary::load_from_file("./words.txt")

//  }}}

//  Common newbie mistakes or bad practices
//  LINK: https://users.rust-lang.org/t/common-newbie-mistakes-or-bad-practices/64821
//  {{{

//  Strings are not paths: use &Path / join() instead of &str / format!("{}/{}",dir,filename)

//  Use: 'fn parse(reader: impl std::io::Read)' instead of 'fn parse(filename: &str)'

//  Arrays coerce to slices, use: 'f(&array)' instead of 'f(&array[..])' 

//  Adding '&Foo' to a struct (and getting stuck in borrow-checker hell) when really the struct should own Foo

//  <(Use the non-panicking counterparts to 'args()' / 'vars()')>

//  Avoid matched trait-struct pairs: 'impl IFoo for Foo'

//  avoid '#[path]'

//  Prefer 'read_exact()' / 'write_all()' to 'Read::read()' / 'Write::write()'

//  Don't put IO resources into mutexes / containers

//  Use 'structopt' (/similar) instead of parsing arguments manually (or even configuring 'clap' manually)

//  <(contention: don't overuse 'From' for type conversions?)>

//  }}}


fn one_unneccessary_indirection()
{
    //  Use '&str' instead of 'String' (the latter coerces to the former)
    //  This is more flexible, as we can now accept both String and &str arguments
    fn fancy_print_i(s: String) {
        println!("s=({})", s);
    }
    fn fancy_print_ii(s: &str) {
        println!("s=({})", s);
    }

    println!("{}, DONE", get_func_name!());
}

fn two_overuse_slice_indexing()
{
    let points = vec![5,9,7,4,2,6];
    println!("points=({:?})", points);

    //  Using C-style indexing
    let mut deltas = vec![];
    for i in 1..points.len() {
        let current = points[i];
        let previous = points[i-1];
        let delta = current - previous;
        deltas.push(delta);
    }
    println!("deltas=({:?})", deltas);

    //  slice.array_windows()
    //  (unstable) Returns an iterator over overlapping windows of N elements
    //  <(use 'copied()' or else it consumes values?)>

    //  Using 'array_windows()' (unstable)
    let mut deltas = vec![];
    for [previous, current] in points.array_windows().copied() {
        let delta = current - previous;
        deltas.push(delta);
    }
    println!("deltas=({:?})", deltas);

    //  Replacing loop with map
    let mut deltas = points.array_windows().copied()
        .map(|[previous,current]| current - previous)
        .collect::<Vec<_>>();
    println!("deltas=({:?})", deltas);

    println!("{}, DONE", get_func_name!());
}

fn three_sentinel_values()
{
    //  Sentinel: value with a special meaning

    //  Eg: returning -1 / "" / Null to indicate failure
    fn find_i(arr: &[i32], val: i32) -> i32 {
        for i in 0..arr.len() {
            if arr[i] == val {
                return i as i32;
            }
        }
        -1
    }

    //  Instead use Option<T>
    fn find_ii(arr: &[i32], val: i32) -> Option<usize> {
        for i in 0..arr.len() {
            if arr[i] == val {
                return Some(i);
            }
        }
        None
    }

    println!("{}, DONE", get_func_name!());
}

fn four_enums_and_pattern_matching()
{
    //  Bad: using String values to indicate options
    fn can_publish_blog_i(role: &str) -> bool {
        if role == "Admin" || role == "Writer" {
            return true;
        } 
        false
    }
    //  Instead use an Enum to encode options
    enum Role {
        Admin, Reader, Writer,
    }
    fn can_publish_blog_ii(r: Role) -> bool {
        match r {
            Role::Admin | Role::Writer => true,
            _ => false,
        }
    }


    //  if statement vs let pattern matching:
    let opt = Some(32);
    if opt.is_some() {
        let value = opt.unwrap();
        //  { ... }
    }
    if let Some(value) = opt {
        //  { ... }
    }


    //  if statement vs let pattern matching:
    let list = vec![1,2,3];
    if !list.is_empty() {
        let first = list[0];
        //  { ... }
    }
    if let [first, ..] = list.as_slice() {
        //  { ... }
    }


    println!("{}, DONE", get_func_name!());
}


fn five_error_handling()
{
    //  Error propagation operator: '?'
    //          let a: i32 = a.parse()?;
    //  is equivalent to
    //          let a = a.parse()
    //          if let Err(e) = a { return Err(e); }
    //          let a = a.unwrap()
    //  (containing function must return the relevant Result type)

    //  Bad: not using '?' operator for error propagation
    fn parse_then_add_i(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let a = a.parse::<i32>();
        if let Err(e) = a { 
            return Err(e);
        }
        let b = b.parse::<i32>();
        if let Err(e) = b {
            return Err(e);
        }
        Ok(a.unwrap() + b.unwrap())
    }
    //  Instead:
    fn parse_then_add_ii(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let a: i32 = a.parse()?;
        let b: i32 = b.parse()?;
        Ok(a + b)
    }


    //  Bad: custom Error types that do not implement 'std::error:Error'
    //  (However) Implementing Error ourselves is tedious:
    #[derive(Debug)]
    pub enum DataStoreError {
        Disconnect(std::io::Error),
        Redaction(String),
        InvalidHeader { expected: String, found: String },
        Unknown,
    }
    impl std::fmt::Display for DataStoreError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
//  impl std::error::Error for DataStoreError {
//      fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//          //  ...
//      }
//      fn cause(&self) -> Option<&dyn std::error::Error> {
//          //  ...
//      }
//  }


    //  recommendation: use "This Error" library
    //  <>

    println!("{}, DONE", get_func_name!());
}

fn sixth_standard_library_traits()
{
    //  'std::default::Default' allows sensible default values to be <specified/created>
    pub trait Eg_Default: Sized {
        fn default() -> Self;
    }

    //  'std::convert::From' define a conversion between types
    //  (also provides 'std::convert::Into')
    pub trait Eg_From<T>: Sized {
        fn from(value: T) -> Self;
    }

    //  'std::convert::TryFrom' defines a conversion between types that can fail
    //  (also provides 'std::convert::TryInto')
    pub trait Eg_TryFrom<T>: Sized {
        type Error;
        fn try_from(value: T) -> Result<Self, Self::Error>;
    }
    

    //  Example: defining a custom error type with conversions to use with '?' operator
    enum CliError {
        IoError(std::io::Error),
        ParseError(std::num::ParseIntError),
    }
    impl From<std::io::Error> for CliError {
        fn from(error: std::io::Error) -> Self {
            CliError::IoError(error)
        }
    }
    impl From<std::num::ParseIntError> for CliError {
        fn from(error: std::num::ParseIntError) -> Self {
            CliError::ParseError(error)
        }
    }
    fn read_int_in_file(filename: &str) -> Result<i32, CliError> {
        let mut content = std::fs::read_to_string(filename)?;
        let num: i32 = content.trim().parse()?;
        Ok(num)
    }


    //  'trim_matches()'
    //  <>


    //  'std::str::FromStr' defines conversion from Strings
    pub trait Eg_FromStr: Sized {
        type Err;
        fn from_str(s: &str) -> Result<Self, Self::Err>;
    }

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl std::str::FromStr for Point {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(",")
                .collect();
            let x = coords[0].parse::<i32>()?;
            let y = coords[1].parse::<i32>()?;
            Ok(Point { x, y, })
        }
    }

    //  Need to bring 'FromStr' into scope to call 'Point::from_str()'
    use std::str::FromStr;
    let p = Point::from_str("(1,2)");
    assert_eq!(p.unwrap(), Point { x: 1, y: 2, });

    println!("{}, DONE", get_func_name!());
}


fn seven_standard_library_macros()
{
    //  'std::todo' indicates unfinished code
    //  (attempting to call results in runtime panic)
    fn unfinished() {
        todo!();
    }


    //  'std::concat' concatnates literals into a static string slice
    let s = concat!("test", 10, 'b', true);
    assert_eq!(s, "test10btrue");


    //  'std::format' creates a String using interpolation of runtime expressions
    let s1 = "hello".to_string();
    let s2 = "world";
    let s3 = format!("{} {}!", s1, s2);
    assert_eq!(s3, "hello world!");

    println!("{}, DONE", get_func_name!());
}


fn eight_tooling()
{
    //  #>$ cargo fmt
    //  Format code, run command in project directory
    //  run within vim: '%! rustfmt'

    //  cargo clippy
    //  Linter, run in project directory
}


fn bonus()
{
    use rand::Rng;
    use std::rc::Rc;
    use std::cell::RefCell;

    //  Rc<RefCell<T>> is a Rust idiom to provide multiple mutable references to something
    //  It can be used to replicate code from other languages, but its use oftern indicates bad design

    //  Bad Design:
    {
        struct Monster {
            health: u32,
            //recieved_damaged: Vec<Box<dyn Fn(u32)>>,
            callback: Option<Box<dyn Fn(u32)>>,
        }
        impl Monster {
            fn take_damage(&mut self, amount: u32) {
                let damage_recieved = std::cmp::min(self.health, amount);
                self.health -= damage_recieved;
                if self.callback.is_some() {
                    self.callback.as_ref().unwrap()(damage_recieved);
                }
            }
            fn add_callback(&mut self, callback: Box<dyn Fn(u32)>) {
                self.callback = Some(callback);
            }
        }
        impl Default for Monster {
            fn default() -> Self {
                Monster { health: 100, callback: None, }
            }
        }
        #[derive(Default)]
        struct DamageCounter {
            damage_inflicted: u32,
        }
        impl DamageCounter {
            fn reached_target_damage(&self) -> bool {
                self.damage_inflicted > 100
            }
            fn on_damage_recieved(&mut self, damage: u32) {
                self.damage_inflicted += damage;
            }
        }

        let mut rng = rand::thread_rng();
        let mut counter = Rc::new(RefCell::new(DamageCounter::default()));
        let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();
        for monster in &mut monsters {
            let counter = Rc::clone(&counter);
            monster.add_callback(Box::new(
                    move |x| counter.borrow_mut().on_damage_recieved(x)
            ));
        }
        println!("Run Monsters:");
        while !counter.borrow().reached_target_damage() {
            let index = rng.gen_range(0..monsters.len());
            let target = &mut monsters[index];
            let damage = rng.gen_range(0..50);
            target.take_damage(damage);
            println!("Monster {} recieved {} damage", index, damage);
        }
    }


    //  Better Design: don't hold long-lived reference to other objects
    {
        struct Monster {
            health: u32,
        }
        impl Monster {
            fn take_damage(&mut self, amount: u32, callback: impl FnOnce(u32)) {
                let damage_recieved = std::cmp::min(self.health, amount);
                self.health -= damage_recieved;
                callback(damage_recieved);
            }
        }
        impl Default for Monster {
            fn default() -> Self {
                Monster { health: 100 }
            }
        }
        #[derive(Default)]
        struct DamageCounter {
            damage_inflicted: u32,
        }
        impl DamageCounter {
            fn reached_target_damage(&self) -> bool {
                self.damage_inflicted > 100
            }
            fn on_damage_recieved(&mut self, damage: u32) {
                self.damage_inflicted += damage;
            }
        }

        let mut rng = rand::thread_rng();
        let mut counter = DamageCounter::default();
        let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();
        println!("Run Monsters:");
        while !counter.reached_target_damage() {
            let index = rng.gen_range(0..monsters.len());
            let target = &mut monsters[index];
            let damage = rng.gen_range(0..50);
            target.take_damage(damage, |x| counter.on_damage_recieved(x));
            println!("Monster {} recieved {} damage", index, damage);
        }
    }
    //  Provide the callback function as argument to 'Monster::take_damage()' instead of trying to store it in 'Monster' objects

}


fn main() 
{
    one_unneccessary_indirection();
    two_overuse_slice_indexing();
    three_sentinel_values();
    four_enums_and_pattern_matching();
    five_error_handling();
    sixth_standard_library_traits();
    seven_standard_library_macros();
    eight_tooling();
    bonus();
}

