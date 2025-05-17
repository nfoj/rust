# Syntax and Semantics

- Comments
```rust
// line;
  
/*
	block;
*/
```

- print
```rust
// print text without line breacks
print!("Hello, Maria!");

// print text with line breaks;
println!("Hello, Carlos!");
```

- line break: \n
```rust
// line break: \n
println!("What is your name?\nMy name is Rodolfo!");

// : \n
println!("What is your name?\\nMy name is Rodolfo!");
```

- format!
```rust
format!("Hello");                 // => "Hello"

format!("Hello, {}!", "world");   // => "Hello, world!"

format!("The number is {}", 1);   // => "The number is 1"

format!("{:?}", (3, 4));          // => "(3, 4)"

format!("{value}", value = 4);    // => "4"

let people = "Rustaceans";
format!("Hello {people}!");       // => "Hello Rustaceans!"

format!("{} {}", 1, 2);           // => "1 2"

format!("{:04}", 42);             // => "0042" with leading zeros

format!("{:#?}", (100, 200));     // => "(
                                  //       100,
                                  //       200,
                                  //     )"
```

> [!IMPORTANT]
For more information access: https://doc.rust-lang.org/std/fmt/
>


- variables
```rust
let name = "Alice";
println!("What is your name: {}", name);

let num = 24;
println!("What number did you choose? {}", num);
  
let letter = 'a';
println!("What is the first letter that comes to your mind? {}", letter);
  
let ok = true;
println!("One plus one equals two? {}", ok);
```

- mutability
```rust
let mut name = "Alice";
println!("What is your name: {}", name);
     
name = "Carlos";
println!("What is your name: {}", name);

     
let mut num = 24;
println!("Which number did you choose? {}", num);
     
num = 12;
println!("Which number did you choose? {}", num);
     

let mut letter = 'a';
println!("What is the first letter that comes to your mind? {}", letter);
     
letter = 'b';
println!("What is the first letter that comes to your mind? {}", letter);
     

let mut ok = true;
println!("Is 1 + 1 equal to two? {}", ok);

ok = false;
println!("Is 1 + 1 equal to two? {}", ok);
  
```

- constant 
```rust
const POINTS: i32 = 3;
println!("{}", POINTS);
```

- shadowing
```rust
let food = "bread";
println!("{}", food);

let food = "milk";
println!("{}", food);

let food = "pizza";
println!("{}", food);  
```

- scope
```rust
let x = 1;
println!("{}", x);

{
  let x = 2;
  println!("{}", x);
}

println!("{}", x);
```

> [!TIP]
Use the constant associated with the type: 'std::<type>::MIN or std::<type><::MAX'
println!("u8 ({} - {})", std::u8::MIN, std::u8::MAX);
println!("u16 ({} - {})", std::u16::MIN, std::u16::MAX);
>

- u
```rust
// u8 = 0 - 255
let number_u8: u8 = 255;
println!("{}", number_u8);

// u16 = 0 - 65.535
let number_u16: u16 = 255;
println!("{}", number_u16);

// u32 = 0 - 4.294.967.295
let number_u32: u32 = 255;
println!("{}", number_u32);
 
// u64 = 0 - 18.446.744.073.709.551.615
let number_u64: u64 = 255;
println!("{}", number_u64);
  
// u128 = 0 - ... 
let number_u128: u128 = 255;
println!("{}", number_u128);
```

- i 
```rust
// i8 = -128 - 127
let number_i8: i8 =  127;
println!("{}", number_i8);

// i16 = -32.768 - 32.767 (-32_768 até 32_767)
let number_i16: i16 =  127;
println!("{}", number_i16);

// i32 = -2.147.483.648 - 2.147.483.647
let number_i32: i32 =  127;
println!("{}", number_i32);

// i64 = -9.223.372.036.854.775.808 - 9.223.372.036.854.775.807
let number_i64: i64 =  127;
println!("{}", number_i64);

// i128 =  ... and ...
let number_i128: i128 =  127;
println!("{}", number_i128);  
```

- f (32 and 64)
```rust
// f32 = approx. -3.4e+38 a +3.4e+38
let number_f32: f32 =  179.76;
println!("{}", number_f32);

// f64 = approx. -1.8e+308 a +1.8e+308
let number_f64: f64 =  179.76;
println!("{}", number_f64);
```

- char
```rust
let character: char = 'a';
println!("{}", character);

let symbol: char = ' '
println!("{}", symbol);
```

- bool
```rust
let checked: bool = true;
println!("The data was checked? {}", checked);
 
let checked: bool = false;
println!("The data was checked? {}", checked);
```
> [!IMPORTANT] 
> Rust offers additional data types: usize and isize. Refer to the documentation for details.
> These types automatically adjust to the system's architecture (32-bit or 64-bit).
>

- tuples

```rust
  let data_types: (u8, char, f32, i64) = (2, 'a', 5.4, 28);
  println!("{:?}", data_types);
  
  let person = ("Gregor", 64, 1.82);
  let (x, y, z) = person;
  println!("My name is {x}, i'm {y} years old and my height is {z}");
```

- array

```rust
  let list: [u8; 3] = [1, 5, 9];
  println!("{:?}", list);
```

- Arithmetic Operators
    - sum = +
    - subtraction = -
    - multiplication = *
    - division = /
    - remainder = %

```rust
5 + 4 == 9
5 - 2 == 3
5 * 2 == 10
5.0 / 2.0 == 2.5
5 % 2 == 1
```

- Precede ordination
    - 1 - ()
    - 2 - *, / and %
    - 3 - + and -

```rust
println!("{}", 5 + 3 * 2); // 11
println!("{}", (5 + 3) * 2); // 16
```
