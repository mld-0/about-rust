//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-11-14T22:52:25AEDT 'Read' method return types -> book uses u64, rust-docs uses usize
//  Ongoing: 2022-11-14T23:27:24AEDT io::BufRead vs io::BufReader
//  Ongoing: 2022-11-14T23:41:06AEDT what is going on with (paths that return) errors in 'use_grep()'?
//  }}}
use std::io;

//  <(To import all/common io classes)>
use std::io::prelude::*;

//  Rust provides three general traits for IO:
//          Read            byte-oriented input
//          BufRead         text-or-byte-oriented input 
//          Write           text-or-byte oriented output

fn example_IO_trait_definitions()
{
    //  A Reader is closed when it is dropped
    trait Eg_Read {
        //  Read input bytes and store then in 'buf' (up to 'buf.size()' bytes)
        //  Return number of bytes read as Result
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;

        //  Read all remaining input into 'byte_vec'
        //  (Do not use on untrusted input)
        fn read_to_end(&mut self, byte_vec: &mut Vec<u8>) -> io::Result<usize>;

        //  Read all remaining input into 'buf'
        //  (Input must be valid UTF8, do not use on untrusted input)
        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>;

        //  Read enough bytes to fill 'buf'
        //  (Input must be long enough to fill 'buf')
        fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;

        //  Additional input methods
        //  <>

        //  Return an iterator over bytes of input
        //  (Highly inefficent for unbuffered input)
        fn bytes(self) -> io::Bytes<Self> 
            where Self: Sized;

        //  Returns new reader, combining self/next Readers
        fn chain<R: io::Read>(self, next: R) -> io::Chain<Self, R>
            where Self: Sized;

        //  Returns new reader, which reads at most 'n' bytes
        fn take(self, n: u64) -> io::Take<Self>
            where Self: Sized;

        //  <()>
        //fn chars(self) -> 
    }

    //  Buffered readers/writers use additional memory to reduce the number of system calls that must be made (allowing text to be read line-by-line efficently)
    trait Eg_Buffered: io::Read {

        //  Read next line of text, appending it to 'line' (newlines are not stripped)
        //  Return number of bytes read
        fn read_line(&mut self, line: &mut String) -> io::Result<usize>;

        //  Returns iterator over lines of input (newlines are stripped)
        fn lines(self) -> io::Lines<Self>
            where Self: Sized;

        //  Read bytes into 'buf' until delimiter 'byte' or EOF is reached
        //  Return number of bytes read
        fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize>;

        //  Return an iterator over contents of reader, split on 'byte'
        fn split(self, byte: u8) -> io::Split<Self>
            where Self: Sized;

        //  Additional methods:
        //  <>

        //  Low level methods:
        //  Return contents of internal buffer (refilling it with new data)
        fn fill_buf(&mut self) -> io::Result<&[u8]>;
        //  Tell reader that 'amt' bytes have been consumed
        fn consume(&mut self, amt: usize);
    }

    println!("example_IO_trait_definitions, DONE");
}

fn example_Reading_Lines()
{
    fn grep_stdin(target: &str) -> io::Result<()> {
        let stdin = io::stdin();
        //  stdin is protected by mutex, we must lock it before we can read from it
        for line_result in stdin.lock().lines() {
            let line = line_result?;
            if line.contains(target) {
                println!("{}", line);
            }
        }
        Ok(())
    }

    //  'grep_generic' supports either 'stdin.lock()' or a buffered File 'BufReader::new(f)'
    fn grep_generic<R>(target: &str, reader: R) -> io::Result<()> 
        where R: io::BufRead
    {
        for line_result in reader.lines() {
            let line = line_result?;
            if line.contains(target) {
                println!("{}", line);
            }
        }
        Ok(())
    }

    //  Using our grep function - with search string and file paths taken from cli arguments
    use std::error::Error;
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::PathBuf;
    fn use_grep() -> Result<(), Box<dyn Error>> {
        let mut args = std::env::args().skip(1);
        let target = match args.next() {
            Some(s) => s,
            None => Err("usage: grep PATTERN FILE...")?
        };
        let files: Vec<PathBuf> = args.map(PathBuf::from).collect();
        if files.is_empty() {
            let stdin = io::stdin();
            grep_generic(&target, stdin.lock())?;
        } else {
            for file in files {
                let f = File::open(file)?;
                grep_generic(&target, BufReader::new(f))?;
            }
        }
        Ok(())
    }

    println!("example_Reading_Lines, DONE");
}


fn example_Collecting_Lines()
{
    println!("example_Collecting_Lines, DONE");
}


fn main() 
{
    example_IO_trait_definitions();
    example_Reading_Lines();
    example_Collecting_Lines();
}

