//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  }}}

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
    //  &str implements 'Default'

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

    //  <>

    println!("example_Cow_and_String, DONE");
}


fn example_Formatting_Values()
{
    println!("example_Formatting_Values, DONE");
}


fn example_Regular_Expressions()
{
    println!("example_Regular_Expressions, DONE");
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
    example_Normalization();
}

