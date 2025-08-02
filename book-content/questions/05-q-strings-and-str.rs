// Write code that creates an empty String and prints it.
fn main() {
    //
    let empty_string: String = String::new();
    println!("{}", empty_string);
}

// Create a String from the string literal "Hello, Rust!".
fn main() {
    //
    let literal_string: String = String::from("Hello, Rust!");
    println!("{}", literal_string);
}

// Declare a variable that holds a string slice (&str) with the value "Learning Rust".
fn main() {
    //
    let hold_string: &str = "Learning Rust";
    println!("{}", hold_string);
}

// Given a String, write the code to get a &str that represents the entire String.
fn main() {
    //
    let input_string: String = String::from("Learning Rust!");
    let input_str: &str = &input_string;
    println!("{}", input_str);
}

// or

fn main() {
    //
    let input_string: String = String::from("Learning Rust!");
    let input_str: &str = input_string.as_str();
    println!("{}", input_str);
}

// Write a code that receives a text in "string" and converts it to "str" and then prints it on the screen.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_string: String = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Data entry error!");
    let text_string: &str = input_string.as_str();
    println!("\nText converted to &str: {}", text_string);
}

// Convert the integer 123 into a String.
use std::io;

fn main() {
    //
    println!("Enter number:");
    let mut input_number: String = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Data entry error!");

    //
    let number: u8 = input_number.trim().parse().expect("Data converting error!");
    println!("Converted value = {}", number);
}

// Use the .to_string() method or String::from() to create a new String by concatenating a &str and an existing String.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Data entry error!");

    //
    let text_01: &str = &input_text.as_str();
    let text_02: String = String::from("string");

    //
    println!("\n{}", text_01.to_string() + &text_02);
}

// Create a mutable String and append a &str to it using the push_str() method.
fn main() {
    //
    let mut mutable_string: String = String::from("Hello, ");
    mutable_string.push_str("word!");
    println!("{}", mutable_string);
}

// Add a single character (char) to the end of a mutable String.
fn main() {
    //
    let mut single_char: String = String::from("Hello, word");
    let character: char = '!';
    single_char.push(character);
    println!("{}", single_char);
}

// Combine a &str, a String, and a number into a new String using the format! macro.
fn main() {
    //
    let var_str: &str = "1";
    let var_string: String = String::from("2");
    let var_u8: u8 = 3;

    //
    let result = format!("{}{}{}", var_str, var_string, var_u8);
    println!("{}", result);
}

// Create a new String by removing all whitespace from a string.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    let remove_wspace = input_text.replace(" ", "");
    println!("\nFirst Letter = {}", remove_wspace.trim());
}

// Replace all occurrences of the substring "error" with "success" in a &str, creating a new String.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    let new_text = input_text.replace("error", "success");
    println!("\nFirst Letter = {}", new_text.trim());
}

// Replace only the first occurrence of "old" with "new" in a &str, creating a new String.
use std::io;

fn main() {
    //
    println!("Enter phrase:");
    let mut input_phrase: String = String::new();
    io::stdin().read_line(&mut input_phrase).err();

    //
    let phrase = input_phrase.replacen("old", "new", 1);
    println!("{}", phrase.trim());
}

// Check if a &str contains the substring "Rust".
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    if input_text.contains("Rust") {
        println!("\n'{}' contains 'Rust'", input_text.trim());
    } else {
        println!("\n'{}' does not contain 'Rust'", input_text.trim());
    }
}

// Check if a filename in a string ends with the suffix .rs.
use std::io;

fn main() {
    //
    println!("Enter filename:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    if input_text.trim().ends_with(".rs") {
        println!("\n'{}' ends with '.rs'", input_text.trim());
    } else {
        println!("\n'{}' does not ends with '.rs'", input_text.trim());
    }
}

// Check if a &str starts with the prefix "https://".
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    if input_text.starts_with("https://") {
        println!("\n'{}' start with 'https://'", input_text.trim());
    } else {
        println!("\n'{}' does not start with 'https://'", input_text.trim());
    }
}

// Create code that takes a String and returns a slice containing only the first word. Assume that words are separated by spaces.
use std::io;

fn main() {
    //
    println!("Enter phrase:");
    let mut input_phrase: String = String::new();
    io::stdin().read_line(&mut input_phrase).err();

    //
    let phrase = input_phrase.split_whitespace().next();
    println!("{}", phrase.unwrap());
}

// Find the starting byte index of the first occurrence of the substring "world".
use std::io;

fn main() {
    //
    println!("Enter filename:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    //
    if let Some(index) = input_text.find("world") {
        println!("\nByte index: world = {}", index);
    } else {
        println!("\nNo word: world");
    }
}

// Write a function that accepts a String and returns its first character as a char.
use std::io;

fn main() {
    //
    let mut input_word: String = String::new();
    io::stdin().read_line(&mut input_word).err();

    //
    let mut first_letter: char = ' ';
    for i in input_word.trim_start().chars() {
        first_letter = i;
        break;
    }
    println!("First Letter = {}", first_letter);
}

// or
use std::io;

fn main() {
    //
    let mut input_word: String = String::new();
    io::stdin().read_line(&mut input_word).err();

    //
    let first_letter: char = input_word.trim_start().chars().next().unwrap_or(' ');
    println!("First Letter = {}", first_letter);
}

// Write a code that counts the number of characters (char) in a &str, not the number of bytes.
use std::io;

fn main() {
    //
    println!("Enter word:");
    let mut input_word: String = String::new();
    io::stdin().read_line(&mut input_word).err();

    //
    let word = input_word.trim();
    let mut count: u8 = 0;

    //
    for _i in word.chars() {
        count += 1;
    }

    println!("{}", count);
}

// Write a code that counts the number of characters (char) in a sentence, excluding whitespace.
use std::io;

fn main() {
    //
    println!("Enter phrase:");
    let mut input_phrase: String = String::new();
    io::stdin().read_line(&mut input_phrase).err();

    //
    let phrase = input_phrase.trim().replace(" ", "");
    let mut count: u8 = 0;

    //
    for _i in phrase.chars() {
        count += 1;
    }

    println!("{}", count);
}

// Write a code that takes a &mut String and replaces all lowercase 'a' characters with uppercase 'A's, modifying the original String.
use std::io;

fn main() {
    // create new var
    println!("Enter phrase:");
    let mut input_phrase: String = String::new();
    io::stdin().read_line(&mut input_phrase).err();

    //
    let phrase = input_phrase.trim().replace("a", "A");
    println!("{}", phrase);

    // mut input_phrase
    println!("Enter phrase:");
    let mut input_phrase: String = String::new();
    io::stdin().read_line(&mut input_phrase).err();

    //
    let bytes = unsafe { input_phrase.as_bytes_mut() };

    for i in bytes.iter_mut() {
        if *i == b'a' {
            *i = b'A';
        }
    }
    println!("{}", phrase);
}

// Write code that removes the last character from a String.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_character: String = String::new();
    io::stdin()
        .read_line(&mut input_character)
        .expect("Data entry erro!");

    let mut character: String = input_character.trim_end().to_string();
    character.pop();
    println!("{}", character);
}

// or

use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_character: String = String::new();
    io::stdin()
        .read_line(&mut input_character)
        .expect("Data entry erro!");

    //
    let mut count = 0;
    let mut character: String = String::new();

    for i in input_character.trim_end().chars() {
        count += 1;

        if count < input_character.len() - 1 {
            character.push(i);
        }
    }
    println!("{}", character);
}

// Use the .clear() method to empty a String without deallocating its memory capacity.
fn main() {
    //
    let mut var_string: String = String::with_capacity(50);
    var_string.push_str("Hello, world!");

    //
    println!("String = {}", var_string);
    println!("Capacity = {}", var_string.capacity());
    println!("Len = {}", var_string.len());
    println!("");

    //
    var_string.clear();
    println!("String = {}", var_string);
    println!("Capacity = {}", var_string.capacity());
    println!("Len = {}", var_string.len());
}

// Find all byte indices of a specific character (e.g., 'a') in a &str.
