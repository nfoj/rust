// Questions - Variables Constants and Data Types

// comments
// How would you write a single-line comment in rust?
// single-line comment

// What is the syntax for a block comments in rust?
/* block comments */

// print
// Which macro would you use to print text on the same line, whihout adding a newline at the end?  Give an example.
// print();
fn main() {
    //
    print("Print: whihout adding a newline at the end!");
}

// Which macro would you use to print text and automatically add a newline at the end? Give an example.
// println!();
fn main() {
    //
    println!("Print: add newline at the end!");
}

// line-break
// How do you insert an explicit newline within a string that is being printed with println!?
fn main() {
    //
    println!("First\nSecond!");
}

// What would be printed by the following code?
// println!("First line\\nSecond line");
// First line\nSecond line

// format!
// What will be the output of the following code?
/*
    let name = "Ana";
    let age = 30;
    let text = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", text);
*/
// Hello, Ana! You are 30 years old.

// How would you use the format! macro to create the string "The value is: 42" from the number 42?
fn main() {
    //
    let number = 42;
    let formated = format!("The value is: {}", number);
    println!("{}", formated);
}

// How can you format the number 7 so that it is displayer as "0007" using format!?
fn main() {
    //
    let number: u8 = 7;
    println!("{:04}", number);

    let formated = format!("{:04}", number);
    println!("{}", formated);
}

// What is the utily of "{:?}" in the format! macro? Give an example of when you would use it.
// The "{:?}" specifier in the format! macro is used for debug formatting. It's particularly useful for printing data structures like structs, enums, tuples, and vectors in a way that is easy for developers to inspect. It typically shows the structure of the data along with its values.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //
    let p = Point { x: 10, y: 20 };
    println!("The point is: {:?}", p);
}

// What does the specifier "{:#?}" do differently from "{:?}" when formatting a tuple or struct?
// The specifier "{:#?}" does the same as "{:?}" but with pretty-printing. When formatting tuples or structs, "{:#?}" will often add newlines and indentation to make the output more readable, especially for complex data structures.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Debug format: {:?}", rect);
    println!(" ");
    println!("Debug format: {:#?}", rect);
}

// variables
// How do you declare a variable named score and assign in the value 100?
fn main() {
    //
    let score: u8 = 100;
    println!("Value: {}", score);
}

// Write a line of code that declares a variable city with the value "Paris" and then prints it to the screen.
fn main() {
    //
    let city: &str = "Paris";
    println!("City: {}!", city); // output = City: Paris
    println!("City: {:?}!", city); // output = City: "Paris"
}

// mutability
// Which keyword is used to declare a variable that can have its value changed after initialization?
// mut

// Declare a mutable variable named counter initialized with 0. Then, write the code to change the value of counter to 5 and print it.
fn main() {
    //
    let mut counter: u8 = 0;
    println!("Counter = {:?}", counter);

    counter = 5;
    println!("Counter = {:?}", counter);
}

// constants
// How do you declare a constant named MAX_SPEED with the value 9000 of type i32?
fn main() {
    //
    const MAX_SPEED: i32 = 9000;
    println!("Max Speed: {:?}", MAX_SPEED);
}

// Is it possible to change the value of a constant after its declaration?
// no

// shadowing
// What is "shadowing" in Rust? Explain with a code example and what the output would be.
// Shadowing in Rust is the ability to declare a new variable with the same name as a previous variable within the same scope.
fn main() {
    //
    let x: i8 = 0;
    {
        let x: f32 = 1.0;
        println!("{:?}", x);
    }
    println!("{:?}", x);
}

// What will be the output of the following code?
/* Rust

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The inner value of x is: {}", x);
    }
    println!("The outer value of x is: {}", x);
*/
// 12
// 6

// scope
// What will be the output of the following code? Explain why
/*
fn main () {
    //
    let a = 10; {
        let b = 20;
        println!("Inside the block: a = {}, b = {}", a, b);
    }
    // println!("Outside the block: a = {}, b = {}", a, b);
    println!("Outside the block: a = {}", a);
}
*/

// Inside the block: a = 10, b = 20
// Outside the block: a = 10

// What would happen if the commented line were uncommented and compiled?
// fail to compile and produce a compile-time error.

// int
// What is the range of values that a u8 type can store?
// u8 - Max = 255 and Min = 0

// What is the range of values that an i8 type can store?
// i8 - Max = 127 and Min = -128

// How do you declare a variable maximum_age of type u16 with the value 150?
fn main() {
    //
    let maximum_age: u16 = 150;
    println!("maximum_age: {}", maximum_age);
}

// How can you print the minimum and maximum values for the u32 type using its associated constants?
fn main() {
    //
    const MAXIMUM: u32 = u32::MAX;
    println!("Max = {}", MAXIMUM);

    const MINIMUM: u32 = u32::MIN;
    println!("MIN = {}", MINIMUM);
}

// float
// Declare a variable named 'price' of type f32 with the value 19.99
fn main() {
    //
    let price: f32 = 19.99;
    println!("Price = {}", price);
}

// Which of the types f32 or f64 offers greater precision?
// f32 (single-precision floating-point): Uses 32 bits to represent a floating-point number. This allows for a certain level of precision, but it's less precise than f64.
// f64 (double-precision floating-point): Uses 64 bits to represent a floating-point number. The extra bits allow for a significantly larger range of representable numbers and, more importantly for this question, a much higher degree of precision in those representations.

// char
// How do you declare a variable named 'initial' that stores the character 'P'?
fn main() {
    //
    let initial = 'P';
    println!("The initial is: {}", initial);
}

// Can a char type in Rust store only ASCII characters? Explain.
fn main() {
    //
    let cyrillic_char = 'Я';
    println!("Cyrillic character: {}", cyrillic_char);
}

// bool
// Declare a variable named 'rain' and assign the value true to it.
fn main() {
    //
    let rain: bool = true;
    println!("Rain? {}", rain);
}

// What are the only two possible values for a variable of type bool?
// true and false

// tuples
// How do you declare a tuple named 'product' that stores an item's name (string), its quantity (integer), and its price (floating-point)? For example: ("Pen", 10, 1.50).
fn main() {
    //
    let product = ("Pen", 10, 1.50);
    println!("Product = {:?}", product);
}

// Given the tuple 'let coordinates = (10, 20, 30);', how would you destructure this tuple into three separate variables x, y, and z?
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
// How would you access the second element of the tuple 'let rgb = (255, 0, 128):'?
fn main() {
    //
    let product: (&str, &str, &str) = ("Item 1", "Item 2", "Item 3");
    println!("Product: {}", product.1);

    //
    let (item_01, item_02, item_03) = product;
    println!("Product: {}", item_02);
}

// arrays
// Declare an array named 'notes' that can store 5 numbers of type f32. Initialize it with some values.
fn main() {
    //sdfsdiiuuuu
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

// What is the main difference between an array and a tuple in terms of the data types they can store?
/*
Arrays ([T; N])

    Homogeneous: All elements must have the identical type T.
    Fixed Size: The number of elements N is known at compile time and cannot change.
    Example:
    Rust

let numbers: [i32; 3] = [1, 2, 3]; // All are i32
let words: [&str; 2] = ["hello", "world"]; // All are &str

Tuples ((T1, T2, ..., Tn))

    Heterogeneous: Each element can have a different type (T1, T2, etc.).
    Fixed Size: The number of elements is known at compile time and cannot change.
    Example:
    Rust

    let person_data: (&str, i32, f64) = ("Alice", 30, 60.5); // &str, i32, f64
    let coordinates: (i32, i32) = (10, 20); // i32, i32 (though they are the same type here, they *could* be different)
*/

// How would you access the first element of the array `let cores = ["red", "green", "blue"];`?
fn main() {
    //
    let list: [&str; 3] = ["red", "green", "blue"];
    println!("List: {:?}", list);

    //
    let list: [&str; 3] = ["red", "green", "blue"];
    println!("List: {:?}", list[0]);
}

// arithmetic operators
// What is the result of the expression 10 % 3?
// 1
fn main() {
    //
    println!("{}", 10 % 3);
}

// What is the result of the expression 5.0 / 2.0?
// 2.5
fn main() {
    //
    println!("{}", 5.0 / 2.0);
}

// What will be the result of the following expression in Rust, and why? println!("{}", 20 - 5 * 2);
// 10
fn main() {
    //
    println!("{}", 20 - 5 * 2);
}

// What will be the result of the following expression in Rust, and why? println!("{}", (20 - 5) * 2);
// 30
fn main() {
    //
    println!("{}", (20 - 5) * 2);
}
