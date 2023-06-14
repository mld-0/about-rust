//	Ongoing: 2022-09-12T01:52:26AEST doesn't work?

//  Make crate cited in Cargo.toml available
extern crate iron;
use iron::status;
//  wildcard allows all public names of a module to be made available
//  <(A module named 'prelude' denotes that it is a collection of things it is useful to make available)>
//  <(every rust program has an implicit 'use std::prelude::*')>
use iron::prelude::*;

//  Make crate cited in Cargo.toml available along with its macros
#[macro_use]
extern crate mime;

extern crate router;
use router::Router;

extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn gcd(mut n: u64, mut m: u64) -> u64 {
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

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    //  Create a <> server, and set it to listening on TCP port 3000
    println!("Serving on 'http://localhost:3000' ...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

//  <(Beginning a parameter name with '_' warns Rust we expect the variable to be unused
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    //  Ongoing: 2022-09-12T01:28:52AEST does our raw string have leading whitespace on each line from indendation?
    //  raw string syntax, any number of '#' can be used (same number at start and end)
    //  (best practice is to use more '#' than will appear consecutively in the string)
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD</button>
        </form>
    "#,
    );
    //  Return 'response' as a result type
    Ok(response)
}

//	Match expression:
//	<(Functions like an if-statement, equal to the block matching the value of the given variable 'res')>
//		match res {
//			Ok(success) => {...},
//			Err(error) => {...}
//		}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        "The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d
    ));
    Ok(response)
}

