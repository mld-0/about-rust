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
//  Ongoing: 2022-11-14T23:41:06AEDT what is going on with (paths that return) errors in 'use_grep()'? [...] (recall that '?' either evaluates to the value, or returns the error) ... (return-ing branches of a match statement *don't* have to be the same type as the rest of the branches?)
//  Ongoing: 2022-11-15T22:56:01AEDT why is it 'mut buffer: io::BufReader<R>' and not 'buffer: mut io::BufReader<R>'? [...] (and why is it necessary to specify 'BufReader' is Readable?)
//  Ongoing: 2022-11-15T23:16:43AEDT writing to the middle of a Cursor<Vec<u8>> (can it be done / is it efficent)
//  Ongoing: 2022-11-15T23:28:08AEDT Reader/Writer on in-memory Vec<u8>: seeking? (what about writing to / reading from same buffer?)
//  Ongoing: 2022-11-15T23:40:41AEDT debug-printing Cursor doesn't show contents of underlying buffer?
//  Ongoing: 2022-11-20T22:56:35AEDT reader_collect_lines_ii, (result that succedes for some lines fails for others?) ... using '?' -> (we want to return 'Vec<String>', but this introduces a Result) ... (this '?' or '.unwrap()' is unwrapping a single value (either an error, or Vec<String>)?)
//  Ongoing: 2022-11-20T23:04:19AEDT BufReader, Cursor, (various other types) play nicely here recieving their ctor argument either by-ref/by-val
//  Ongoing: 2022-11-20T23:09:03AEDT (are these the best/correct way to pass BufReader/BufWriter): 'fn read_Reader<R: Read>(mut buffer: BufReader<R>)' / 'fn write_Writer<W: Write>(buffer: &mut BufWriter<W>)'
//  Ongoing: 2022-11-20T23:11:55AEDT creating BufReader from Cursor/bytes -> example where one is seekable / one is not (passing BufReader/BufWriter (see above) does not permit seeking?)
//  Ongoing: 2022-11-20T23:19:02AEDT is 'Eg_Buffered' correct (should it be (something like) 'Eg_BufRead'?)
//  Ongoing: 2022-11-20T23:30:58AEDT (this chapter is another example of a subject presented suboptimially - a better summary is called for?)
//  Ongoing: 2022-11-21T00:57:28AEDT stdin/stdout/stderr usage examples, (imitating/replacing stdin?)
//  Ongoing: 2022-11-25T18:26:38AEDT difference between 'readlink' / 'realpath' (corresponding to) canonicalize / read_link 
//  Ongoing: 2022-11-25T18:29:11AEDT 'remove_file' (and other deletion (and creation) functions) return nothing to indicate success/failure (do they panic on failure?)
//  Ongoing: 2022-11-25T18:41:19AEDT (case study) object 'MetaData' - return type of a function that <differs/represents> by OS(?)
//  Ongoing: 2022-11-25T19:40:33AEDT recusive 'copy_dir_to()' function untested [...] (also I don't like '&dest.join(entry.file_name())')
//  }}}

//  Continue: 2022-11-25T20:08:39AEDT networking
//  Continue: 2022-11-25T20:08:57AEDT other-crates
//  Continue: 2022-11-25T20:09:02AEDT cleanup

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

use std::io;

use std::io::prelude::*;
//  <(Provides the 4 main IO Traits)>
//          Read            byte-oriented input
//          BufRead         text-or-byte-oriented input 
//          Write           text-or-byte oriented output
//          Seek            a cursor which can be moved within a stream of bytes

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
            None => Err("usage: grep PATTERN FILE...")?,
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


fn example_Emulating_BufReaderBufWriter()
{
    //  LINK: https://stackoverflow.com/questions/41069865/how-to-create-an-in-memory-object-that-can-be-used-as-a-reader-writer-or-seek

    //  Creating a BufReader from bytes / str using Cursor
    use std::io::{Read,BufReader,BufWriter};
    fn read_Reader<R: Read>(mut buffer: BufReader<R>) {
        let mut s = String::new();
        let _ = buffer.read_to_string(&mut s);
        println!("got s=({})", s);
    }

    //  Creatign a BufReader from bytes using Cursor
    read_Reader(BufReader::new(Cursor::new("abc".as_bytes())));

    //  Creating a BufReader from bytes
    read_Reader(BufReader::new("abc".as_bytes()));

    fn write_Writer<W: Write>(buffer: &mut BufWriter<W>) {
        buffer.write("Hello World".as_bytes());
    }

    let mut b = BufWriter::new(Cursor::new(Vec::<u8>::new()));
    write_Writer(&mut b);
    println!("b=({:?})", b);

    let mut b = BufWriter::new(Vec::<u8>::new());
    write_Writer(&mut b);
    println!("b=({:?})", b);

    let mut v = Vec::<u8>::new();
    let mut b = BufWriter::new(&mut v);
    write_Writer(&mut b);
    println!("b=({:?})", b);
    drop(b);
    println!("v=({:?})", &v);


    //  <(Best way?)>
    //  'Cursor<Vec<u8>>' can be used like a file 
    //  (it implements Read/Write/Seek)
    use std::io::{Cursor,SeekFrom};
    let mut c = Cursor::new(Vec::<u8>::new());
    c.write_all(&[1,2,3,4,5]).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::<u8>::new();
    c.read_to_end(&mut out).unwrap();
    let z = c.into_inner();     //  access underlying Vec<u8> from Cursor<Vec<u8>>

    //  Create BufReader from Cursor<Vec<u8>>
    //  <>

    //  Alternatively, Create BufReader from Vec<u8> <(cannot use seek)>
    //  <>

    //  Creating a BufWriter from String
    //  (can be done, requires unsafe function)
    //  <(or, instead, use Vec<u8> and convert that to String later?)>
    let mut s = String::new();
    let mut b = BufWriter::new(unsafe { s.as_mut_vec() } );
    write_Writer(&mut b);
    drop(b);
    println!("s=({:?})", s);

    //  Vec<u8> can be used as a file
    //  (It implements Write, and &[u8] implements Read, but cannot Seek)
    let mut f = Vec::<u8>::new();
    f.write_all(&[1,2,3,4,5]).unwrap();
    let mut out = Vec::<u8>::new();
    let mut slice = f.as_slice();
    slice.read_to_end(&mut out).unwrap();

    println!("example_Emulating_BufReaderBufWriter, DONE");
}


fn example_Collecting_Lines()
{
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Read;
    use std::io::Cursor;

    //  If we collect lines from 'lines()', which produces Result values, we are left with a Vec of Results
    //let r1: Vec<io::Result<String>> = reader.lines().collect();

    //  However we can use:
    //let r2 = reader.lines().collect::<io::Result<Vec<String>>>()?;

    fn reader_collect_lines_i<R: Read>(mut buffer: BufReader<R>) -> Vec<String> 
    {
        let mut result = Vec::<String>::new();
        for line_result in buffer.lines() {
            let line = line_result.unwrap();
            result.push(line);
        }
        result
    }
    //  or
    fn reader_collect_lines_ii<R: Read>(mut buffer: BufReader<R>) -> Vec<String>
    {
        //let result = buffer.lines().collect::<io::Result<Vec<String>>>()?;
        let result = buffer.lines().collect::<io::Result<Vec<String>>>().unwrap();
        result
    }

    //  The standard library implements 'FromIterator' for 'Result',
    //  So if T can be collected into C, then Result<T,E> can be collected into Result<C,E>
    //  <(making 'io::Result<Vec<String>' a collection type)>

    //  Passing Vec<u8> as BufReader (using Cursor)
    let mut v = Vec::<u8>::from( "hello\nworld\n".as_bytes() );
    let mut c = Cursor::new(&mut v);
    let mut b = BufReader::new(&mut c);
    let mut r = reader_collect_lines_ii(b);
    println!("r=({:?})", r);

    println!("example_Collecting_Lines, DONE");
}


fn example_Writers()
{
    //  'write!()' and 'writeln!()' are comperable to 'print!()' and 'println!()'
    //  They take a Writer as first argument, and return a Result
    //  (write/writeln panic if the write fails)
    write!(io::stdout(), "writeln! example 1\n").unwrap();
    writeln!(io::stdout(), "writeln! example 2").unwrap();

    //  <(when is dealing with result (using unwrap/?) required?)>
    write!(io::stdout(), "writeln! example 3\n");
    writeln!(io::stdout(), "writeln! example 4");

    trait Eg_Write {
        //  Low level method, avoid using directly
        //  Returns Result containing number of bytes written
        fn write(&mut self, buf: &[u8]) -> io::Result<usize>;

        //  Flush any buffered data
        fn flush(&mut self) -> io::Result<()>;

        //  Attempt to write all bytes 'buf'
        fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;

        //  For use with 'format_args!()'
        fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>;

        //  Return a by-ref adapter for this instance of Write
        fn by_ref(&mut self) -> &mut Self
            where Self: Sized;
    }

    //  All remaining data is flushed when the Writer is dropped
    //  (However, any error during this final write is ignored - for this reason, manually '.flush()' buffered writers before dropping them to ensure errors are caught)

    println!("example_Writers, DONE");
}


fn example_TempDir()
{
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};

    //  Get env $TMPDIR
    let tmp = std::env::temp_dir();
    println!("tmp=({:?})", tmp);

    //  requires 'tempfile = "3"'
    //      tempfile()      temporary file (cleaned up by OS)
    //      tempdir()       temporary directory (cleaned up by dtor)

    //  Create a temporary file
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    println!("tmpfile=({:?})", tmpfile);
    write!(tmpfile, "Hello World!").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);

    //  Create a temporary directory
    let dir = tempfile::tempdir().unwrap();
    println!("dir=({:?})", dir.path());
    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Brian was here. Briefly.").unwrap();
    file.flush();
    drop(file);
    dir.close().unwrap();

    println!("example_TempDir, DONE");
}


fn example_Files()
{
    use std::fs::File;
    use std::fs::OpenOptions;

    //  Basic opening of files is handled with:
    //      fs::File::open(filename)
    //      fs::File::create(filename)
    
    let dir = tempfile::tempdir().unwrap();
    let path_file = dir.path().join("my-stuff-file.txt");

    //  More advanced behaviour can be specified with 'fs::OpenOptions'
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)                   //  fail if file exists
        .open(path_file).unwrap();
    println!("file=({:?})", file);

    println!("example_Files, DONE");
}


fn example_Seeking()
{
    use std::io::{Write, Read, Seek, SeekFrom};

    //  'File' implements Seek
    //  <(Definition:)>
    pub trait Eg_Seek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
    }
    pub enum Eg_SeekFrom {
        Start(u64), 
        End(i64),
        Current(i64),
    }

    //  Seek allows us to hop around in a file
    //  It is a slow operation (even with an SSD)

    //  Rewind to beginning:
    //      file.seek(SeekFrom::Start(0))

    //  Go back 8 bytes:
    //      file.seek(SeekFrom::Current(-8))

    println!("example_Seeking, DONE");
}


fn example_Other_ReaderWriterTypes()
{
    //  io::stdin()
    //  Reader for standard input (type 'io::Stdin')
    //  Has a mutex that must be unlocked before use
    //  <('io::stdin().lock()' doesn't work, assign it to a variable first:)>
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    //  error:
    //let lines = io::stdin().lock().lines();

    //  io::stdout()
    //  io::stderr()
    //  Writers for standard output/error
    //  Also has a mutex that must be unlocked before use 

    //  Vec<u8> implements Write

    //  Cursor::new(buf)
    //  Supports any type which implements AsRef<[u8]>
    //  A cursor is a buffered reader that reads from 'buf'.
    //  Implements Read/BufRead/Seek. Also implements Write for '&mut [u8]' / 'Vec<u8>'.

    //  std::new::TcpStream
    //  Represents a TCP network connection, supports Read / Write

    //  std::process:Command
    //  Supports spawning a child process and piping data to its stdin
    use std::process::{Command,Stdio};
    let my_words = vec_of_strings!["asdf", "zxcv", "qwer"];
    let mut child = Command::new("grep")
        .arg("-e")
        .arg("[aeiou]")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();
    let mut to_child = child.stdin.take().unwrap();
    for w in my_words {
        writeln!(to_child, "{}", w);
    }
    drop(to_child);
    let mut from_child = child.stdout.take().unwrap();
    let mut s = String::new();
    from_child.read_to_string(&mut s);
    println!("s=({})", s);

    //  io::sink()
    //  Writer which discards any data recieved

    //  io::empty()
    //  Reader which immediately returns EOF

    //  io::repeat(byte)
    //  Reader which repeats 'byte' endlessly

    println!("example_Other_ReaderWriterTypes, DONE");
}


fn example_Other_IO_Crates()
{
    //  byteorder:
    //  <>

    //  flate2: (gzip io)
    //  <>

    //  serde: (serialization)
    //  <>

    println!("example_Other_IO_Crates, DONE");
}


fn example_files_and_directories()
{
    //  std::ffi::OsStr is a String type that is a superset of UTF-8
    //  It represents filenames / cli-arguments / env-variables, whether they are valid UTF-8 or not.
    //  <(On Unix, this is any sequence of bytes. On Windows, this is UTF-8 extended to 16-bits)>

    //  std::ffi::OsString
    //  A heap-allocated OsStr

    //  std::path::Path
    //  An extension of OsStr, adding filename related methods

    //  Use Path for absolute/relative paths
    //  Use OsStr for indervidual components of a path

    //  Passing a Path:
    //  str / OsStr / Path all implement 'AsRef<Path>', allowing us to declare a function that accepts any of them as a path
    fn swizzle_file<P>(path_arg: P) -> io::Result<()> 
        where P: AsRef<std::path::Path>
    {
        let path: &std::path::Path = path_arg.as_ref();
        //  ...
        Ok(())
    }


    //  std::path::Path methods:
    //
    //  Path::new(str)
    //      Convert &str/&OsStr to &Path
    //
    //  path.parent()
    //      Return the parent directory
    //
    //  path.file_name()
    //      Returns the last component of path
    //
    //  path.is_relative()
    //  path.is_absolute()
    //      Returns whether the path is relative/absolute
    //
    //  path1.join(path2)
    //      Join two paths, returning new PathBuf
    //
    //  path.components()
    //      Return an iterator over components of path
    //
    //  path.exists()
    //  path.is_file()
    //  path.is_dir()
    //  path.read_dir()
    //  path.canonicalize()
    //  <>
    //
    //  path.to_str()
    //      Convert Path to str
    //
    //  path.to_string_lossy()
    //      Convert Path to str, replacing invalid characters
    //
    //  path.display()
    //      Return value implementing 'std::fmt::Display' for printing path


    //  'std::fs' filesystem functions:
    //  Designed for consistent behaviour across Unix/Windows
    //  Implementation is <performed> by system calls
    //  (return io::Result<()> unless otherwise noted)
    //
    //      create_dir(path)
    //      create_dir_all(path)
    //          create a single/multi-level directory
    //
    //      remove_dir(path)
    //      remove_dir_all(path)
    //          remove a single/muti-level directory
    //
    //      copy(src, dest) -> Result<u64>
    //          return size of copied file in bytes
    //
    //      remove_file(path)
    //      rename(src, dest)
    //      hard_link(src, dest)
    //      
    //      canonicalize(path) -> Result<PathBuf>
    //          realpath
    //
    //      metadata(path) -> Result<Metadata>
    //          stat
    //
    //      symlink_metadata(path) -> Result<Metadata>
    //          lstat
    //
    //      read_dir(path) -> Result<ReadDir>
    //          return a stream of items in a directory
    //
    //      read_link(path) -> Result<PathBuf>
    //          readlink
    //      
    //      set_permissions(path, perm)
    //          chmod


    //  Reading directories:
    //  Use 'std::fs::read_dir' or 'Path.read_dir()'
    //  ('.' and '..' are not listed)
    let path = std::env::temp_dir();
    for entry_result in path.read_dir().unwrap() {
        let entry: std::fs::DirEntry = entry_result.unwrap();
        let filename: std::ffi::OsString = entry.file_name();
        let filepath: std::path::PathBuf = entry.path();
        let filetype: std::fs::FileType = entry.file_type().unwrap();
        let metadata: std::fs::Metadata = entry.metadata().unwrap();
    }

    //  Platform specific features:
    //  There is no portable symlink function, since windows cannot handle symlinks

    //  'std::os' provides <various> platform specific features (only available on said platform)
    //  The '[#cfg]' attribute indicates conditional compilation

    //  Example: recursively copy a directory
    //  (uses library 'symlink()' for Unix, and provides said function as an error otherwise)
    use std::fs;
    use std::io;
    use std::path::Path;
    fn copy_dir_to(src: &Path, dest: &Path) -> io::Result<()> {
        //  Provide 'symlink()' for unix
        #[cfg(unix)]
        use std::os::unix::fs::symlink;
        //  Provide 'symlink()' for non-unix
        #[cfg(not(unix))]
        fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q) -> std::io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other,
                               format!("can't copy symlink {}", src.as_ref().display())));
        }
        fn handle_copy(src: &Path, src_type: &fs::FileType, dest: &Path) -> io::Result<()> {
            if src_type.is_file() {
                fs::copy(src, dest)?;
            } else if src_type.is_dir() {
                copy_dir_to(src, dest);
            } else if src_type.is_symlink() {
                let target = src.read_link()?;
                symlink(target, dest);
            } else {
                return Err(io::Error::new(io::ErrorKind::Other,
                                          format!("Can't copy: {}", src.display())));
            }
            Ok(())
        }
        if !dest.is_dir() {
            fs::create_dir(dest)?;
        }
        for entry_result in src.read_dir()? {
            let entry = entry_result?;
            let file_type = entry.file_type()?;
            handle_copy(&entry.path(), &file_type, &dest.join(entry.file_name()))?;
        }
        Ok(())
    }

    //  Prelude to enable unix extensions
    //use std::os::unix::prelude::*;

    println!("example_files_and_directories, DONE");
}


fn example_Networking()
{
    //  <>

    println!("example_Networking, DONE");
}


fn main() 
{
    example_IO_trait_definitions();
    example_Reading_Lines();
    example_Emulating_BufReaderBufWriter();
    example_Collecting_Lines();
    example_Writers();
    example_TempDir();
    example_Files();
    example_Seeking();
    example_Other_ReaderWriterTypes();
    example_Other_IO_Crates();
    example_files_and_directories();
    example_Networking();
}

