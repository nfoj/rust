# Answers - Variables Constants and Data Types

---
01 - How would you write a single-line comment in rust?

```rust
fn main () {
    // single-line comment
}
```
  
02 - What is the syntax for a block comments in rust?

```rust
fn main () {
    /* block comments */
}
```

03 - Which macro would you use to print text on the same line, whihout adding a newline at the end?

```rust
fn main() {
    //
    print!("Print: whihout adding a newline at the end!");
}
```

04 - Which macro would you use to print text and automatically add a newline at the end?

```rust
fn main() {
    //
    println!("Print: add newline at the end!");
}
```

05 - How do you insert an explicit newline within a string that is being printed with println!?

```rust
fn main() {
    //
    println!("First\nSecond!");
}
```

06 - What would be printed by the following code?
```
println!("First line\\nSecond line");
```

```rust
fn main () {
    //
    println!("First line\\nSecond line");
}
```

07 - What will be the output of the following code?
```
let name = "Ana";
let age = 30;
let text = format!("Hello, {}! You are {} years old.", name, age);
println!("{}", text);
```

```rust
fn main () {
    //
    let name = "Ana";
    let age = 30;
    let text = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", text);
}
```

08 - How would you use the format! macro to create the string "The value is: 42" from the number 42?

```rust
fn main() {
    //
    let number = 42;
    let formated = format!("The value is: {}", number);
    println!("{}", formated);
}
```

09 - How can you format the number 7 so that it is displayer as "0007" using format!?

```rust
fn main() {
    //
    let number: u8 = 7;
    println!("{:04}", number);

    //
    let formated = format!("{:04}", number);
    println!("{}", formated);
} 
```

10 - What is the utily of "{:?}" in the format! macro? Give an example of when you would use it.

The "{:?}" specifier in the format! macro is used for debug formatting. It's particularly useful for printing data structures like structs, enums, tuples,
and vectors in a way that is easy for developers to inspect. It typically shows the structure of the data along with its values.

```rust
fn main () {
    //
    let number: u8 = 1;
    println!("{:?}", number);
}
```

11 - What does the specifier "{:#?}" do differently from "{:?}" when formatting a tuple or struct?

```rust
fn main () {
    //
    let arr = (1, 2, 3, 4);

    //
    println!("{:#?}\n", arr);
    println!("{:?}", arr);
}  
```

12 - How do you declare a variable named score and assign in the value 100?

```rust
fn main() {
    //
    let score: u8 = 100;
    println!("Value: {}", score);
} 
```

13 - Write a line of code that declares a variable city with the value "Paris" and then prints it to the screen.

```rust
fn main() {
    //
    let city: &str = "Paris";
    println!("City: {}!", city);
    println!("City: {:?}!", city);
} 
```

15 - Which keyword is used to declare a variable that can have its value changed after initialization?

```rust
fn main () {
    // mut
}
```

16 - Declare a mutable variable named counter initialized with 0. Then, write the code to change the value of counter to 5 and print it.

```rust
fn main() {
    //
    let mut counter: u8 = 0;
    println!("Counter = {:?}", counter);

    counter = 5;
    println!("Counter = {:?}", counter);
}
```

17 - How do you declare a constant named MAX_SPEED with the value 9000 of type i32?

```rust
const MAX_SPEED: i32 = 9000;

fn main() {
    //
    println!("Max Speed: {:?}", MAX_SPEED);
}
```

18 - Is it possible to change the value of a constant after its declaration?

```rust
fn main () {
    // no
}  
```

19 - What is "shadowing" in Rust? Explain with a code example and what the output would be.

Shadowing in Rust is the ability to declare a new variable with the same name as a previous variable within the same scope.

```rust
fn main() {
    //
    let x: i8 = 0;
    {
        let x: f32 = 1.0;
        println!("{:?}", x);
    }
    println!("{:?}", x);
}
```

20 - What will be the output of the following code?
```
let x = 5;

let x = x + 1;
{
  let x = x * 2;
  println!("The inner value of x is: {}", x);
}

println!("The outer value of x is: {}", x);
```

```rust
fn main () {
  //
  let x = 5;
  let x = x + 1;
  {
      let x = x * 2;
      println!("The inner value of x is: {}", x);
  }
  println!("The outer value of x is: {}", x);
}  
```

21 - What will be the output of the following code? Explain why
```
let a = 10;
  {
    let b = 20;
    println!("Inside the block: a = {}, b = {}", a, b);
  }

// println!("Outside the block: a = {}, b = {}", a, b);
println!("Outside the block: a = {}", a);
```

```rust
fn main () {
    //
    let a = 10;
    {
        let b = 20;
        println!("Inside the block: a = {}, b = {}", a, b);
    }
    // println!("Outside the block: a = {}, b = {}", a, b);
    println!("Outside the block: a = {}", a);
}  
```

22 - What would happen if the commented line were uncommented and compiled?

```rust
fn main () {
    // fail to compile and produce a compile-time error.
}  
```

23 - What is the range of values that a u8 type can store?

```rust
fn main () {
    //
    println!("u8: Max = {} and Min = {}", std::u8::MAX, std::u8::MIN);
}
```

24 - What is the range of values that an i8 type can store?

```rust
fn main () {
    //
    println!("i8: Max = {} and Min = {}", std::i8::MAX, std::i8::MIN);
}  
```

25 - How do you declare a variable maximum_age of type u16 with the value 150?

```rust
fn main() {
    //
    let maximum_age: u16 = 150;
    println!("maximum_age: {}", maximum_age);
}  
```

26 - How can you print the minimum and maximum values for the u32 type using its associated constants?

```rust
const MAXIMUM: u32 = u32::MAX;
const MINIMUM: u32 = u32::MIN;

fn main() {
    //
    println!("Max = {}", MAXIMUM); 
    println!("MIN = {}", MINIMUM);
}
```

27 - Declare a variable named 'price' of type f32 with the value 19.99.

```rust
fn main() {
    //
    let price: f32 = 19.99;
    println!("Price = {}", price);
}
```

28 - Which of the types f32 or f64 offers greater precision?

- **f32 (single-precision floating-point):** Uses 32 bits to represent a floating-point number. This allows for a certain level of precision, but it's less precise than f64;
- **f64 (double-precision floating-point):** Uses 64 bits to represent a floating-point number. The extra bits allow for a significantly larger range of representable numbers and, more importantly for this question, a much higher degree of precision in those representations.  

29 - How do you declare a variable named 'initial' that stores the character 'P'?

```rust
fn main() {
    //
    let initial = 'P';
    println!("The initial is: {}", initial);
}
```

30 - Can a char type in Rust store only ASCII characters? Explain.

```rust
fn main() {
    //
    let cyrillic_char = 'Я';
    println!("Cyrillic character: {}", cyrillic_char);
}
```

31 - Declare a variable named 'rain' and assign the value true to it.

```rust
fn main() {
    //
    let rain: bool = true;
    println!("Rain? {}", rain);
}
```

32 - What are the only two possible values for a variable of type bool?

```rust
fn main () {
    // true and false
}
```

33 - How do you declare a tuple named 'product' that stores an item's name (string), its quantity (integer), and its price (floating-point)? For example: ("Pen", 10, 1.50).

```rust
fn main() {
    //
    let product = ("Pen", 10, 1.50);
    println!("Product = {:?}", product);
}
```


34 - Given the tuple 'let coordinates = (10, 20, 30);', how would you destructure this tuple into three separate variables x, y, and z?

```rust
fn main() {
    //
    let product: (&str, u8, f32) = ("Pen", 10, 1.50);
    println!("Product: {}, {} and {}", product.0, product.1, product.2);

    //
    let (name, quantity, price): (&str, u8, f32) = ("Pen", 10, 1.50);
    println!("Product: {}, {} and {}", name, quantity, price);

    //
    let (name, quantity, price) = product;
    println!("Product: {}, {} and {}", name, quantity, price);
}
```

35 - How would you access the second element of the tuple 'let rgb = (255, 0, 128):'?

```rust
fn main() {
    //
    let product: (&str, &str, &str) = ("Item 1", "Item 2", "Item 3");
    println!("Product: {}", product.1);

    //
    let (item_01, item_02, item_03) = product;
    println!("Product: {}", item_02);
}
```

36 - Declare an array named 'notes' that can store 5 numbers of type f32. Initialize it with some values.

```rust
fn main() {
    //
    let notes: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", notes);

    //
    let notes: [u8; 5];
    notes = [1, 2, 3, 4, 5];
    println!("{:?}", notes);

    //
    let notes_slice: &[u8] = &notes;
    println!("{:?}", notes_slice);

    //
    let partial_notes_slice: &[u8] = &notes[0..3];
    println!("{:?}", partial_notes_slice);
}
```

37 - What is the main difference between an array and a tuple in terms of the data types they can store?

- Tuples ((T1, T2, ..., Tn)):
    - Heterogeneous: Each element can have a different type (T1, T2, etc.);
    - Fixed Size: The number of elements is known at compile time and cannot change.

    Example:

    ```rust
    let person_data: (&str, i32, f64) = ("Alice", 30, 60.5);
    let coordinates: (i32, i32) = (10, 20);
    ```

- Arrays ([T; N]):
    - Homogeneous: All elements must have the identical type T;
    - Fixed Size: The number of elements N is known at compile time and cannot change.

    Example:

    ```rust
    let numbers: [i32; 3] = [1, 2, 3];
    let words: [&str; 2] = ["hello", "world"];
    ```

38 - How would you access the first element of the array: let cores = ["red", "green", "blue"];?

```rust
fn main() {
    //
    let list: [&str; 3] = ["red", "green", "blue"];
    println!("List: {:?}", list);

    //
    let list: [&str; 3] = ["red", "green", "blue"];
    println!("List: {:?}", list[0]);
}
```

39 - What is the result of the expression 10 % 3?

```rust
fn main() {
    //
    println!("{}", 10 % 3);
}
```

40 - What is the result of the expression 5.0 / 2.0?

```rust
fn main() {
    //
    println!("{}", 5.0 / 2.0);
}
```

41 - What will be the result of the following expression in Rust, and why? println!("{}", 20 - 5 * 2);

```rust
fn main() {
    //
    println!("{}", 20 - 5 * 2);
}    
```

42 - What will be the result of the following expression in Rust, and why? println!("{}", (20 - 5) * 2);

```rust
fn main() {
    //
    println!("{}", (20 - 5) * 2);
}
```
