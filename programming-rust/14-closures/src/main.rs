//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-26T23:28:05AEDT example_capturing_variables, (are examples losing something by using 'usize' as sorting key (instead of a (presumedly non-copy) struct)?)
//  Ongoing: 2022-10-26T23:29:37AEDT example_capturing_variables, (please provide the rule that distinguishes closure borrowing from stealing)
//  Ongoing: 2022-10-26T23:45:25AEDT (surely) Rust provides something akin to 'GetRowVec' convert a Vec<Struct> to Vec<Field>
//  Ongoing: 2022-10-26T23:50:26AEDT 'get_statistic' can't be by-reference (or else 'stat' must be passed as '&stat')(?)
//  Ongoing: 2022-10-26T23:57:47AEDT closure using values that habe been captured (as arguments) vs not
//  Ongoing: 2022-10-27T00:10:03AEDT is it possible to mix moving/borrowing values into a closure
//  Ongoing: 2022-10-27T00:10:24AEDT (please clarify) closures capturing values by-reference (despite no '&' being present in the body of the lambda for said variable?)
//  Ongoing: 2022-10-27T00:10:58AEDT (to clairfy) -> book repo (implementation for 'City' / 'Stat'?)
//  Ongoing: 2022-10-27T00:20:40AEDT a function like 'get_statistic' -> prospect of / how to handle returning of multiple types (book repo implementation?)
//  Ongoing: 2022-10-27T00:39:56AEDT acceptance of lambda as function -> (a case of implict conversion?) [...] (significance of (capital-F 'Fn')?)
//  Ongoing: 2022-10-27T00:55:24AEDT can't pass lambda with no parameter/return types to 'fn' parameter (as we have above with closures with both those things)(?) [...] (the problem is the 'FnOnce' nature of the lambda (contains 'drop()'))
//  Ongoing: 2022-10-27T01:05:52AEDT debug_dump_dict_ii -> moves a value (not included as parameter) into a closure (without specifying 'move') 
//  Ongoing: 2022-10-27T01:46:16AEDT '|_|' as lambda parameter
//  Ongoing: 2022-10-27T01:56:58AEDT how '|_| get_form_response()' and '|req| get_gcd_response(req)' have different types (both recieve '&Request' and return 'Response' (why aren't they both 'Fn(&Request) -> Response' (why do we have to box them))
//  Ongoing: 2022-10-27T01:57:39AEDT '|req| get_gcd_response(req)' (example of passing to lambda a ref using by-val syntax) (which we ruled out <earlier>)
//  Ongoing: 2022-10-27T16:25:39AEDT A closure borrows a shared reference to any variable it uses (is the variable in the closure effectively as reference?)
//  Ongoing: 2022-10-27T16:26:30AEDT <(Rust calls them closures, not references, because closures are about capturing variables)>
//  Ongoing: 2022-10-27T16:29:37AEDT 'increment_x' can be declared as non-mut, but cannot be called unless it is declared mut ... (when must / what does it mean for a closure be mutable)
//  Ongoing: 2022-10-27T16:41:43AEDT closures coerce to function pointers if and only if they do not capture any variables from their environment -> (meaning no mutable references or no references?)
//  }}}

//  Continue: 2022-10-27T00:45:29AEDT Rules of capturing variables (by-val/by-ref) but clear this time

//  LINK: https://medium.com/coding-rust/best-explanation-of-closure-in-rust-2b20210eba53
//  {{{
fn about_regular_closures()
{
    //  closures: Rust allows us to create anonymous functions (lambdas)
    let add_one = |x| { 1 + x };
    assert_eq!(add_one(5), 6);

    //  Syntax:
    let add_one = |x: i32| -> i32 { 1 + x };
    fn  add_one   (x: i32) -> i32 { 1 + x };

    //  Like functions, closures infer their argument type(s)
    //  Unlike functions, closures infer their return type

    //  A closure has access to variables in the scope where it is defined
    //  The closure borrows a shared reference to any variable it uses
    let mut x: i32 = 5;
    let printer = || { println!("x=({})", x); };
    //x = 7;                        //  error, 'x' is borrowed, cannot assign to it
    printer();

    //  <(The closure borrows a mutable reference to any variable it modifies?)>
    //  note: original 'x' still exists inside 'printer()', new 'x' shadows original outside closure
    let mut x: i32 = 5;
    let mut increment_x = || { x += 1; };
    //x = 4;                        //  error, 'x' is borrowed mutably, cannot assign to it
    //assert!(x == 5);              //  error, 'x' is borrowed mutable, cannot borrow shared reference
    increment_x();
    println!("x=({})", x);

    //  A non-move closure can't be freely moved outside its original scope (it contains references to local variables)
    //  (they are <generally> not suitable to be returned)

    println!("best_explanation_of_closure, DONE");
}

fn about_moving_closures()
{
    //  A moving closure always takes ownership of all variables it uses
    //      move || { ... }

    //  Moving closures are useful for concurrency

    println!("about_moving_closures, DONE");
}

fn about_closures_as_arguments()
{
    //  Closures are most useful as arguments to functions
    fn double_result<F>(x: i32, f: F) -> i32 
        where F: Fn(i32) -> i32
    {
        f(x) + f(x)
    }
    let square = |x: i32| { x * x };
    assert_eq!(50, double_result(5, square));
    assert_eq!(50, double_result(5, |x| { x * x }));

    //  <(functions and closures are different types)>

    //  A named function can be used wherever the equivalent closure is accepted
    fn square_f(x: i32) -> i32 { x * x };
    assert_eq!(50, double_result(5, square_f));

    //  Each closure has its own unique type - the definition of the closure is part of its type.
    //  (two closures with the same parameter types and return types do not have the same type).
    fn composite<F,G>(x: i64, f: F, g: G) -> i64 
        where F: Fn(i64) -> i64,
              G: Fn(i64) -> i64,
    {
        g(f(x))
    }
    assert_eq!(17, composite(5, |n| { n * 2 }, |n| { n + 7 }));

    //  <(closures coerce to function pointers if and only if they do not capture any variables from their environment)>

    println!("about_closures_as_arguments, DONE");
}

//  Closures are syntactic sugar for a struct of variables which implements one of 'Fn' / 'FnMut' / 'FnOnce'

//  }}}

//  LINK: https://doc.rust-lang.org/book/ch13-01-closures.html
//  {{{
//  }}}

//  Closure type is determined by what closure does with captured values (not how they are captured)
//  'Fn': closures can be called multiple times without restriction. <(This includes all 'fn' functions)>
//  'FnMut': closures that can be called multiple times if declared mutable (can mutate environment)
//  'FnOnce': closures that can be called once (variables owned by closure can be moved out of closure)

use std::fmt;
use std::collections::HashMap;

#[derive(Copy,Clone)]
enum Statistic { 
    Population, 
}
struct City {
    name: String, population: i64, country: String, monster_attack_risk: i64,
}
impl City {
    fn new<T,U>(name: T, population: i64, country: U) -> Self 
        where T: Into<String>, U: Into<String>
    {
        City { name: name.into(), population, country: country.into(), monster_attack_risk: 0, }
    }
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
        }
    }
}
impl std::fmt::Debug for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.population)
    }
}
fn get_example_cities() -> Vec<City> {
    let mut cities: Vec<City> = Vec::<City>::new();
    cities.push(City::new("Hamilton", 178_500, "NZ"));
    cities.push(City::new("Christchurch", 380_600, "NZ"));
    cities.push(City::new("Hastings", 50_100, "NZ"));
    cities.push(City::new("Auckland", 1_463_000, "NZ"));
    cities.push(City::new("Wellington", 215_900, "NZ"));
    cities[0].monster_attack_risk = 0;
    cities[1].monster_attack_risk = 3;
    cities[2].monster_attack_risk = 0;
    cities[3].monster_attack_risk = 1;
    cities[4].monster_attack_risk = 9;
    cities
}

trait GetRowVec {
    fn get_pops(&self) -> Vec<i64>;
}
impl GetRowVec for Vec<City> {
    fn get_pops(&self) -> Vec<i64> {
        let mut result = Vec::<i64>::new();
        for city in self {
            result.push(city.population);
        }
        result
    }
}

fn get_dict() -> HashMap<String,String>
{
    let mut result = HashMap::<String,String>::new();
    result.insert("abc".to_string(), "ABC".to_string());
    result.insert("def".to_string(), "DEF".to_string());
    result
}


fn example_closures_into()
{
    //  Closures (lambdas) are anonymous helper functions
    //  Rust infers the argument/return type
    //  Form: |x| f(x)

    //  Various standard library features accept closures
    //      map/filter
    //      <(sort)>
    //      thread::spawn

    fn is_sorted<T: Ord>(data: &[T]) -> bool {
        data.windows(2).all(|w| w[0] >= w[1])
    }

    fn sort_cities(cities: &mut Vec<City>) {
        cities.sort_by_key(|city| -city.population);
    }

    let mut cities = get_example_cities();
    println!("cities=({:?})", cities);
    sort_cities(&mut cities);
    println!("cities=({:?})", cities);

    let pops = cities.get_pops();
    assert!(is_sorted(&pops));

    println!("example_closures_into, DONE");
}

fn example_capturing_variables()
{
    use std::thread;

    //  <(Rules of capturing variables (by-val/by-ref) but clear this time:)>
    //  {{{
    //  <>
    //  }}}
    //  <(book says: lambda values (whether they are parameters or not) are used by-ref, unless the keywork 'move' is used(?))>

    //  <(Closures that borrow:)>
    //  Closures can use data belonging to the enclosing <function/scope>
    fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
        //  <(requires 'stat' to be a copy type?)>
        cities.sort_by_key(|x| -x.get_statistic(stat));
    }

    let mut cities = get_example_cities();
    let stat = Statistic::Population;
    println!("cities=({:?})", cities);
    sort_by_statistic(&mut cities, stat);
    println!("cities=({:?})", cities);

    //  <(Closures that steal:)>
    //  Use the keyword 'move' to move values into closure instead of borrowing references to them
    //  <(applies both to arguments, and variables used but not given as arguments?)>
    fn start_sort_by_statistics_thread(mut cities: Vec<City>, stat: Statistic) 
        -> thread::JoinHandle<Vec<City>>
    {
        //  Because 'stat' and 'cities' live beyond the end of the function, we must move them into the closures
        let key_fn = move |x: &City| -> i64 { -x.get_statistic(stat) };
        thread::spawn(move || {
            cities.sort_by_key(key_fn);
            cities
        })
    }
    //  <(usage of 'start_sort_by_row_thread')>

    //  <(Closures that attempt to move copyable types perform a copy instead?)>

    println!("example_capturing_variables, DONE");
}

fn example_function_and_closure_types()
{
    //  Since closures can be used as values, they have types

    //  Type: 'fn(&City) -> i64'
    fn city_population_descending(city: &City) -> i64 {
        -city.population
    }
    fn city_monster_attack_descending(city: &City) -> i64 { 
        -city.monster_attack_risk
    }
    fn has_monster_attacks(city: &City) -> bool {
        city.monster_attack_risk > 0
    }

    //  Type: '|&City| -> bool'
    let f = |city: &City| city.monster_attack_risk > 0;

    let by_population = true;

    //  Type: 'fn(&City) -> i64'
    //  (a fn value is a function pointer)
    let key_func: fn(&City) -> i64 = 
        if by_population {
            city_population_descending
        } else {
            city_monster_attack_descending
        };

    //  Functions can take other functions as arguments
    fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }
    let mut cities = get_example_cities();
    let at_risk = count_selected_cities(&cities, has_monster_attacks);
    println!("at_risk=({})", at_risk);

    //  Closures do not have the same type as functions
    //  (closures coerce to function pointers if and only if they do not capture any variables from their environment)
    //  <(book has example below not work)>
    let at_risk_ii = count_selected_cities(&cities, |city| city.monster_attack_risk > 0);
    assert_eq!(at_risk, at_risk_ii);
    let f = |city: &City| city.monster_attack_risk > 0;
    let at_risk_ii = count_selected_cities(&cities, f);
    assert_eq!(at_risk, at_risk_ii);

    //  <(book provides alternative function signature for accepting closure or function)>
    fn count_selected_cities_alt<F>(cities: &Vec<City>, test_fn: F) -> usize 
        where F: Fn(&City) -> bool
    {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }
    let at_risk_iii = count_selected_cities_alt(&cities, |city| city.monster_attack_risk > 0);
    assert_eq!(at_risk, at_risk_iii);

    println!("example_function_and_closure_types, DONE");
}

fn example_closure_performance()
{
    //  Rust closures are built for performance

    //  closures in memory:
    //  It is not neccessary to store the pointer to the function, that is contained in the closure's type
    //  Any references used by the closure are stored as addresses
    //  Any values moved into the closure are stored in the closure

    println!("example_closure_performance, DONE");
}

fn example_closure_safety()
{
    let my_str = "hello".to_string();

    //  'FnOnce': Implemented by any closure that drops a value implements (instead of 'Fn')
    //  <(Other ways a closure can become 'FnOnce'?)>
    //  These closures are themselves dropped the first time they are called
    let f = || drop(my_str);
    f();
    //f();                      //  error, cannot use 'my_str' after dropping it

    fn call_twice<F>(f: F) where F: Fn() {
        f(); f();
    }
    let my_str = "hello".to_string();
    let f = || drop(my_str);
    //call_twice(f);            //  error, 'F' must implement FnOnce

    fn call_twice_ii<F>(f: F) where F: FnOnce() {
        f(); 
        //f();                  //  error, cannot call 'FnOnce' function twice
    }

    let my_dict = get_dict();
    let debug_dump_dict_i = || { for (k,v) in &my_dict { print!("{:?}: {:?}, ", k,v); } println!(); };
    debug_dump_dict_i();

    //  It is possible to accidently write closures that consume values (note how 'my_dict' is iterated over by-val)
    //  Rust automatically makes such closures 'FnOnce'
    let debug_dump_dict_ii = || { for (k,v) in my_dict { print!("{:?}: {:?}, ", k,v); } println!(); };
    //println!("dict=({:?})", my_dict);             //  error, 'my_dict' has been moved into closure
    debug_dump_dict_ii();
    //debug_dump_dict_ii();                         //  error, closure is 'FnOnce'

    println!("example_closure_safety, DONE");
}

fn example_fnMut()
{
    //  'FnMut': Any closures that requires mutable access to a value, but doesn't drop any values

    let mut i = 0;
    //  <(book example does declare 'incr' as 'mut')>
    let mut incr = || {
        i += 1;                 //  closure borrows a mutable reference to 'i'
        println!("i=({})", i);
    };

    //  Passing a FnMut closure:
    fn call_twice_i<F>(mut f: F) where F: FnMut() {
        f(); f();
    }
    call_twice_i(incr);

    fn call_twice_ii(f: fn()) {
        f(); f();
    }
    //call_twice_ii(incr);             //  error, cannot pass 'FnMut' as 'fn'

    call_twice_i(|| i += 1);
    //call_twice_ii(|| i += 1);         //  error

    println!("example_fnMut, DONE");
}

fn example_callbacks()
{
    //  A callback is a function passed to another function as argument

    //  <(classes 'Request' / 'Response' convolute example)>
    struct Request {
        //method: String, url: String, headers: HashMap<String,String>, body: Vec<u8>,
        url: String,
    }
    struct Response {
        //code: u32, headers: HashMap<String,String>, body: Vec<u8>,
    }

    type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
    struct BasicRouter { 
        routes: HashMap<String,BoxedCallback> 
    }

    impl BasicRouter {
        fn new() -> Self {
            Self { routes: HashMap::new() }
        }
        //  <(We Box our callbacks to allow different types)>
        //  `'static` is required to <store/Box> the closure (<asserting> that it does not contain borrowed references)
        fn add_route<C>(&mut self, url: &str, callback: C) 
            where C: Fn(&Request) -> Response 
                + 'static
        {
            self.routes.insert(url.to_string(), Box::new(callback));
        }
        fn handle_request(&self, request: &Request) -> Option<Response> {
            match self.routes.get(&request.url) {
                None => None,
                Some(callback) => Some(callback(request)),
            }
        }
    }

    fn get_form_response() -> Response {
        println!("get_form_response");
        let mut result = Response { };
        result
    }
    fn get_gcd_response(req: &Request) -> Response {
        println!("get_gcd_response");
        let mut result = Response { };
        result
    }

    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_response());
    router.add_route("/gcd", |req| get_gcd_response(req));

    let r = Request { url: "/".to_string() };
    router.routes["/gcd"](&r);
    router.routes["/"](&r);

    println!("example_callbacks, DONE");
}

fn example_closure_effective_use()
{
    //  Model-View-Controller (MVC) is a design pattern which does not play well with Rust's ownership model
    //  (Rust does not permit reference cycles - its radical wager is that good alternatives exist)

    //  <(Relevance to closures?)>



    println!("example_closure_effective_use, DONE");
}

fn main() 
{
    about_regular_closures();
    about_moving_closures();
    about_closures_as_arguments();

    example_closures_into();
    example_capturing_variables();
    example_function_and_closure_types();
    example_closure_safety();
    example_fnMut();
    example_callbacks();
    example_closure_effective_use();
}

