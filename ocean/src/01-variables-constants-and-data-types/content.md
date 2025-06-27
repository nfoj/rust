# Variables, Constants, and Data Types

### Comments

Comments are used to document your code, making it more understandable for you and other developers. They are ignored by the compiler, meaning they don't affect how your program runs, only its readability.
- **Clarity:** use them to explain something complex or important.
- **Maintenance:** make it easier to work with your code in the future.
- **Collaboration:** create comments that are understandable to anyone.
- **Debugging:** temporarily disable a line or block of code for testing.

```rust
// This is a single-line comment.

/*
  ...
  This is a multi-line comment.
  ...
*/
```

>💡
>- **Avoid excessively long comments:** keep them concise;
>- **Review and update comments:** ensure they remain accurate as your code evolves;
>- **Focus on the "why," not the "what":** explain the reasoning behind your code, not just what it does
>- **Don't comment self-explanatory lines of code:** if the code is clear on its own, a comment isn't >necessary.

### Print

The print! macro and its variations are used for debugging, informing the user, and generating formatted output.

```rust
// Prints text without a line break
print!("Hello,");
print!("world!");
print!("...");

//
println!("\n");

// Prints text with a line break
println!("Hello,");
println!("world!");
println!("...");
```

### Line break

Line breaks are used when you want to improve text formatting.
- **println!:** used to print something and then move to a new line.
- **\n:** used to insert a new line character anywhere within the text.

```rust
// line break: \n
println!("What is your name?\nMy name is Rodolfo!");

// 
println!("");

// It allows the visualization of (\n): \\n
println!("What is your name?\\nMy name is Rodolfo!");
```

### Format

The format! macro in Rust is used to improve data output formatting by using curly braces "{}" as placeholders that will be filled with the information.

```rust
//
let name = "Marcos";
println!("I'm, {}!", name);

//
let name = "Alice";
let years = 30;
println!("Hi, I'm {}, {} years old", name, years);
```

> ⚠️
>
>**For more information, see the [Rust `std::fmt` documentation](https://doc.rust-lang.org/std/fmt/).**

### Variables

Variables are named spaces in memory that can hold a value.

```rust
let name = "Roberto";
println!("What is your name: {}", name);

let num = 24;
println!("What number did you choose? {}", num);
```

### Mutability

Once a variable's value is set, you cannot change it later. However, using "mut" after "let" makes the created variable mutable.

```rust
//
let mut name = "Alice";
println!("What is your name: {}", name);

name = "Roberto";
println!("What is your name: {}\n", name);

//
let mut num = 24;
println!("Which number did you choose? {}", num);

num = 12;
println!("Which number did you choose? {}", num);
```

### Constant

The constant is a variable with a fixed, immutable value, explicit type, and can be declared in any scope.

```rust
const POINTS: i32 = 3;
println!("{}", POINTS);
```

### Shadowing

You can declare a new variable with the same name as an existing one.

```rust
let food = "bread";
println!("{}", food);

let food = "milk";
println!("{}", food);

let food = "pizza";
println!("{}", food);
```

### Scope

Enclosing a block of code within curly braces {}, limits its functionality to that specific scope, while still allowing access to previously defined data.

```rust
let x = 1;
println!("{}", x);

{
  let x = 2;
  println!("{}", x);
}

println!("{}", x);
```

>💡
>
> Use the constant associated with the type:
> ```rust
> println!("u8: Min = {} | Max = {}", std::u8::MIN, std::u8::MAX);
> println!("Usize: Min = {} | Max = {}", std::usize::MIN, std::usize::MAX);
> ```

### u: unsigned integers

Whole numbers that can only be positive.

```rust
// u8 = 0 - 255
let number_u8: u8 = 255;
println!("{}", number_u8);
```

### i: signed integers

Whole numbers that can be either positive or negative.

```rust
// i8 = -128 - 127
let number_i8: i8 =  127;
println!("{}", number_i8);
```

### f: floating-point numbers

Numbers that have a decimal point.

```rust
// f32 = approx. -3.4e+38 a +3.4e+38
let number_f32: f32 =  179.76;
println!("{}", number_f32);
```

### char: character

Used to represent a single character and requires the use of single quotes (e.g., 'a').

```rust
let character: char = 'a';
println!("{}", character);
```

### bool: boolean

Represents only two types of values: true or false.

```rust
let checked: bool = true;
println!("The data was checked? {}", checked);
```
> ⚠️
> 
> Rust offers additional data types: usize and isize. Refer to the documentation for details.
> These types automatically adjust to the system's architecture (32-bit or 64-bit).


### Tuples

Store multiple data items of different types in a fixed-size structure.

```rust
  let data_types: (u8, char, f32, i64) = (2, 'a', 5.4, 28);
  println!("{:#?}", data_types);
```

### Array

A list of data items that are all of the same type and have a fixed size.

```rust
  let list: [u8; 3] = [1, 5, 9];
  println!("{:#?}", list);
```

### Arithmetic Operators

Used for basic mathematical calculations.

- sum = +
- subtraction = -
- multiplication = *
- division = /
- remainder = %

```rust
fn main () {
    //
    println!("Sum: {}", 5 + 4);
    println!("Sub: {}", 5 - 2);
    println!("Mul: {}", 5 * 2);
    println!("Div: {}", 5.0 / 2.0);
    println!("Rem: {}", 5 % 2);
}
```

### Precede ordination

- ( )
- *, / and %
- \+ and -

```rust
println!("Result: {}", 5 + 3 * 2);
println!("Result: {}", (5 + 3) * 2);
```
