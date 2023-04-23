###

``` rust
enum IpAddrKind {
    V4,
    V6,
}
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

we can put data directly into each enum variant.

``` rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

``` rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// same as 
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); 
```

### Option

- Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>

``` rust
enum Option<T> {
    None,
    Some(T),
}
```

### Match

``` rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns That Bind to Values

``` rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska))
```

### Matching with Option<T>

``` rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
```


``` rust
const dice_roll: i32 = 9;

fn move_player(x: i32) -> i32 {}

fn add_player(x: i32) -> i32 { 
  match dice_roll {
    3 => 3,
    other => move_player(other),
  }
}
```

- Catch-all Patterns and the _ Placeholder

``` rust
const dice_roll: i32 = 9;

fn add_player(x: i32) -> i32 { 
  match dice_roll {
    3 => 3,
    (_) => (),
  }
}
```

### Concise Control Flow with if let

``` rust
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("{max}"),
  _ => (),
}

// same as
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("{max}")
}
```

Using if let means less typing, less indentation, and less boilerplate code. However, you **lose the exhaustive checking that match enforces**. 

Choosing between match and if let depends on what you’re doing in your particular situation and whether **gaining conciseness** is an appropriate trade-off for losing **exhaustive checking**.

In other words, you can think of if let as **syntax** sugar for a match that runs code when the value **matches one pattern** and then **ignores all other values**.

### if else

``` rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

// same as
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("State quarter from {:?}!", state);
} else {
  count += 1;
}
```
