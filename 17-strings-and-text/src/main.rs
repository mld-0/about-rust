//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-11-10T22:12:08AEDT String formatting example, printing 'insn' / 'lsb' / 'msb' produces YCM error message about (adding a feature to use) capturing (but it works as expected, without error) ... (how to check YCM is current / rebuild it periodically if it is not?)
//  Ongoing: 2022-11-10T23:07:34AEDT 'write!()' macro returns 'std::fmt::Result' (return value for custom 'Display' implementation)
//  Ongoing: 2022-11-11T00:10:43AEDT YouCompleteMe, how to report version of <Rust/other> language being used
//  }}}

//  Continue: 2022-11-10T21:57:29AEDT using Cow with str/String
//  Continue: 2022-11-10T23:15:04AEDT (expand example) Using formatting code in custom code example 'write_log_entry()' / 'Eg_log!()'
//  Continue: 2022-11-11T23:03:56AEDT conversion between str / String

use std::str::FromStr;
use std::borrow::Cow;

//  Unicode text:                           String / &str
//  Filenames                               std::path::PathBuf / &Path
//  Binary data                             Vec<u8> / &[u8]
//  Env variables / cli arguments           OsString / &OsStr
//  C-strings                               std::ffi::CString / &CStr

fn example_aboutUnicode()
{
    //  Unicode and ASCII match for 0 to 0x7f
    //  Latin-1: a subset of unicode, extends ASCII to 0xff

    //  UTF-8 is used for String / str
    //  It uses 1-4 bytes to encode a character (no more than is neccessary)
    //  Not all byte sequences are valid UTF-8: [0xd800,0xdfff] and >0x10ffff are invalid

    assert_eq!("うどん: udon".as_bytes(), 
            &[0xe3, 0x81, 0x86,                     // う 
            0xe3, 0x81, 0xa9,                       // ど
            0xe3, 0x82, 0x93,                       // ん
            0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e      // : udon 
        ]);


    //  Text directionality:
    //  Some methods use 'left' / 'right' to mean 'start' / 'end' of text.
    //  <(Book claim - for Hebrew text, rightmost character is returned by 'next()')>
    assert_eq!("ערב טוב".chars().next(), Some('ע'));
    assert_eq!("abc def".chars().next(), Some('a'));

    println!("example_aboutUnicode, DONE");
}


fn example_Char()
{
    //  32 bit value holding a Unicode code point
    //  Guaranteed to be [0x0, 0xd7ff] or [0xe000, 0x10ffff]

    //  Implements Copy/Clone, as well as comparison/hashing/formatting

    //  Methods:
    //      is_numeric()
    //      is_alphabetic()
    //      is_alphanumeric()
    //      is_whitespace()
    //      is_control()

    //  <(Converting to/from digits:)>
    //      ch.to_digit(radix)
    //      std::char::from_digit(num, radix)
    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15, 16), Some('f'));

    //  Case check
    //      ch.is_lowercase() / ch.is_uppercase()

    //  Case conversion
    //      ch.to_lowercase() / ch.to_uppercase()
    //  Returns iterators that produce case-converted characters
    let mut upper = 's'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    //  Unicode case-conversion is not always 1:1
    let mut upper = 'ß'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    //  Use 'as' to cast chars to integers
    assert_eq!('B' as u32, 66);
    assert_eq!('饂' as u8, 66);             //  upper bits truncated 
    assert_eq!('二' as i8, -116);           //  upper bits truncated

    //  u8 can be cast to char, but wider integers cannot
    assert_eq!(66u8 as char, 'B');
    assert_eq!(char::from(66), 'B');
    assert_eq!(std::char::from_u32(0x9942), Some('饂'));
    assert_eq!(std::char::from_u32(0xd800), None);

    println!("example_Char, DONE");
}


fn example_String()
{
    //  String and str are guaranteed to only ever hold well-formatted UTF-8
    //  <(String is a resizeable buffer containing str)>
    //  <(String is implemented using Vec<u8>)>

    //  String methods make recieve either 'String' or 'str', depending on whether they need a resizeable buffer
    //  String dereferences to &str, so any method defined on 'str' is available to 'String' as well

    //  String methods <generally> index text by byte instead of by character


    //  Creating Strings:
    //      String::new()
    //      String::with_capacity()
    //      slice.to_string()
    //      iter.collect()

    //  Simple inspection:
    //      slice.len()
    //      slice.is_empty()
    //      slice[range]
    //      slice.split_at(i)
    //      slice.is_char_boundry(i)

    //  Appending an inserting Text:
    //      string.push(ch)
    //      string.push_str(slice)
    //      string.extend(iter)
    //      string.insert(i, ch)
    //      string.insert_str(i, slice)

    //  'write!()' and 'writeln!()' macros can append to Strings (import 'std::fmt::Write')

    //  String implements Add/AddAssign
    //  (the left operand is taken by-value (and must be a 'String' not 'str'))
    let l = "partners".to_string();
    let r = "crime".to_string();
    assert_eq!(l + " in " + &r, "partners in crime");
    let mut r = "crime".to_string();
    r += " doesn't pay";
    assert_eq!(r, "crime doesn't pay");


    //  Removing text:
    //      string.clear()
    //      string.truncate(n)
    //      string.pop()
    //      string.remove(i)
    //      string.drain(range)

    let mut c = "chocolate".to_string();
    assert_eq!(c.drain(3..=5).collect::<String>(), "col");
    assert_eq!(c, "choate");


    //  A 'Pattern' is a type for searching text ('std::str::Pattern')
    let haystack = "One fine day, in the middle of the night";
    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));

    //  There are 4 main kinds of pattern
    //      char                    matches that character
    //      String/&str             matches that substring
    //      FnMut(char) -> bool     closure matching a single character
    //      &[char]                 matches any character in list

    //  <(Pattern is not stable?)>


    //  Search and replace
    //      slice.contains(pattern)
    //      slice.starts_with(pattern)
    //      slice.ends_with(pattern)
    //      slice.find(pattern)
    //      slice.rfind(pattern)
    //      slice.replace(pattern, replacement)
    //      slice.replacen(pattern, replacement, n)
    assert!("2017".starts_with(char::is_numeric));
    let quip = "We also know there are known unknowns";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));
    assert_eq!("The only thing we have to fear is fear itself"
               .replace("fear", "spin"),
               "The only thing we have to spin is spin itself");
    assert_eq!("`Borrow` and `BorrowMut`"
               .replace(|ch: char| !ch.is_alphanumeric(), ""),
               "BorrowandBorrowMut");


    //  Iterating over text:
    //  (splits are the ranges between matches)
    //      slice.chars()
    //      slice.char_indices()
    //      slice.bytes()
    //      slice.lines()
    //      slice.split(pattern)
    //      slice.rsplit(pattern)
    //      slice.split_terminator(pattern)
    //      slice.rsplit_terminator(pattern)
    //      slice.splitn(n, pattern)
    //      slice.rsplitn(n, pattern)
    //      slice.split_whitespace()
    //      slice.matches(pattern)
    //      slice.rmatches(pattern)
    //      slice.matches_indices(pattern)
    //      slice.rmatch_indices(pattern)

    
    //  Trimming:
    //      slice.trim()
    //      slice.trim_left()
    //      slice.trim_right()
    //      slice.trim_matches(pattern)
    //      slice.trim_start_matches(pattern)
    //      slice.trim_right_matches(pattern)
    assert_eq!("\t*.rs ".trim(), "*.rs");
    assert_eq!("001990".trim_start_matches('0'), "1990");
    assert_eq!("001990".trim_end_matches('0'), "00199");


    //  Case conversion:
    //      slice.to_uppercase()
    //      slice.to_lowercase()


    //  Parsing other types from Strings:
    //  'std::str::FromStr' is implemented by other types that can be parsed from Strings
    //  <(Definition:)>
    pub trait Eg_FromStr: Sized {
        type Err;
        fn from_str(s: &str) -> Result<Self, Self::Err>;
    }
    assert_eq!(usize::from_str("3628800"), Ok(3628800));
    assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(f64::from_str("abc").is_err());

    //  <(types that implement 'FromStr' are also implemented for 'String::parse')>
    use std::net::IpAddr;
    assert_eq!("fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap(), 
               IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));


    //  Converting Strings to other types:
    //  'std::fmt::Display' is implemented by types that can be printed as '{}'
    assert_eq!(format!("{}, wow", "doge"), "doge, wow");
    assert_eq!(format!("{}", true), "true");
    //  Smart pointer types (eg: 'Box<T>') implement Display if T does
    //  (Standard containers do not (use 'std::fmt::Debug' instead))

    //  types that implement 'Display' are also implemented for 'std::str::ToString'
    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    assert_eq!(address.to_string(), "fe80::3ea9:f4ff:fe34:7a50");

    //  'std::fmt::Debug' is implemented by types that can be printed as '{:?}'
    //  (implemented by every public type in the standard library)
    assert_eq!(format!("{:?}", vec![0,1,2,3]), "[0, 1, 2, 3]");

    //  Automatically derive 'Debug' for a custom type
    #[derive(Debug)]
    struct Eg_Complex { r: f64, i: f64, };
    assert_eq!(format!("{:?}", Eg_Complex { r: 1.0, i: 1.0 }), "Eg_Complex { r: 1.0, i: 1.0 }");


    //  Borrowing as other text-like types:
    //  Slices/Strings implement: AsRef<str>, AsRef<[u8]>, AsRef<Path>, AsRef<OsStr>, Borrow<str>


    //  Accessing Text as UTF-8:
    //      slices.as_bytes()           &[u8]
    //      string.into_bytes()         Vec<u8>


    //  Producing Text from UTF-8 data:
    //      str::from_utf8(byte_slice)
    //      String::from_utf8(vec)
    //      String::from_utf8_lossy(byte_slice)
    //      String::from_utf8_unchecked(vec)
    //      str::from_utf8_unchecked(byte_slice)
    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86]; 
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));


    //  Strings as Generic Collections: 
    //  String implements both 'std::default::Default' / 'std::iter::Extend'
    //  &str implements 'std::default::Default'

    println!("example_String");
}


fn example_Cow_and_String()
{
    //  Use 'std::borrow::Cow' to avoid having to allocate for a String unless necessary

    //  <(Return static string on Windows / <String/str> on Unix)>
    fn get_name() -> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| Cow::Owned(v))
            .unwrap_or(Cow::Borrowed("Person Experiencing Windows"))
    }
    println!("user=({})", get_name());

    //  Using Cow with <String/str>
    //  <>

    println!("example_Cow_and_String, DONE");
}


fn example_Formatting_Values()
{
    //  Rust uses formatting macros for outputting text
    //      format!()
    //      println!() / print!()
    //      writeln!() / write!()
    //      panic!()
    //  These always borrow a reference to their arguments (never mutating/taking-ownership of them)

    println!("{:.3}μs: relocated {} at {:#x} to {:#x}, {} bytes",
             0.84391, "object", 140737488346304_usize, 6299664_usize, 64);

    //  Implement 'std::fmt::Display' to use these with custom types

    //  <(Use 'format_args!' and 'std::fmt::Arguments' to create custom functions/macros supporting formatting)>

    //  Format parameters: {which:how}
    //  <(where no 'which' is given, arguments are used L->R)>

    assert_eq!(format!("from the {1} to the {0}", "grave", "cradle"), 
               "from the cradle to the grave");

    assert_eq!(format!("v=({:?})", vec![0,1,2,5,12,29]),
                "v=([0, 1, 2, 5, 12, 29])");

    assert_eq!(format!("name=({:?})", "Nemo"),
                "name=(\"Nemo\")");

    assert_eq!(format!("{:08.2} km/s", 11.186),
                "00011.19 km/s");

    assert_eq!(format!("{:02x} {:02x}", 105, 42),
                "69 2a");

    //let insn = "adc #42"; let lsb = 105; let msb = 42;
    //assert_eq!(format!("{lsb:02x}, {msb:02x}, {insn}"), "69, 2a, adc #42");

    //  Use double '{{' to print a literal '{'
    assert_eq!(format!("{{}}"), "{}");

    //  'std::path::Path' supports Display

    //  Formatting text values:
    //          {:4}            minimum field width
    //          {:.12}          text length limit
    //          {:4.12}         minimum field width and text length limit
    //          {:<12}          width, aligned left
    //          {:^12}          width, aligned center
    //          {:>12}          width, aligned right
    //          {:=^12}         width, aligned center, pad with '='
    //          {:*>12.4}       width, limit, aligned right, pad with '*'
    //  <(Width of special characters)>
    //  (is best handled by UI toolkit / HTML/CSS/Browser)

    //  Formatting integers:
    //          {:+}            forced sign
    //          {:12}           minimum field width
    //          {:012}          width, leading zeros
    //          {:+012}         width, leading zeros, forced sign
    //          {:<12}          width, align left
    //          {:^12}          width, align center
    //          {:>12}          width, align right
    //          {:<+12}         width, align left, forced sign
    //          {:=^12}         width, align center, pad with '='
    //          {:b}            binary notation
    //          {:o}            octal notation
    //          {:x}            hex notation
    //          {:+#12x}        width, hex notation, forced sign, pad with '#'
    //  <(When we request leading zeros, alignment/padding are ignored)>

    //  Formatting floats:
    //          {:.2}           precision
    //          {:12}           minimum field width
    //          {:12.2}         width, precision
    //          {:012.6}        width, precision, leading zeros
    //          {:e}            scientific
    //          {:E}            scientific
    //          {:12.3e}        width, precision, scientific

    //  Custom types that need to be displayed should implement 'std::fmt::Display'
    //  Error types should implement 'std::error::Error' (which extends 'Display')


    //  '{:?}' is used for debugging output
    //  Implement 'std::fmt::Debug' or use '#[derive(Debug)]' for custom types
    //  Supported by standard containers, although exact format is not guaranteed not to change

    #[derive(Copy, Clone, Debug)]
    struct Eg_Complex { re: f64, im: f64, };
    let x = Eg_Complex { re: -0.5, im: 1.75, };
    println!("x=({:?})", x);


    //  Printing pointers: {:p}
    //  (For any kind of pointer: Box / Rc / <ect>)
    //  <(The value printed is that of the pointer to the underlying resource)>
    use std::rc::Rc;
    let a = Rc::new("mazurka".to_string());
    let b = a.clone();
    let c = Rc::new("mazurka".to_string());
    println!("a=({:p}), b=({:p}), c=({:p})", a, b, c);


    //  Specifying order of parameters:
    assert_eq!(format!("{1}, {0}, {2}", "zeroth", "first", "second"),
                "first, zeroth, second");

    //  Specifying parameters by name:
    assert_eq!(format!("{description:.<25}{quantity:2} @ {price:5.2}",
                       price=3.25, quantity=3, description="Maple Tumeric Latte"),
                "Maple Tumeric Latte...... 3 @  3.25");

    //  Ordered/named/positional arguments to print/format macros can be mixed 
    //  (named arguments appear at the end of the list)
    assert_eq!(format!("{mode} {2} {} {}",
                       "people", "eater", "purple", mode="flying"),
                "flying purple people eater");


    //  Dynamic widths/precisions
    let content = "There can be no other end";
    assert_eq!(format!("{:>.9}", content), "There can");
    assert_eq!(format!("{:>.*}", 9, content), "There can");
    assert_eq!(format!("{:>.1$}", content, 9), "There can");
    assert_eq!(format!("{:>.width$}", content, width=9), "There can");
    //  (field width / precision values must be usize)


    //  Formatting custom types:
    //      {}                  std::fmt::Display
    //      {bits:#b}           std::fmt::Binary
    //      {:#5o}              std::fmt::Octal
    //      {:4x}               std::fmt::LowerHex
    //      {:016X}             std::fmt::UpperHex
    //      {:.3e}              std::fmt::LowerExp
    //      {:.3E}              std::fmt::UpperExp
    //      {:#?}               std::fmt::Debug
    //      {:p}                std::fmt::Pointer

    //  <(All these traits have the same form)>
    trait Eg_Display {
        fn fmt(&self, dest: &mut std::fmt::Formatter) 
            -> std::fmt::Result;
    }

    //  Implement Display for Eg_Complex, with optional polar format (specified by '#')
    impl std::fmt::Display for Eg_Complex {
        fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
            if dest.alternate() {
                let abs = f64::sqrt( self.re * self.re + self.im * self.im );
                let theta = f64::atan2(self.im, self.re) / std::f64::consts::PI * 180.0;
                write!(dest,  "{} ∠ {}°", abs, theta)
            } else {
                let i_sign = if self.im < 0.0 { '-' } else { '+' };
                write!(dest, "{} {} {}i", self.re, i_sign, f64::abs(self.im))
            }
        }
    }

    let x = Eg_Complex { re: 0.0, im: 1.0, };
    assert_eq!(format!("{}", x), "0 + 1i");
    assert_eq!(format!("{:#}", x), "1 ∠ 90°");

    //  <(custom 'Display' implementations should never originate errors themselves(?))>


    //  Using custom formatting in code:
    //  <>
    fn write_log_entry(entry: std::fmt::Arguments) {
        //  ...
    }
    write_log_entry(format_args!("abc, i=({})", 53));
    macro_rules! Eg_log {
        ($format:tt, $($arg:expr),*) => 
            ( write_log_entry(format_args!($format, $($arg), *)) )
    }
    Eg_log!("abc, i=({})", 53);
    //  (Creating an 'std::fmt::Arguments' object is cheap - formatting is not performed unless it is required)


    println!("example_Formatting_Values, DONE");
}


fn example_Regular_Expressions()
{
    //  Rust's official regex library is an external crate
    //  (add 'regex = "0.2.2"' to Cargo.toml)
    extern crate regex;

    //  'regex' uses Re format <?>

    //  Basic use:
    use regex::Regex;

    let re = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
    let haystack = r#"regex = '0.2.5'"#;
    assert!(re.is_match(haystack));

    //  'Regex::captures()' returns a 'regex::Captures' value
    let captures = re.captures(haystack).unwrap();
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    //  Accessing results by index panics if requested group didn't match
    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);

	let haystack = "In the beginning, there was 1.0.0. For a while, we used 1.0.1-beta, but in the end, we settled on 1.2.4.";

    //  Iterate over matches
	let matches: Vec<&str> = re.find_iter(haystack).map(|x_| x_.as_str()).collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    for x in re.find_iter(haystack) {
        print!("x=({}), ", x.as_str());
    }
    println!();

    //  <('find_iter()' produces a iterator returning a Match value for each match)>
    //  <('captures_iter()' (slower) produces all capture groups)>

    //  'Regex::new()' is slow

    println!("example_Regular_Expressions, DONE");
}


fn example_Lazily_Built_Regex()
{
    println!("example_Lazily_Built_Regex, DONE");
}


fn example_Normalization()
{
    println!("example_Normalization, DONE");
}


fn main() 
{
    example_aboutUnicode();
    example_Char();
    example_String();
    example_Cow_and_String();
    example_Formatting_Values();
    example_Regular_Expressions();
    example_Lazily_Built_Regex();
    example_Normalization();
}

