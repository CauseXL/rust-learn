####

Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. 

This code does not compile!

``` rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

- when s2 and s1 go out of scope, they will both try to free the same memory. 
- This is known as a double free error.
- To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
- Therefore, Rust doesn’t need to free anything when s1 goes out of scope.

### Returning values can also transfer ownership

``` rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
  a_string  // a_string is returned and moves out to the calling function
}
```

### Reference and Borrowing

- they do not have ownership.

``` rust
fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
```

- The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
- Because it does not own it, the value it points to will not be dropped when the reference stops being used.


- It doesn’t work! When you try to modify something we’re borrowing.

``` rust
fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String) {
  // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  some_string.push_str(", world");
}
```

### Mutable References

``` rust
fn main() {
  let mut s = String::from("hello");
  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

- Mutable references have one big restriction: no more than one mutable references

``` rust
// This code does not compile!
  let mut s = String::from("hello");

  let r1 = &mut s;
  // cannot borrow `s` as mutable more than once at a time
  let r2 = &mut s;

  println!("{}, {}", r1, r2);
```

- use curly brackets to create a new scope will work

``` rust
  let mut s = String::from("hello");

  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let r2 = &mut s;
```

- We also cannot have a mutable reference while we have an immutable one to the same value.

``` rust
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  let r3 = &mut s; // BIG PROBLEM

  println!("{}, {}, and {}", r1, r2, r3);
```

- change to this will work

``` rust
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{}", r3);
```

- The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created.

### Dangling References

- a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. 
- if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.


### The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.