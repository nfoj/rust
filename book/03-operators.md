# Operators - arithmetic, precedence, comparison, logical and assignment

- arithmetic 
```rust

// sum: +
let sum = 1 + 1;
println!("{:?}", sum);

// subtraction: - 
let sub = 2 - 2;
println!("{:?}", sub);

// multiplication: *
let mul = 3 * 3;
println!("{:?}", mul);

// division: /
let div = 4 / 4;
println!("{:?}", div);

// remainder: %
let rem = 5 % 5;
println!("{:?}", rem);  
```

- arithmetic (extra)
```rust
// print
println!("{:?}", 10 + 10);

// let
let calc: u8 = 2 + 5;
println!("{:?}", calc);

// let + let
let num01: u8 = 4;
let num02: u8 = 5;
println!("{:?}", num01 * num02);

// let ++
let num_01: u16 = 9;
let num_02: u16 = 9;

let result: u16 = num_01 / num_02;
println!("{:?}", result);  

// &str
let num_01: &str = "1";
let num_02: &str = "2";
println!("{:?}", num_01.to_owned() + num_02);

// string
let num_03: String = String::from("1");
let num_04: String = String::from("2");
println!("{:?}", num_03 + &num_04);

// String + &str
let part_01: &str = "10";
let part_02: String = String::from("20");
println!("{}", part_01.to_owned() + &part_02);

// String + &str + let
let part_01: String = String::from("Hello, ");
let part_02: &str = "World";

let result: String =  part_01 + part_02;
println!("{}", result);

// tuples


```
