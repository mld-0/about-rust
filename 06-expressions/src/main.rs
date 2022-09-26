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

    println!("example_loops, DONE");
}


fn example_return() 
{
    //  'return' exits the current function, returning a value to the caller
    //  return without a value is shorthand for 'return ()'

    //  Use the '?' operator to check for errors after calling a function that can fail

}


fn main() 
{
    example_blocks_and_semicolons();
    example_declarations();
    example_if_and_match();
    example_loops();
}

