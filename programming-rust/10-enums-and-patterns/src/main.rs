//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-09T00:50:01AEDT (how much I hate) 'constructor' as the name for 'thing from an enum'
//  Ongoing: 2022-10-09T00:53:16AEDT (Unlike the book) we use 'OK' instead of 'Ok' for <an/our> enum value</constructor> [...] (which is Ok(?) (but the syntax highlighting picks up on it))
//  Ongoing: 2022-10-09T00:56:50AEDT book advises (in previous chapter) to not import <functions/methods> directly, but instead import one the module one level above them and use that, i.e: 'use std::mem; mem::size_of();', but then does not-that with (among other examples) size_of
//  Ongoing: 2022-10-09T01:02:24AEDT why 'TimeUnit' only implements PartialEq
//  Ongoing: 2022-10-09T01:15:15AEDT returning &'static str as opposed to String(?) (returning a string constant?)
//  Ongoing: 2022-10-09T01:16:13AEDT what's the disgusting hackey way to do what TimeUnit::singular (get the name of each enum as a string) does? (without the verbosity/tedium)
//  Ongoing: 2022-10-11T23:16:04AEDT expressions produce values, patterns consume values -> (value getting 'consumed' (moved) into/by a match expression) (what about match expression by-reference?)
//  Ongoing: 2022-10-11T23:21:31AEDT it is possible to have an unreachable match case, with no compiler warning (consider 'rough_time_to_english', placing the (units,1) case after the (units,count) case)
//  Ongoing: 2022-10-11T23:24:37AEDT pattern items, ranges, (with 3 dots?)
//  Ongoing: 2022-10-11T23:26:41AEDT pattern matching, ref variable (is 'ref varname' not '&varname'?)
//  Ongoing: 2022-10-11T23:56:29AEDT Rust not giving us warnings about unreachable cases in match expressions?
//  Ongoing: 2022-10-12T00:05:19AEDT 'existing variables can't be used in patterns' -> except where they can be, eg: 'describe_point' (where we use Equal/Greater/Less from std::cmp::Ordering::*) 
//  Ongoing: 2022-10-12T00:35:35AEDT example_reference_patterns, matching '&StringHolder' versus matching 'StringHolder' (where we use them on a reference to a StringHolder and they both behave the same?)
//  Ongoing: 2022-10-12T01:17:06AEDT 'impl<T: Ord>' -> (does this mean) implementation is only for Ord (orderable) things(?)
//  Ongoing: 2022-10-12T01:20:09AEDT match guard if statement must succeeds (book says) -> except we use if/else (with else not being a fail) for BinaryTree::add?
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]
use rand::Rng;
//  Continue: 2022-10-12T01:07:07AEDT example_at_patterns, (on the use of 'x @ pattern')
//  Continue: 2022-10-12T01:14:55AEDT example_patterns_other_uses, implement examples
//  Continue: 2022-10-12T01:27:36AEDT clarify <()>'s

//  Enums are otherwise known as sum-types, discriminated-unions, or algebraic-datatypes
//  Traditional Enums provide named constants

//  <(Rust enums provide an either/or type)>
//  Rust allows Enums of containing multiple types
//  (while remaining type safe)
//  These enums are useful whenever a value might be either one thing or another

//  There are three kinds of enum item:
//      <(numeric values)>
//      tuple items
//      struct items
//  A single enum can have items of all 3 kinds

//  All constructors/fields of a public enum are automatically public

//  <(expressions produce values, patterns consume values)> <(expressions and patterns are natural opposites)>

fn example_C_style_enums()
{
    //  In a C-style enum, values are stored using the smallest integer type that will contain them
    //  We can override this type with a '[#repr]' attribute

    //  Each value is known as a variant or a constructor
    //enum Ordering {
    //    Less, 
    //    Equal, 
    //    Greater,
    //}
    //  This enum above is part of the standard library
    //use std::cmp::Ordering;
    //use std::cmp::Ordering::*;

    enum Pet {
        Orca,
        Giraffe,
        Camel, 
    }
    let p1 = Pet::Orca;

    //  Use a self-import to import the <constructors> values of an enum in the current module
    //use self::Pet::*;         //  <(need to be in namespace/crate/module/<?>)>
    use Pet::*;
    let p2 = Orca;

    //  We can supply custom values, otherwise Rust will assign values beginning at 0
    enum HttpStatus {
        OK = 200,
        NotModified = 304,
        NotFound = 404,
    }

    use std::mem::size_of;
    assert_eq!(size_of::<Pet>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    //  Casting a C-style enum value to an integer is allowed
    //  (even where the cast results in an <underflow>) <(in which case we don't even get a warning?)>
    assert_eq!(HttpStatus::OK as i32, 200);

    //  Casting an integer to an enum is not allowed
    //  (Rust guarantees that an enum is only ever one of the values in the declaration)
    //  (Write a checked conversion function if needed as an alternative) (or use the 'enum_primitive' crate)
    fn HttpStatus_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::OK),
            304 => Some(HttpStatus::NotModified),
            404 => Some(HttpStatus::NotFound),
            _   => None,
        }
    }

    //  As with structs, the compiler can implement features</traits> copy and compare if told to do so:
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years,
    }
    //  Enums <(like all types)> can have methods:
    impl TimeUnit {
        fn singular(self) -> &'static str {
            match self { TimeUnit::Seconds => "second", TimeUnit::Minutes => "minute", TimeUnit::Hours => "hour", TimeUnit::Days => "day", TimeUnit::Months => "month", TimeUnit::Years => "year", }
        }
    }

    println!("example_C_style_enums, DONE");
}


fn example_enums_with_data()
{
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years,
    }
    impl TimeUnit {
        fn singular(self) -> &'static str {
            match self { TimeUnit::Seconds => "second", TimeUnit::Minutes => "minute", TimeUnit::Hours => "hour", TimeUnit::Days => "day", TimeUnit::Months => "month", TimeUnit::Years => "year", }
        }
    }

    //  Enum item Tuple Variants:
    #[derive(Copy,Clone,Debug,PartialEq)]
    enum RoughTime {
        Past(TimeUnit, u32),
        Now,
        Future(TimeUnit, u32),
    }
    let fourScoreAndSevenYearsAgo = RoughTime::Past(TimeUnit::Years, 4*20+7);
    let justNow = RoughTime::Now;
    let threeHoursFromNow = RoughTime::Future(TimeUnit::Years, 3);

    struct Point3d {
        x: f32, y: f32, z: f32,
    }
    impl Point3d {
        fn new(x: f32, y: f32, z: f32) -> Point3d {
            Point3d { x, y, z }
        }
    }

    //  Enum item Struct Variants:
    enum Shape {
        Sphere { center: Point3d, radius: f32 },
        Cuboid { c1: Point3d, c2: Point3d },
    }
    let unit_sphere = Shape::Sphere { center: Point3d::new(0.0,0.0,0.0), radius: 1.0 };

    println!("example_enums_with_data, DONE");
}

//  Enums in memory:
//  consist of a small integer, 'tag', denoting which item it is, plus enough memory to hold the largest kind of item
//  (however, Rust makes no guarantees about enum layout)

fn example_richDataStructures_usingEnums()
{
    use std::collections::HashMap;

    //  Enums can be used as a simple element for creating tree structures:
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String,Json>>),
    }
    //  (This is a considerably simpler solution than an equivalent C++ class)

    println!("example_richDataStructures_usingEnums, DONE");
}

fn example_generic_enums()
{
    //  Enums can be generic:
    enum MyOption<T> {
        None,
        Some(T),
    }
    enum MyResult<T,E> {
        Ok(T),
        Err(E),
    }

    //  Enums can be used to build datastructures with only a few lines of code:
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }
    struct TreeNode<T> {
        val: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    impl<T: Ord> BinaryTree<T> {
        fn add(&mut self, val: T) {
            match *self {
                BinaryTree::Empty => 
                    *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        val,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    })),
                BinaryTree::NonEmpty(ref mut node) =>
                    if val <= node.val {
                        node.left.add(val)
                    } else {
                        node.right.add(val)
                    }
            }
        }
    }

    //  <(Usage: (without impl method))>
    use BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        val: "Jupiter",
        left: Empty,
        right: Empty,
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode {
        val: "Mars",
        left: jupiter_tree,
        right: Empty,
    }));

    println!("example_generic_enums, DONE");
}

struct Point3d {
    x: f32, y: f32, z: f32,
}
impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d { x, y, z }
    }
}
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { c1: Point3d, c2: Point3d },
}

fn example_enums_and_patterns()
{
    //  The downside to enums is that we cannot access fields only present for some items directly:
    let s = Shape::Sphere { center: Point3d::new(0.0,0.0,0.0), radius: 1.0 };
    //let r = s.radius;         //  error, no field 'radius' in enum

    //  The way to access fields of an enum value is with a match expression:
    //  (recall that a match expression will ensure all cases are covered)
    fn get_shape_radius(s: Shape) -> Option<f32> {
        match s {
            Shape::Sphere {center, radius} => Some(radius),
            Shape::Cuboid {c1, c2} => None,
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years,
    }
    impl TimeUnit {
        fn singular(self) -> &'static str {
            match self { TimeUnit::Seconds => "second", TimeUnit::Minutes => "minute", TimeUnit::Hours => "hour", TimeUnit::Days => "day", TimeUnit::Months => "month", TimeUnit::Years => "year", }
        }
    }
    #[derive(Copy,Clone,Debug,PartialEq)]
    enum RoughTime {
        Past(TimeUnit, u32),
        Now,
        Future(TimeUnit, u32),
    }
    let fourScoreAndSevenYearsAgo = RoughTime::Past(TimeUnit::Years, 4*20+7);
    let justNow = RoughTime::Now;
    let threeHoursFromNow = RoughTime::Future(TimeUnit::Years, 3);

    fn rough_time_to_english(rt: &RoughTime) -> String {
        match rt {
            RoughTime::Past(units,1) => format!("1 {} ago", units.singular()),
            RoughTime::Past(units,count) => format!("{} {}s ago", count, units.singular()),
            RoughTime::Now => format!("just now"),
            RoughTime::Future(units,1) => format!("1 {} from now", units.singular()),
            RoughTime::Future(units,count) => format!("{} {}s from now", count, units.singular()),
        }
    }
    println!("{}", rough_time_to_english(&fourScoreAndSevenYearsAgo));
    println!("{}", rough_time_to_english(&justNow));
    println!("{}", rough_time_to_english(&threeHoursFromNow));

    let oneSecondFromNow = RoughTime::Future(TimeUnit::Seconds, 1);
    println!("{}", rough_time_to_english(&oneSecondFromNow));

    println!("example_enums_and_patterns, DONE");
}


fn example_pattern_rules()
{
    //  Things that can be used in patterns:
    //      literals        "name"
    //      range           0...100             match any value in range (including end value)
    //      wildcard        _                   match any value and ignore it
    //      variable        varname             Like _, but move/copy the value into new variable
    //      ref variable    ref varname         Borrow reference to the matched value
    //      enum pattern    Some(value)
    //      tuple pattern   (key, value)
    //      struct pattern  Color(r,g,b)
    //      reference       &value
    //      multiple patterns           'a' | 'A'       (In match only, not let, ect.)
    //      guard expressions           x if x*x <= r2  (In match only, not let, ect.)
    //      binding with subpattern     val @ 0...9     Match pattern to right of '@' using variable name to the left

    //  A match with integer values is equivalent to a C-switch statement
    //  Use a fallback wildcard that panics for cases we are sure cannot occur
    let x = rand::thread_rng().gen_range(0..5);
    let x_str = match x { 
        0 => "zero", 
        1 => "one", 
        2 => "two", 
        3 => "three", 
        4 => "four", 
        other => panic!("Invalid value, other=({})", other), 
    };
    println!("x=({}), x_str=({})", x, x_str);

    //  <(alternatively)>
    let x = rand::thread_rng().gen_range(0..5);
    let x_str = match x { 
        0 => Some("zero"), 
        1 => Some("one"), 
        2 => Some("two"),
        3 => Some("three"), 
        4 => Some("four"), 
        _ => None, 
    }.unwrap();
    println!("x=({}), x_str=({})", x, x_str);

    //  Existing variables can't be used in patterns
    //  (attempting to do so creates new variable, shaddowing existing one, introducing bug the compiler will not warn us about)
    let x = rand::thread_rng().gen_range(0..5);
    let zero = 0;
    let x_str = match x { 
        zero => "zero",         //  incorrect, creates new variable, will match anything '_' would
        1 => "one", 
        2 => "two", 
        3 => "three", 
        4 => "four", 
        other => panic!("Invalid value, other=({})", other), 
    };
    assert_eq!(x_str, "zero");

    //  Instead, to use variables in a match expresssion, use an if statement
    let x = 0;
    let zero = 0;
    let x_str = match x { 
        1 => "one", 
        2 => "two", 
        3 => "three", 
        4 => "four", 
        other => 
            if other == zero {
                "zero"
            } else {
                panic!("Invalid value, other=({})", other) 
            }
    };
    assert_eq!(x_str, "zero");

    println!("example_pattern_rules, DONE");
}

struct Point2d { x: i32, y: i32, }
struct Account { name: String, id: i32, address: String, history: Vec<String>, }

fn example_tuple_and_struct_patterns()
{
    //  Tuple patterns match tuples
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the orign",
            (_, Equal) => "on x axis",
            (Equal, _) => "on y axis",
            (Greater, Greater) => "1st quadrant",
            (Less, Greater) => "2nd quadrant",
            _ => "somewhere else",
        }
    }

    //  Struct patterns match structs:
    //  (the elements of a struct do not need to be in order)
    fn describe_Point2d(p: Point2d) {
        match p {
            Point2d { x: 0, y: h } => 
                println!("straight up, h=({})", h),
            Point2d { x, y } =>                         //  shorthand for { x: x, y: y }
                println!("at ({},{})", x, y),
        }
    }

    //  Use '..' to tell Rust we don't care about any other fields
    fn describe_account(a: Account) {
        match a {
            Account { id: 1, name, .. } => println!("account #1"),
            Account { name, .. } => println!("other account"),
        }
    }

    println!("example_tuple_and_struct_patterns, DONE");
}


fn example_reference_patterns() 
{
    //  Matching on a noncopyable value moves the value
    //  Use the 'ref' keyword to borrow the matched values instead of moving them
    fn describe_account(a: &Account) {
        match a {
            Account { id: 1, ref name, .. } => println!("account #1"),
            Account { ref name, .. } => println!("other account"),
        }
    }
    //  (Use 'ref mut' to borrow a mutable reference)

    struct StringHolder { s: String };

    let mut s = StringHolder { s: "asdf".to_string(), };
    let ps = &s;

    //  To match a reference, prefix the pattern with '&'
    match ps {
        &StringHolder{ ref s } => println!("s=({})", s),
    }
    match ps {
        StringHolder{ ref s } => println!("s=({})", s),
    }
    //  <(difference between (see above)?)>
    //  <(match automatically dereferences?)>

    //  Matching 'Some(&var)':
    //match chars.peek() {
    //    Some(&c) => println!("coming up: {:?}", c),
    //    None => println!("end of chars"),
    //}

    println!("example_reference_patterns, DONE");
}

fn example_match_multiple_possibilities()
{
    //  '|' can be used to combine several patterns:
    //let at_end = match chars.peek() {
    //    Some(&'\r') | Some(&'\n') | None => true,
    //    _ => false,
    //};

    //  Ranges can be used in pattern matching:
    //  '..' defines an open range (end is excluded)
    //  '...' defines a closed range (begin/end are both included)
    //  <('...' ranges are deprecated?)> 
    //  <(use '..=' instead?)>

    let next_char = '9';
    let kind_char = match next_char {
        '0' ..= '9' => "number",
        'a' ..= 'z' | 'A' ..= 'Z' => "letter",
        ' ' | '\t' | '\n' => "whitespace",
        _ => "other",
    };
    assert_eq!(kind_char, "number");

    println!("example_match_multiple_possibilities, DONE");
}

fn example_pattern_guards()
{
    //  Use 'if' to add a guard to a match arm.
    //  The match succeedes only if the guard condition evaluates to true
    //  <(panics if guard evaluates to false?)>
    //  <(but it doesn't (see BinaryTree::add)?)>

    //  If a pattern moves any values, a guard if cannot be used.
    //  (for non-copyable types, use 'Some(ref var)')>

    println!("example_pattern_guards, DONE");
}


fn example_at_patterns()
{
    //  'x @ pattern' creates a variable 'x' on success, and moves/copies the whole value into it
    //  <(this is to avoid using multiple variables on matching a struct?)>
    //  <>
    println!("example_at_patterns, DONE");
}


fn example_patterns_other_uses()
{
    //  Pattern matching features that have other uses:

    //  Unpack a struct
    //let Track { album, track_number, title, .. } = song;

    //  Unpack a tuple function argument
    //fn distance_to((x,y): (f64,f64)) -> f64 { ... }

    //  Iterate over k,v pairs of a HashMap
    //for (k,v) in &values_map {
    //}

    //  Automatically dereference an argument to a closure
    //let sum = numbers.fold(0, |a, &num| a + num)

    //  Only irrefutable patterns can be used for these purposes.
    //  (An irrefutable pattern is a pattern that is guaranteed to match)

    //  Handle just one enum variant
    //if let RoughTime::Future(_,_) = user.date_of_birth() {
    //}

    //  Run code only if lookup succeeds
    //if let Some(document) = cache_map.get(&id) {
    //}

    //  Repeat until something succeeds
    //while let Err(err) = do_thing() {
    //}

    //  Manually loop over iterator
    //while let Some(_) = lines.peek() {
    //}

    println!("example_patterns_other_uses, DONE");
}

//  Rust enums serve the role of C unions (which are so unsafe they should be used as a last resort)
//  They allow easy handling of cases where a value may be one type or another
//  (One downside is flexibility: adding new variants to an enum will likely require rewriting of the code that uses it)
//  (They are therefore best used for types that are not likely to change)
//  (Rust provides traits for where more flexibility is needed)

fn main() 
{
    example_C_style_enums();
    example_enums_with_data();
    example_richDataStructures_usingEnums();
    example_generic_enums();
    example_enums_and_patterns();
    example_pattern_rules();
    example_tuple_and_struct_patterns();
    example_reference_patterns();
    example_match_multiple_possibilities();
    example_pattern_guards();
    example_at_patterns();
    example_patterns_other_uses();
}

