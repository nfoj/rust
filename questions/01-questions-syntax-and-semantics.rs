// Questions - Syntax-and-semantics

// Create a comment in the code and insert the URL of the official website of the selected programming language.
// Rust URL: https://www.rust-lang.org/

// Represent the different existing syntaxes for creating comments in the language (in one line, several...).
// # foj.dev
/* #foj.dev */

// Create a variable (and a constant, if the language supports it).
let name = "Alice";
println!("What is your name: {}", name);

const NAME: &str = "Alice";
println!("What is your name: {}", NAME);

// Create variables that represent all the primitive data types of the language (text strings, enters, booleans...).
let type_u = 1;
let type_i = 1;
let type_f = 1.0;
let type_char = 'a';
let type_str = "abc";
let type_bool = true;
let type_tuples = ("Rodofol", 64, 1.87);
let type_aray = [1, 2, 3, 4, 5, 6, 7, 8, 9];                              

// Print the text via terminal: "Hello, [and the name of your language]!"
fn main () {
    println!("Hello, rust!");
}
