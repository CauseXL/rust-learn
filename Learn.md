##

``` html
<p>我在读完 <a href="https://link.juejin.cn?target=https%3A%2F%2Fdoc.rust-lang.org%2Fbook" target="_blank" title="https://doc.rust-lang.org/book" ref="nofollow noopener noreferrer">Rust 官方文档</a> 后，又跟着 <a href="https://link.juejin.cn?target=https%3A%2F%2Fdocs.microsoft.com%2Fen-us%2Flearn%2Fpaths%2Frust-first-steps%2F" target="_blank" title="https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/" ref="nofollow noopener noreferrer">Rust 第一步</a> 敲了一遍代码。发现这只是个开始，在看了一遍<a href="https://link.juejin.cn?target=https%3A%2F%2Fdoc.rust-lang.org%2Fnomicon%2F" target="_blank" title="https://doc.rust-lang.org/nomicon/" ref="nofollow noopener noreferrer">Rust Nomicon</a> <a href="https://link.juejin.cn?target=https%3A%2F%2Fnomicon.purewhite.io" target="_blank" title="https://nomicon.purewhite.io" ref="nofollow noopener noreferrer">Rust Nomicon 中文</a>和<a href="https://link.juejin.cn?target=https%3A%2F%2Frust-lang.github.io%2Fasync-book" target="_blank" title="https://rust-lang.github.io/async-book" ref="nofollow noopener noreferrer">Async Book</a>之后，就已经想放弃了。太多晦涩的内容了，比如：<code>Unsafe</code>, <code>PhantomData</code>,<code>Send and Sync</code>, <code>Pin</code>...</p>
```

### data types

- data type subsets: scalar and compound.
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.


### Array

- Have elements of the same type
- Have a fixed length.

``` rust
let mut a: [i8; 10] = [42; 10];
```


### Tuple

- Group together values of different types into a compound type.

``` rust
let t: (i8, bool) = (7, true);
println!("1st index: {}", t.0);
```

### Reference (borrow)

- The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

- like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

``` rust
fn main() {
  let mut x: i32 = 6;
  let ref_x: &mut i32 = &mut x;
  *ref_x  = 20;
  println!("x: {x}"); // 20
}
```

### Slice

``` rust
fn main() {
  let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
  let s: &[i32] = &a[2..4];
  println!("s: {s:?}");
}
```

### String vs str

- ```&str``` an **immutable** reference to a string slice.
- ```String``` a **mutable** string buffer.

``` rust
fn main() {
  let s1: &str = "World";
  let mut s2: String = String::from("Hello ");
  println!("s1: {s1}"); // World
  println!("s2: {s2}"); // Hello 
  s2.push_str(s1);
  println!("s2: {s2}"); // Hello World
  let s3: &str = &s2[6..];
  println!("s3: {s3}"); // World
}
```

### Function

- The last expression in a function body (or any block) becomes the return value.
- Default values are not supported:

``` rust
fn test(n: u32) -> () { // `-> ()` is normally omitted
  for i in 1..=n {
    println!("{i}");
  }
}
```

### Methods

``` rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn inc_width(&mut self, delta: u32) {
    self.width += delta;
  }
}

fn main() {
  let mut rect = Rectangle { width: 10, height: 5 };
  println!("old area: {}", rect.area());
  rect.inc_width(5);
  println!("new area: {}", rect.area());
}

```

### function vs methods

Rust functions and methods are both ways to define reusable code in Rust. However, there are some differences between the two:

- Functions are standalone pieces of code that take input arguments and return a value. They are not associated with any particular type or instance of a type.

- Methods, on the other hand, are functions that are associated with a particular type or instance of a type. They take  `self`  as their first parameter, which refers to the object the method is being called on.

``` rust
// This is a function that takes two integers and returns their sum
fn add(a: i32, b: i32) -> i32 {
  a + b
}

// This is a method that is associated with the `String` type
impl String {
  fn shout(&self) -> String {
    self.to_uppercase()
  }
}
fn main() {
  let result = add(1, 2);
  println!("Result of add: {}", result);

  let my_string = "hello".to_string();
  let shouted_string = my_string.shout();
  println!("Shouted string: {}", shouted_string);
}
```

### Statements and Expressions

``` rust
fn main() {
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {y}"); // 4
}
```

This expression:

``` rust
{
  let x = 3;
  x + 1
}
```
- Note that the x + 1 line doesn’t have a semicolon at the end. 
- Expressions do not include ending semicolons. 
- If you add a semicolon to the end of an expression, **you turn it into a statement**, and it will then not return a value.

Using if in a let Statement

- Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
- results from each arm of the if must be the same type

``` rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // 5
}
```

loop

``` rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 20
}
```

### const

### static

- the variable lives through the entirety of the program execution.

### Move Semantics

- An assignment will transfer ownership between variables:

``` rust
fn main() {
  let s1: String = String::from("Hello!");
  let s2: String = s1;
  println!("s2: {s2}");
  println!("s1: {s1}"); // error[E0382]: borrow of moved value: `s1`
}
```

The assignment of s1 to s2 transfers ownership.
The data was moved from s1 and s1 is no longer accessible.

### Moves in Function Calls

- When you pass a value to a function, **the value is assigned to the function parameter. This transfers ownership**:

``` rust
fn say_hello(name: String) {
  println!("Hello {name}")
}

fn main() {
  let name = String::from("Alice");
  say_hello(name);
  say_hello(name); // error[E0382]: use of moved value: `name`
}
```

With the first call to say_hello, main gives up ownership of name. Afterwards, name cannot be used anymore within main.


