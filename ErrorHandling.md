### Unrecoverable Errors with panic!

``` rust
fn main() {
  panic!("crash and burn");
}

fn main() {
  let v = vec![1, 2, 3];
  v[99];
}
```

### Recoverable Errors with Result

``` rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

``` rust
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening file: {:?}", error),
  };
}
```

- nest match

``` rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    },
  };
}
```

``` rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
  });
}
```

### Shortcuts for Panic on Error: unwrap and expect

``` rust
use std::fs::File;

fn main() {
  let greeting_file = File::open("hello.txt").unwrap();
}
```

- ```expect``` can provide good error messages

``` rust
use std::fs::File;

fn main() {
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
```

### Propagating Errors

``` rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
      k(_) => Ok(username),
      Err(e) => Err(e),
  }
}

```

### A Shortcut for Propagating Errors: the ? Operator

``` rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username)
}

// chaining
fn read_username_from_file() -> Result<String, io::Error> {
  let mut username = String::new();

  File::open("hello.txt")?.read_to_string(&mut username)?;

  Ok(username)
}

// shorter
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

- So far, all the main functions we’ve used return (). The main function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected.

- Luckily, main can also return a Result<(), E>.

``` rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
  let greeting_file = File::open("hello.txt")?;

  Ok(())
}
```