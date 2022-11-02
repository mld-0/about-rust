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
    //  32 bit value holding a Unicode value

    //  Methods:
    //      is_numeric()
    //      is_alphabetic()
    //      is_alphanumeric()
    //      is_whitespace()
    //      is_control()

    //  <(Converting to/from digits:)>

    println!("example_Char, DONE");
}


fn main() 
{
    example_aboutUnicode();
    example_Char();
}

