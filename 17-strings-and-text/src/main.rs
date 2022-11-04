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


    //  Patterns for searching text:
    //  <>

    println!("example_String");
}


fn main() 
{
    example_aboutUnicode();
    example_Char();
    example_String();
}

