# Operators - arithmetic, assignment, precedence, comparison and logical

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
let num_01: u8 = 4;
let num_02: u8 = 5;
println!("{:?}", num_01 * num_02);

// let + let + let
let num_01: u16 = 9;
let num_02: u16 = 9;

let result: u16 = num_01 / num_02;
println!("{:?}", result);  
```

- assignment
```rust
//
let mut num: u8 = 10;

// addition and assignment: +=
num += 1;
println!("{}", num);

// subtraction and assignment: -=
num -= 1;
println!("{}", num);

// multiplication and assignment: *=
num *= 2;
println!("{}", num);

// division and assignment: /=
num /= 3;
println!("{}", num);

// remainder and assignment: %=
num %= 4;
println!("{}", num);  
```
- string and &str
```rust
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
```

- tuples
```rust
// tuple
let tup: (u8, u8) = (1, 4);
println!("{:?}", tup.0 + tup.1);

// tuple let
let tup: (u8, u8);
tup = (3, 8);
println!("{:?}", tup.0 + tup.1);

// tuple + tuple
let tup: ((u8, u8),(u8, u8)) = ((2, 4),(6, 16));
println!("{:?}", tup.0.0 + tup.0.1);
println!("{:?}", tup.1.0 + tup.1.1);
println!("{:?}", tup.0.0 + tup.1.0);
println!("{:?}", tup.0.1 + tup.1.1);

// tuple + let + let
let tup_01: (u8, u8) = (1, 4);
let tup_02: (u8, u8) = (14, 9);

let result = tup_01.0 + tup_01.1 + tup_02.0 + tup_02.1;
println!("{:?}", result);
```

- array
```rust
// array
let arr: [u8; 2] = [1, 4];
println!("{:?}", arr[0] + arr[1]);

// array let
let arr: [u8; 2];
arr = [3, 8];
println!("{:?}", arr[0] + arr[1]);

// array + array
let arr: [[u8; 2]; 2] = [[2, 4], [6, 16]];
println!("{:?}", arr[0][0] + arr[0][1]);
println!("{:?}", arr[1][0] + arr[1][1]);
println!("{:?}", arr[0][0] + arr[1][0]);
println!("{:?}", arr[0][1] + arr[1][1]);

// array + let + let
let arr_01: [u8; 2] = [1, 4];
let arr_02: [u8; 2] = [14, 9];

let result = arr_01[0] + arr_01[1] + arr_02[0] + arr_02[1];
println!("{:?}", result);
```

- scope
```rust
// scope
let scop_00: i8 = 2;
let scop_01: i8 = 2;

{
    let result: i8 = scop_00 + scop_01;
    println!("{:?}", result)
}

println!("{:?}", scop_00 + scop_01);
// println!("{:?}", result);  
```

- precedence
```rust
// () > *, /, and % > + and - 
let preced: [i8; 2] = [5, 3];
//
println!("{:?}", preced[0] + preced[1] * preced[1]);
// ()
println!("{:?}", (preced[0] + preced[1]) * preced[1]);
// *, / and % 
println!("{:?}", preced[0] * preced[1] / preced[1] % preced[1]);
// + and - 
println!("{:?}", preced[0] + preced[1] - preced[0] + preced[0] * preced[0]);
// 
println!("{:?}", ((preced[0] + preced[1]) - (preced[0] + preced[0]) * preced[0]));  
```

- comparison
```rust
// ==
let a = 1; 
let b = 1;
println!("{}", a == b);
// 
let a = 2; 
let b = 3;
println!("{}", a == b);

// !=
let a = 1; 
let b = 1;
println!("{}", a != b);
// 
let a = 2; 
let b = 3;
println!("{}", a != b);

// >
let a = 1; 
let b = 1;
println!("{}", a > b);

// <
let a = 2; 
let b = 3;
println!("{}", a < b);

// >=
let a = 1; 
let b = 1;
println!("{}", a >= b);

// <=
let a = 2; 
let b = 3;
println!("{}", a <= b);
```

- logical
```rust
//
let t: bool = true;
let f: bool = false;

// &&
println!("{}", t && t); // true AND true = true
println!("{}", f && f); // false AND false = false
println!("{}", t && f); // true AND false = false
println!("{}", f && t); // false AND true = false
    
// ||
println!("{}", t || t); // true OR true = true
println!("{}", f || f); // false OR false = false
println!("{}", t || f); // true OR false = true
println!("{}", f || t); // false OR true = true
   
// !
println!("{}", !t); // true = false
println!("{}", !f); // false = true
```
