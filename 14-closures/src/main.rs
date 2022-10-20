//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
#![allow(non_snake_case)]
//  Ongoings:
//  {{{
//  }}}
use std::fmt;

fn example_closures_into()
{
    //  Closures (lambdas) are anonymous helper functions
    //  Rust infers the argument/return type

    //  Various standard library features accept closures
    //      map/filter
    //      <(sort)>
    //      thread::spawn

    struct City {
        name: String, population: i64, country: String,
    }
    impl City {
        fn new<T,U>(name: T, population: i64, country: U) -> Self 
            where T: Into<String>, U: Into<String>
        {
            City { name: name.into(), population, country: country.into(), }
        }
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
    impl std::fmt::Debug for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} ({})", self.name, self.population)
        }
    }

    fn is_sorted<T: Ord>(data: &[T]) -> bool {
        data.windows(2).all(|w| w[0] >= w[1])
    }

    fn sort_cities(cities: &mut Vec<City>) {
        cities.sort_by_key(|city| -city.population);
    }

    let mut cities: Vec<City> = Vec::<City>::new();
    cities.push(City::new("Hamilton", 178_500, "NZ"));
    cities.push(City::new("Christchurch", 380_600, "NZ"));
    cities.push(City::new("Hastings", 50_100, "NZ"));
    cities.push(City::new("Auckland", 1_463_000, "NZ"));
    cities.push(City::new("Wellington", 215_900, "NZ"));

    sort_cities(&mut cities);
    println!("cities=({:?})", cities);

    let pops = cities.get_pops();
    assert!(is_sorted(&pops));

    println!("example_closures_into, DONE");
}

//  <>

fn main() 
{
    example_closures_into();
}

