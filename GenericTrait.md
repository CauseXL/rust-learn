## Generic

### in function

``` rust
fn largest(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
```

### in struct

``` rust
struct Point<T, U> {
  x: T,
  y: U,
}
```

### in enum

``` rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

### in method

``` rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
```

## Trait

- similar to interfaces in other languages

``` rust
pub trait Summary {
  fn summarize(&self) -> String;
}

// implementation

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

// use
use aggregator::{Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
}

```

### Default Implementations

``` rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}
```

### Traits as Parameters

``` rust
pub fn notify(item: &impl Summary) {
  println!("{}", item.summarize());
}

pub fn notify(item: &(impl Summary + Display)) {}
// sugar for a longer form known as a trait bound; it looks like this:
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary + Display>(item: &T) {}
```

### short bounds

``` rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// shorten
fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{}
```

### Returning Types that Implement Traits

``` rust
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}
```







