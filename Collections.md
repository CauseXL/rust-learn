
- A vector allows you to store a variable number of values **next to each other**.
- A string is a collection of characters.
- A hash map allows you to associate a value with a particular key.

## Vec

``` rust
let v: Vec<i32> = Vec::new();

// or
let v = vec![1, 2, 3];
// Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary
```

### Updating a Vector

``` rust
let mut v = Vec::new();
v.push(1);
v.push(1);
v.push(1);
```

### Reading Elements of Vectors

- use index or get
- index larger than the number of elements will throw an error
- however get will return None in match

``` rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The  element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The  element is {third}"),
    None => println!("There is no third element."),
}
```

### Borrowing

- can’t have mutable and immutable references in the same scope. 

``` rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // - immutable borrow occurs here

v.push(6); // mutable borrow occurs here

println!("The first element is: {first}"); // immutable borrow later used here

// error
```

- why should a reference to the first element care about changes at the end of the vector?
  - because vectors put the values next to each other in memory
  - adding a new element onto the end of the vector might require **allocating new memory and copying the old elements** to the new space
  - if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 
  - In that case, the reference to the first element would be **pointing to deallocated memory**. 
  - The borrowing rules prevent programs from ending up in that situation.

### Iterating over the Values in a Vector

``` rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

### Using an Enum to Store Multiple Types

``` rust
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),
];
```

## String

- String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities

- be sure to remember that **valid Unicode scalar values** may be made up of **more than 1 byte**.

``` rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

### Updating a String

``` rust
let mut s = String::from("foo");
s.push_str("bar");
s.push("l"); // The push method takes a single character as a parameter
```

- the format! macro uses references so that this call doesn’t take ownership of any of its parameters.

``` rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

### Indexing into Strings

- Rust strings don’t support indexing.

### Slicing Strings

``` rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

### Methods for Iterating Over Strings

``` rust
for c in "Зд".chars() {
  println!("{}", c);
}
```


## Hash Map

- hash maps store their data on the heap.
- keys must have the same type as each other, and same as values.

``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### Accessing Values in a Hash Map

``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

- The get method returns an ```Option<&V>```
- This program handles the Option by calling copied to get an Option<**i32**> rather than an Option<**&i32**>
- then ```unwrap_or``` to set score to zero if scores doesn't have an entry for the key.


``` rust
for (k, v)in scores {
  println!("=={k}: {v}==");
}
```

### Ownership

- For types that implement the **Copy trait**, like i32, the **values are copied into** the hash map. 
- For **owned values** like String, the values will be moved and the **hash map will be the owner** of those values


``` rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
```

### Updating a Hash Map

### Overwriting a Value

``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

### Adding a Key and Value Only If a Key Isn’t Present

``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

// {"Yellow": 50, "Blue": 10}
println!("{:?}", scores);
```

### Updating a Value Based on the Old Value

``` rust
let text = "hello world wonderful world";

let mut map = HashMap::new();
for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}
// {"world": 2, "hello": 1, "wonderful": 1}
println!("{:?}", map);
```






