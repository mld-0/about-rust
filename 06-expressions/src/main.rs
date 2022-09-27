//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-09-26T12:33:32AEST Rust do-while loop expression?
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

fn gcd(mut n: u64, mut m: u64) -> u64 {
//  {{{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
//  }}}

//  Expression:
//  5 * (fahr-32) / 9

//  Statement:
//  for (; begin != end; ++begin) {
//      if (*begin == target) { break; }
//  }

//  Expressions have values (statements do not(?))
//  Rust does not have the ternary operator (expr ? expr2 : expr3), as an if/match statement handles both cases

//  Blocks are expressions, they produce a value and can be used anywhere a value is needed
//  The value of a block is the last expression (without a semicolon)
//  The default value of a block is '()' (used if there is no final value without a semicolon)
fn example_blocks_and_semicolons() 
{
    let post_author = Some("asdf");
    let code = match post_author {
        Some(author) => 1,
        None => 2,
    };

    //  An empty statement is a stray semicolon:
    ;

    println!("example_blocks_and_semicolons, DONE");
}


fn example_declarations() 
{
    //  let declarations declare a variable (the type and initalizer are optional).
    //  (a variable cannot be used before it is initalized)
    //   let name: type = expr;

    //  'name' does not need to be declared mutable, because it is initalized exactly once
    let user = "asdf";
    let name;
    if user == "abc" {
        name = user;
    } else {
        name = "def";
    }


    //  Rust allows us to re-declare <(shadow)> variables
    let lines = vec!["Line One", "Line Two", "Line 3", "Line Four"];
    for line in lines {
        let line = line.to_string();
    }

    //  A block can contain item declarations (functions, structs, <ect>)
    //  A nested function cannot access variables from its containing function
    fn f_i(x: i32) {
        //let a = user;                     //  invalid
    }
    //  Use a closure if one needs access to the enclosing scope

    println!("example_declarations, DONE");
}


fn example_if_and_match() 
{
    //  each condition of of an if statement must be of type bool
    //  (Rust will not implicitly convert numbers/pointers to bool)
    if false {
        ;
    } else if true {
        ;
    } else {
        ;
    }

    //  A match expression is a more flexible version of the switch statement
    //  '_' matches everything, making it a default case (where is must be last)
    //  The compiler implements match as a jump table (making it more efficent than an extended if-statement)
    let code = 53;
    let msg = match code {
        0 => "code 0",
        1 => "code 1",
        53 => "code 53",
        _ => "invalid",
    };
    println!("msg=({})", msg);

    let name = Some("asdf");
    let msg = match name {
        Some(x) => println!("Hello name=({})", x),
        None => println!("Begone stranger"),
    };

    //  Rust prohibits match expressions that do not cover all possible values
    let card = "King";
    //let score = match card {                  //  invalid, does not cover all possible cases
    //    "Jack" => 11,
    //    "Queen" => 12,
    //};

    //  All blocks of an if/match expression must produce values of the same type
    let name = "Ian";
    //let number = if name == "Ian" { "eleventy-one" } else { 7 };      //  invalid, incompatible types

    //  if-let statements are shorthand for a match with one pattern
    if let "Ian" = name {
        println!("Ian");
    } else {
        println!("Not-Ian");
    }
    match name {
        "Ian" => println!("Ian"),
        _ => println!("Not-Ian"),
    }

    println!("example_if_and_match, DONE");
}

fn example_loops() 
{
    //  There are four looping expressions:
    //  while condition { block }
    //  while let pattern = expr { block }
    //  loop { block }
    //  for item in collection { block }

    //  'condition' must have type bool
    //  Loops are blocks, but their value is ()

    //  Emulate a do-while loop with:
    //  loop { block; if !condition { break; } }

    //  Use the '..' operator to write for loops
    for i in 0..20 { print!("{}, ", i); }
    println!();

    let s = vec!["abc".to_string(), "def".to_string(), "hij".to_string()];

    //  A for loop over a value consumes that value:
    for x in s { print!("x=({}), ", x); } println!();
    //println!("s.len()=({})", s.len());            //  invalid, use of 's' after move

    let mut s = vec!["abc".to_string(), "def".to_string(),];

    //  Use a reference to that value for a loop that does not consume the value
    //  Iterating over a reference produces a reference to each item in the collection
    for x in &s { print!("x=({}), ", x); } println!();
    println!("s.len()=({})", s.len());

    //  Iterating over a mutable reference produces a mutable reference to each item in the collection
    for x in &mut s { x.push('1'); }

    //  'break' exists an enclosing loop
    //  'continue' jumps to the beginning of the next iteration

    //  A loop can be labeled with a lifetime
    //  Using this label with break/continue exits/restarts that loop, not the innermost enclosing loop
    'loop_search:
    for x in s {
        for i in 0..10 {
            print!("(x,i)=({},{}), ", x, i);
            if i == 2 { continue 'loop_search; }
        }
    } println!();

    //  Rust does not examine loop conditions, its flow analysis assumes that any condition can be either true or false
    //  for this reason, it provides the 'loop { block }' expression

    //  Expressions that never finish (infinite loop, call to 'exit()') in a usual way have special type '!'
    fn loop_forever() -> !  {
        loop {}
    }

    println!("example_loops, DONE");
}


fn example_return() 
{
    //  'return' exits the current function, returning a value to the caller
    //  return without a value is shorthand for 'return ()'

    //  Use the '?' operator to check for errors after calling a function that can fail

    //  A value can also be returned by specifying it on the last line without a semicolon
    //  (this is the best-practice way for the default returned value)

    println!("example_return, DONE");
}


fn example_function_calls() 
{
    //  Rust makes a distinction between references and values
    //  It is an error to pass a value where a reference is expected and vice-versa
    //  The exception is the '.' operator, which automatically dereference or borrows as required
    let mut x = vec![1,2,3];

    //  Function call:
    let z = gcd(1302, 462);

    //  Method call:
    let a = x.contains(&2);

    //  Static method call:
    let mut numbers = Vec::<i32>::new();
    //  The '::<T>' symbol is known as the turbofish

    println!("example_function_calls, DONE");
}


fn example_fields_and_elements()
{
    struct Game { white_pawns: i32, black_pawns: i32 };
    let mut game = Game { white_pawns: 5, black_pawns: 7 };
    let mut t = (1,2,3);
    let mut v = vec![1,2,3];


    //  Access struct field
    assert_eq!(game.black_pawns, 7);

    //  Access tuple element (can only be constant, not a variable)
    assert_eq!(t.1, 2);

    //  Access array/slice/vector element:
    assert_eq!(v[1], 2);

    //  These produce lvalues (if the variable is mutable)
    game.black_pawns = 5;
    t.1 = 7;
    v[1] = 9;

    //  Extracting a slice from an array/vector
    let a = &v[0..3];
    //  The containing variable is considered borrowed for the lifetime of the slice
    //  The range must not extend beyond the size of the container
    //  Rust does not support negative indexing

    //  The '..' operator allows either operand to be omitted
    let a = &v[..];
    let a = &v[..2];
    let a = &v[0..];
    //  Ranges are half-open, 0..4 = 0,1,2,3

    println!("example_fields_and_elements, DONE");
}


fn example_reference_operators() 
{
    //  Address-of operators: '&' and '&mut'
    //  Dereference operator: '*'
    println!("example_reference_operators, DONE");
}


fn example_operators() 
{
    //  Rust uses the traditional arithemetic operators: +, -, *, /, %

    //  Integer overflow causes a panic in debug builds
    //  Use 'a.wrapping_add(b)' for unchecked wrapping arithemetic

    //  Division by zero triggers a panic in debug and release builds
    //  Use 'a.checked_div(b)' (returns an Option) for division that never panics

    //  Unary '-' negates a number. There is no unary '+'

    //  '%' computes modulus. The result has the same sign as the left operand. It supports integers and floats.

    //  Rust uses the traditional bitwise integer operators: &, |, ^, <<, >>
    //  It uses ! instead of ~ for bitwise-not

    //  Bit-shifting is sign-extending on signed integers and zero-extending on unsigned integers

    //  Unlike C, bitwise operations have higher precedence than comparisons

    //  Rust uses the traditional comparison operators: ==, !=, <, <=, >, >=
    //  Both values must have the same type

    //  Rust has the short-circuting logical operators && and ||
    //  Both operands must have type bool
    println!("example_operators, DONE");
}


fn example_assignment()
{
    //  Assign to mutable variables with '='
    //  Assignment moves non-copyable values
    //  Rust supports compound assignment: +=, -=, *=, ect.
    //  Unlike C, Rust does not support chaining assignment.
    //  Rust does not support ++ / --
    println!("example_assignment, DONE");
}

fn example_type_casts() 
{
    //  Rust requires explicit casts with the 'as' keyword

    //  Numbers can be cast between any builtin numeric types
    //  Casting between integer types are always well defined

    //  Converting to a narrower type results in truncation
    //  Casting signed integers to a wider type is sign-extended, unsigned integers are zero extended

    //  casting too-large floating point values to integers <has/does> cause undefined behaviour

    //  bool/char/C-like-enum can be cast to integers
    //  The reverse is generally not allowed (u8 to char being the exception)
    //  (there are methods, like 'std::char::from_u32()' to perform such conversions)

    println!("example_type_casts, DONE");
}


fn main() 
{
    example_blocks_and_semicolons();
    example_declarations();
    example_if_and_match();
    example_loops();
    example_return();
    example_function_calls();
    example_fields_and_elements();
    example_operators();
    example_assignment();
    example_type_casts();
}

