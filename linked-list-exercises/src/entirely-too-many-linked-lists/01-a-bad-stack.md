# A Bad Stack

## Using enum:

Invalid: recursive type `List` has infinite size
```rust,compile_fail,mdbook-runnable
#fn main() {
pub enum List {
    Elem(i32, List)     //  no way to know how large this recursive expression is
    Empty,
}
#}
```
Rust requires that we use 'indirection' - Box, Rc, or & to make our type representable

(note the compiler error makes this suggestion)


We use `Box<T>` as indirection.

Box is the simplest form of heap allocation in Rust.

Suboptimal: an enum occupies the space of its largest element (that is, the trailing 'Empty' node is wasting space).
```rust,mdbook-runnable
#   fn main() {
#[derive(Debug)]
pub enum List<T> {
    Elem(T, Box<List<T>>),
    Empty,
}
let l: Box<List<i32>> = Box::new(List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty)))));
println!("l=({:?})", l);
#   }
```

## Using struct

```rust,mdbook-runnable
pub struct List<T> {
    head: Link<T>,
}
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#   fn main() {
#   }
```

Implementing `new()` to create an empty list is trivial

```rust,mdbook-runnable
#pub struct List<T> {
#    head: Link<T>,
#}
#enum Link<T> {
#    Empty,
#    More(Box<Node<T>>),
#}
#struct Node<T> {
#    elem: T,
#    next: Link<T>,
#}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
#   fn main() {
#   }
```

invalid: `push_front()` / `pop_front()`

```rust,mdbook-runnable
#pub struct List<T> {
#    head: Link<T>,
#}
#enum Link<T> {
#    Empty,
#    More(Box<Node<T>>),
#}
#struct Node<T> {
#    elem: T,
#    next: Link<T>,
#}
impl<T> List<T> {
    //pub fn push_front(&mut self, elem: T) {
    //    let new_node = Box::new(Node {
    //        elem: elem,
    //        next: self.head,    //  invalid, cannot move out of borrowed mut ref
    //    });
    //    self.head = Link::More(new_node);
    //}
    pub fn pop_front(&mut self) -> Option<T> {
        let result;
        match &self.head {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);   //  invalid, cannot move out of borrowed mut ref
                self.head = node.next;      //  invalid, cannot move out of borrowed mut ref
            }
        }
        result
    }
}
#   fn main() {
#   }
```

We cannot move out of `self` in a method borrowing it as a mutable reference (even though we assign a value to it again later).

Instead we can use `mem::replace` to accomplish this:

```rust,mdbook-runnable
#pub struct List<T> {
#    head: Link<T>,
#}
#enum Link<T> {
#    Empty,
#    More(Box<Node<T>>),
#}
#struct Node<T> {
#    elem: T,
#    next: Link<T>,
#}
use std::mem;
impl<T> List<T> {
    pub fn push_front(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
#   fn main() {
#   }
```

[{elaborate on this problem/solution?}]
[{this wouldn't be necessary if we were implementing push instead of push_front?}]
[{article doesn't even mention that what we are implementing is push_front/pop_front?}]



