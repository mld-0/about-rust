//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-09-13T02:04:10AEST 'closure types have no written form' - how does that work - I can't 'YcmComplete GetType' a variable containing a closure?
//  Ongoing: 2022-09-16T23:08:35AEST coverage of slices <seems> insufficent
//  }}}
#![allow(dead_code)]
#![allow(unused)]

//  Rust types offers safety, efficiency and concision <conciseness?>
//  Rust uses ahead of time compilation: the entire program is compiled before any of it is run.
//  <(The compiler uses efficient, predictable representations of these types)>

//  Rust is statically typed. Every path of excitation is checked for type correctness.
//  We must specify type insofar as it is necessary for every type to be defined. 
//  The return type of a function must be given or else it is '()'. Elsewhere Rust will deduce type where it can.

//  Equivalent:
fn build_vector_i() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v
}
fn build_vector_ii() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v
}

//  Functions can be generic: if the implementation is general enough, it <can> work on a set of types


//  Signed and unsigned integers of given bit widths:
//      i8, i16, i32, i64, isize                            [-2**(n-1), 2**(n-1)-1]
//      u8, u16, u32, u64, usize                            [0, 2**n - 1]

//  Generally we use u8 for 'byte'

//  Single/double float:            f32, f64
//  Boolean (8 bits):               bool
//  Char (32 bits):                 char

//  Tuple (allows mixed types):     (char, u8, i32)
//  Unit (empty tuple):             ()

//  Named-field struct:             struct S {x: f32, y: f32 }
//  Tuple-like struct:              struct T(i32, char)
//  Unit struct:                    struct E

//  Enum:                           enum Attend { OnTime, Late(u32) }

//  Box (pointer to heap):          Box<Attend>

//  Shareable reference:            &i32
//  Shareable mutable reference:    &mut i32

//  String:                         String
//  Reference to string:            &str

//  Fixed length array:             [f64; 4]
//  Vector:                         Vec<f64>

//  Reference to slice:             &[u8]
//  Reference to mutable slice:     &mut [u8]

//  Trait object:                   &Any
//  Trait mutable object:           &mut Any

//  Pointer to function             fn(&str, usize) -> isize
//  Closure                         <no written form>

//  usize is generally used for sizes. Array indices must use this type.


//  Overflow:
//  In debug builds, Rust panics if intger overflow occurs.
//  In release builds, integer overflow is defined behaviour (it wraps).

//  If one desires wrapping overflow behaviour, use the method:
//          big_val.wrapping_add()


//  Integer literals:
//  Type is indicated by suffix: u8/i32/isize/ect.
//  If a type suffix is not given, Rust will infer type from the context, defaulting to i32.
//  Prefixes: 0x = hex, 0o = octal, 0b = binary
//  Undescores can be used for legibility: 4_294_967_295, 0xffff_ffff, 127_u8

//  Byte literals: b'X' 
//  Given ASCII character X as u8
//  Single quote, backslash, newline, carriage return, and tab must all be delimited by '\'
//  Alternatively: b'\xHH' where HH is a hex number 
//  Equivalent: b'A' == 65u8 <(== b'\x41')>


//  Casts:
fn example_casts() {
    //  In-range casts:
    assert_eq!(10_u16,      10_i8 as u16);
    assert_eq!(2525_i16,    2525_u16 as i16);
    assert_eq!(-1_i32,      -1_i16 as i32);         //  sign extended
    assert_eq!(65535_i32,   65535_u16 as i32);      //  zero-extended

    //  Out-of-range casts:
    //  Equivalent to modulo 2**N, where N is the destination width
    assert_eq!(232_u8,  1000_i16 as u8);
    assert_eq!(-1_i16,  65535_u32 as i16);      //  <?>
    assert_eq!(255_u8,  -1_i8 as u8);           //  <?>
    assert_eq!(-1_i8,   255_u8 as i8);          //  <?>
}


//  Integer methods:
fn example_integer_methods() {
    assert_eq!(16,  2_u16.pow(4));
    assert_eq!(4,   (-4i32).abs());
    assert_eq!(4,   0b101101_u8.count_ones());
}


//  Floating point: 
//      f32     at least 6 digits       +/- 3.4 * 10E38
//      f64     at least 16 digits      +/- 1.8 * 10E308
//  Comprised of: integer part, fractional part, expondent, type suffix
//  (Each part is optional, so long as at least 1 non-integer part is present to differentiate it from an integer).
//  If type suffix is missing, Rust infers it from the context, defaulting to f64
//  Rust will not infer a floating point type for an integer literal, or vice versa.
fn example_floating_point() {
    assert_eq!(5.0,     5_f32.sqrt() * 5_f32.sqrt());
    assert_eq!(-1.0,    -1.01_f64.floor());
    assert!((-1.0 / std::f32::INFINITY).is_sign_negative());
}


//  Bool type: true/false
//  A number cannot be used where a bool is expected
//  <>


//  Character: 32 bits, Unicode character
//  Denoted with single quotes
//  ASCII characters can be written as '\xHH' where HH is the hex code 
//  Unicode characters can be written as '\u{HHHHHH}' where HHHHHHH is the hex code (1-6 digits)
fn example_characters() {
    assert_eq!(42,      '*' as i32);
    assert_eq!(0xca0,   'ಠ' as u16);
    assert_eq!(-0x60,   'ಠ' as i8);

    //  character methods
    assert_eq!('*'.is_alphabetic(), false); 
    assert_eq!('β'.is_alphabetic(), true); 
    assert_eq!('8'.to_digit(10), Some(8)); 
    assert_eq!('ಠ'.len_utf8(), 3); 
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}


//  Tuple: A sequence of elements seperated by commas and surrounded by parenthesis
//  Commonly used to return multiple values from a function.
//  Also used to combined related arguments into a single value.
//  Only allows constants as indices
//  An empty tuple '()' is the unit type. This is the default function return type.
fn example_tuples() {
    //  pattern-matching syntax can be used to assign each element of a returned tuple
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    //  Indexed with '.i' where i is a constant
    let values = (5, 9, 7, "asdf", 1_u8);
    assert_eq!(values.0, 5);
}

//  References: Rust's basic pointer type
//  denoted with '&', eg: &String, &i32
//  '&x' borrows a reference to 'x'
//  '*r' refers to the value 'r' points to
//  Immutable by default, can never be null.
//      Rust        C
//      &T          const T*
//      &mut T      T*

//  Boxes: A simple way to allocate a value on the heap
//  When a box goes out of scope, it is freed immediately, unless it has been moved
fn example_boxes() {
    let t = (12, "eggs");
    let _b = Box::new(t);
}


//  Raw pointers
//  Unsafe, can only be dereferenced in an 'unsafe' block.


//  Arrays: [T; N], N values, each of type T
//      Size is a constant determined at compile time
//  Vectors: Vec<T>
//      Dynamically allocated, growable sequence
//  Slices:
//      &[T] shared slice of T-s
//      &mut [T] mutable shared slice of T-s
//  Indexing: 
//      v[0] is the first element
//      v[v.len()-1] is the last element
//      Expression will panic if index falls outside container, or if container is empty
//      Only 'usize' may be used as an index

//  Rust does not allow uninitalized arrays
//  An arrays length is part of its type, and is fixed at compile time.
//  Array length cannot be a variable. Use a vector instead.
fn example_arrays() {
    let lazy_caterer: [u32; 6] = [1,2,4,7,11,16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"]; 
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10_000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    //  Array-related functions are methods of slices, not arrays
    //  But Rust implicitly converts a reference to an array to a slice when searching for methods
    //  So we can call any slice method on an array directly
    let mut chaos: [i32; 5] = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1,2,3,4,5]);
}

//  Vec<T> consists of 3 values: a pointer to a heap-allocated buffer, the capacity of that buffer, and the number of elements in the vector.
//  When a vector needs to grow, the size of the buffer is doubled
fn example_vectors() {
    //  The simplest way to create a vector is the 'vec!' macro
    //  With a list of values
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a,b| a*b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a,b| a*b), 30030);

    //  Repeat a value for a given length
    let pixel_buffer = vec![0; 1024 * 1024];
    assert_eq!(pixel_buffer.len(), 1024 * 1024);

    //  We can build a vector with the values from an iterator
    let v2: Vec<i32> = (0..5).collect();
    assert_eq!(v2, [0,1,2,3,4]);

    //  <(Rust implicitly converts a reference to a vector to a slice when searching for methods)>
    //  So we can call any slice method on a vector
    let mut v3 = vec!["a man", "a plan", "a canal", "panama"];
    v3.reverse();
    assert_eq!(v3, vec!["panama", "a canal", "a plan", "a man"]);

    //  Use 'Vec::with_capacity()' to create a vector with a pre-allocated buffer of a given size
    let mut v4 = Vec::with_capacity(2);
    assert_eq!(v4.len(), 0);
    assert_eq!(v4.capacity(), 2);
    v4.push(1);
    v4.push(2);
    assert_eq!(v4.len(), 2);
    assert_eq!(v4.capacity(), 2);
    v4.push(3);
    assert_eq!(v4.len(), 3);
    assert_eq!(v4.capacity(), 4);

    //  The insert and remove methods will be slow for a long vector
    let mut v5 = vec![10, 20, 30, 40, 50];
    v5.insert(3, 35);
    assert_eq!(v5, [10,20,30,35,40,50]);
    v5.remove(1);
    assert_eq!(v5, [10,30,35,40,50]);

    //  The last element can be retrieved with 'pop()', which returns Option<T>
    let mut v6 = vec!["carmen", "miranda"];
    assert_eq!(v6.pop(), Some("miranda"));
    assert_eq!(v6.pop(), Some("carmen"));
    assert_eq!(v6.pop(), None);

    //  Use a loop to iterate over a vector
    //let languages: Vec<String> = std::env::args().skip(1).collect();      //  from cli arguments
    let languages: Vec<String> = vec!["Lisp".to_string(), "Scheme".to_string(), "C".to_string(), "C++".to_string(), "Fortran".to_string()];
    for l in languages {
        println!("{}: {}", l, 
                 if l.len() % 2 == 0 {
                     "Functional" 
                 } else {
                     "Imperative"
                 });
    }
}

//  A slice, [T], is a region of an array or vector
//  <(Since a slice can be any length, they can't be stored directly in variables or passed as function arguments (is this the slice/reference-to-slice distinction?))>
//  They are always passed by reference.
//  A reference to a slice is a fat pointer - containing a pointer to the first element and number of elements in the slice
fn example_slices() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let _sa: &[f64] = &a;
    //  Function which operates on slice can recieve array or vector
    print_elements(&v);
    print_elements(&a);
    //  <(You can get a reference to a slice of an array or vector, or a slice of an existing slice, by indexing it with a range)>
    print_elements(&v[0..2]);
    print_elements(&a[2..]);
    print_elements(&sv[1..3]);
    //  &[T] and &str are references to slices
    //  Since slices almost always appear behind references, references to slices are oftern refered to simply as slices
}
fn print_elements(n: &[f64]) {
    print!("n.len()=({}): ", n.len());
    for x in n {
        print!("{}, ", x);
    }
    println!("");
}


//  Rust strings are unicode text.
//  Rust guarantees that strings are valid UTF-8.
//  &str is analogous to &[T]
//  String is analogous to Vec<T>
//  Use '&str' for arguments that allow either kind of string
fn example_strings() {
    //  Creating Strings:
    //  '.to_string()' converts a &str to a String (this copies the string)
    let msg = "too many pets".to_string();
    //  '!format()' works like 'println!()', except it returns the string that would have been written (sans newline)
    assert_eq!(format!("{}°{:02}′{:02}′′N", 24, 5, 23), "24°05′23′′N".to_string());
    //  '.concat()' and 'join(sep)' form a String from many strings
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    //  String literals are enclosed in quotes. Backslash is used as an escape (just as with char literals)
    let speech = "\"Ouch!\" said the well.\n";
    //  A string may span multiple lines. The newline, and any leading spaces are included.
    println!("In the room the women come and go,
        Singing of Mount Abora");
    //  If the line ends in a backslash, the newline and any leading spaces are dropped
    println!("It was a bright, cold day in April, and \
        there were four of us -- \
        more or less.");
    //  Raw strings eliminate the need for escape sequences
    //  Any number of '#' can be used (including zero), so long as they are balanced
    let default_path = r"C:\Program Files\Gorillas";
    let pattern = r#"\d+(\.\d+)*"#;

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    //  A strings length is measured in bytes, not characters
    assert_eq!("ಠ_ಠ".len(), 7);
    //  To get the number of characters
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    //  Two strings are equal if they contain the same characters in the same order
    //  Strings also support the comparison operators <, <=, >, and >=
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■"); 
    assert_eq!(" clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") { 
        assert!(word.starts_with("v"));
    }
}

fn example_byte_strings() {
    //  A string literal with the 'b' prefix is a slice of u8 values
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    //  Can only use ASCII and \xHH sequences
    //  A raw byte string begins with 'br'
}

//  Avoid Strings that aren't valid unicode:
//  Use std::path::PathBuf / &Path for filenames
//  Use Vec<u8> / &[u8] for binary data
//  Use OsString / &OsStr for native forms of strings from the OS
//  For interoperability with null-terminated C-strings, use std::ffi::CString / &CStr

fn main() {
    example_casts();
    example_integer_methods();
    example_floating_point();
    example_characters();
    example_tuples();
    example_boxes();
    example_arrays();
    example_vectors();
    example_slices();
    example_strings();
    example_byte_strings();
}

