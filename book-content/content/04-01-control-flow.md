# Control Flow - match

```rust
//
let a = 1;

match a {
  1 => println!("1"),
  2 => println!("2"),
  3 => println!("3"),
  _ => println!("else"),
};

// range
match age {
  0..=11 => "child",
  12..=17 => "teenager",
  18..=64 => "adult",
  _=> "elderly"
};

//
match alph {
  'A'..='Z' => true,
  'a'..='z' => false,
};

// operation
match a {
  1 | 3 | 5 | 7 | 9 => "odd",
  2 | 4 | 6 | 8 | 10 => "even",
  _=> "else",
};

// match guards
let even = (2, -2);
let result = match even {
  (x, y) if x == y => "equal",
  (x, _) if x % 2 == 0 => "first even",
  _=> "none",
}

// 

```
