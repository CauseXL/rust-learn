###

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

### Packages and Crates

- A crate is the smallest amount of code that the Rust compiler considers at a time. 
- A crate can come in one of two forms: a binary crate or a library crate. 
- A package is a bundle of one or more crates that provides a set of functionality. 
- A package contains a Cargo.toml file that describes how to build those crates.
- A package can contain as many binary crates as you like, but at most only one library crate. 

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public

- If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. 

``` rust
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}
```

- In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword

``` rust
mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}
```

### use keyword

- Adding use and a path in a scope is similar to creating a symbolic link in the filesystem
- with use also check privacy, like any other paths.

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}
```

- use only creates the shortcut for the particular scope in which the use occurs.
- a different scope than the use statement, so the function body won’t compile:

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// why not use crate::front_of_house::hosting::add_to_waitlist;
use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // why not use add_to_waitlist();
        hosting::add_to_waitlist();
    }
}
```

- It's idiomatic to bring a function into scope with it’s parent module
- unclear as to where add_to_waitlist is defined.

- On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. 

``` rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- The exception to this idiom is if we’re bringing two items with the same name into scope with use statements


``` rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

- Providing New Names with the as Keyword


``` rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Using Nested Paths to Clean Up Large use Lists


``` rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// --snip--
use std::{cmp::Ordering, io};
// --snip--
```


``` rust
use std::io;
use std::io::Write;

// merge to
use std::io::{self, Write};
```

### The Glob Operator

- The glob operator is often used when testing to bring everything under test into the tests module

``` rust
use std::collections::*
```
