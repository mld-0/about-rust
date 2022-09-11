
////  wildcard allows all public names of a module to be made available
////  <(A module named 'prelude' denotes that it is a collection of things it is useful to make available)>
////  <(every rust program has an implicit 'use std::prelude::*')>
//use iron::prelude::*;
//  Ongoing: 2022-09-12T01:28:52AEST does our raw string have leading whitespace on each line from indendation?

//  raw string syntax, r#""#, any number of '#' can be used (same number at start and end)
//  (best practice is to use more '#' than will appear consecutively in the string)

//  <(Beginning a parameter name with '_' warns Rust we expect the variable to be unused

//	Match expression:
//	<(Functions like an if-statement, equal to the block matching the value of the given variable 'res')>
//		match res {
//			Ok(success) => {...},
//			Err(error) => {...}
//		}

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

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
