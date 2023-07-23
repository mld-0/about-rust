# A Bad Stack

## Basic Data Layout:

Invalid: recursive type `List` has infinite size
```rust,compile_fail,mdbook-runnable
#fn main() {
    pub enum List {
        Empty,
        Elem(i32, List)     //  no way to know how large this recurive expression is
    }
#}
```
Rust requires that we use 'indirection' - Box, Rc, or & to make our type representable

(note the compiler error makes this suggestion)








