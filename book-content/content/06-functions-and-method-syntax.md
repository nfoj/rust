# Functions and Method Syntax

## Function

```rust
fn text() {
    println!("Hello, world!");
}

fn main() {
    text();
}
```

## Parameters

`paramenter`

```rust
fn number (x: i8) {
    println!("Number = {}", x);
}

fn main () {
    number (24);
}
```

`multiple parameters`

```rust
fn multiple_parameters (x: i8, y: i8) {
    println!("{} + {} = {}", x, y, x + y);
}

fn main () {
    multiple_parameters (10, 20);
}
```

## Return values

```rust
fn sum(x: i8, y: i8) -> i8 {
    x + y
}

fn main() {
    let result = sum(15, 25);
    println!("Result: {}", result);
}
```

## Unsed code

```rust
#[allow(dead_code)]
fn unused_function() {
    println!("This function won't be called");
}

fn main() {
    println!("...");
}
```

## Modules

```rust
mod calculator {
    pub fn sum(a: i8, b: i8) -> i8 {
        a + b
    }
    
    pub fn subtract(a: i8, b: i8) -> i8 {
        a - b
    }
}

mod text {
    pub fn text() {
        println!("Hello, world!");
    }
}

fn main() {
    let sum = calculator::sum(10, 5);
    println!("Sum: {}", sum);
    
    let difference = calculator::subtract(10, 5);
    println!("Difference: {}", difference);
    
    text::text();
}
```

## Modules inside modules

```rust
mod utilities {

    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
    
    pub mod text {
        pub fn greet(name: &str) -> String {
            format!("Hello, {}!", name)
        }
        
        pub fn farewell(name: &str) -> String {
            format!("Goodbye, {}!", name)
        }
    }
}

fn main() {
    let sum = utilities::math::add(5, 3);
    println!("Sum: {}", sum);
    
    let greeting = utilities::text::greet("Alice");
    println!("{}", greeting);
}
```

## File-based modules

```rust
// Project structure:
/*
project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── sum.rs
    ├── subtract.rs
    ├── multiply.rs
    └── divide.rs
*/

// sum.rs
pub fn calculate(a: i8, b: i8) -> i8 {
    a + b
}

// subtract.rs  
pub fn calculate(a: i8, b: i8) -> i8 {
    a - b
}

// multiply.rs
pub fn calculate(a: i8, b: i8) -> i8 {
    a * b
}

// divide.rs
pub fn calculate(a: i8, b: i8) -> i8 {
    a / b
}

// main.rs
mod add;
mod subtract;
mod multiply;
mod divide;

fn main() {
    let sum = sum::calculate(10, 5);
    println!("10 + 5 = {}", sum);
    
    let diff = subtract::calculate(10, 5);
    println!("10 - 5 = {}", diff);
    
    let product = multiply::calculate(10, 5);
    println!("10 * 5 = {}", product);
    
    let quotient = divide::calculate(10, 5);
    println!("10 / 5 = {}", quotient);
}
```

## File-based modules with submodules

```rust
// Project structure:
/*
project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── math.rs
    └── text.rs
*/

// math.rs
pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}

pub fn subtract(a: i8, b: i8) -> i8 {
    a - b
}

pub fn multiply(a: i8, b: i8) -> i8 {
    a * b
}

pub fn divide(a: i8, b: i8) -> i8 {
    a / b
}

// text.rs
pub fn hello() {
    println!("Hello!");
}

pub fn thanks() {
    println!("Thank you!");
}

// main.rs
mod math;
mod text;

fn main() {

    text::hello();
    println!("Addition: 8 + 4 = {}", math::sum(8, 4));
    println!("Subtraction: 8 - 4 = {}", math::subtract(8, 4));
    println!("Multiplication: 8 * 4 = {}", math::multiply(8, 4));
    println!("Division: 8 / 4 = {}", math::divide(8, 4));    
    text::thanks();
}
```

## Modules with internal module structure

```rust
// Project structure:
/*
  project/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── calculator.rs
*/

// calculator.rs
pub mod basic_ops {
    pub fn add(a: i16, b: i16) -> i16 {
        a + b
    }
    
    pub fn subtract(a: i16, b: i16) -> i16 {
        a - b
    }
}

pub mod advanced_ops {
    pub fn power(base: i16, exp: u16) -> i16 {
        base.pow(exp)
    }
    
    pub fn square_root(n: f32) -> f32 {
        n.sqrt()
    }
}

// Functions directly in the calculator module
pub fn display_result(operation: &str, result: i16) {
    println!("{} = {}", operation, result);
}

// main.rs
mod calculator;

fn main() {
    // calculator.rs
    let sum = calculator::basic_ops::add(10, 5);
    calculator::display_result("10 + 5", sum);
    
    let power = calculator::advanced_ops::power(2, 3);
    calculator::display_result("2^3", power);    
}
```


```rust

```


```rust

```


```rust

```


```rust

```


```rust

```
