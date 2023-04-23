###

``` rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");

  // update
  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  // in this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2.
  println!("{}", user1.username) // error
}
```

But active and sign_in_count are types that implement the **Copy trait**, so ```user1.active``` still works

### Tuple Structs

``` rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```
