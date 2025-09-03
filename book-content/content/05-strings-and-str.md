## Strings and &str

Think of your memory as a cabinet and a variable as a drawer.

When you create a "String," you own the entire drawer and can freely modify its contents.

When you create a "&str," you're simply looking at a piece of paper inside the drawer—you can read it, but you don't own it and can't modify the original.

| Feature     | &str                    | String              |
|-------------|-------------------------|---------------------|
| Ownership   | Borrowed                | Owned               |
| Mutability  | Immutable content       | Can be mutable      |
| Memory      | Stack (for literals)    | Heap allocated      |
| Size        | Fixed                   | Growable            |
| Performance | Very fast               | Slightly slower     |
| Use case    | Reading, passing around | Building, modifying |

## String

```rust
//
let empty_string: String = String::new();
println!("{}", empty_string);

// from()
let from_literal: String = String::from("Hello, World!");
println!("{}", from_literal);

// to_string()
let to_string_method: String = "Hello, World!".to_string();
println!("{}", to_string_method);

// mut
let mut mutable_string: String = String::from("Initial text");
println!("{}", mutable_string);

mutable_string = "New text".to_string();
println!("{}", mutable_string);
```

## String concatenation

```rust
// concate + new variable
let string1: String = String::from("Hello");
let string2: String = String::from(" World");
let combined: String = format!("{}{}", string1, string2);
println!("{}", combined);

//
let string1: String = String::from("Hello");
let string2: String = String::from(" World");
println!("{}{}", string1, string2); 
```

## push_str(): strings/text

```rust
let mut text: String = String::new();
text.push_str("Hello");
text.push_str(" World!");
println!("{:?}", text);
```

## push() for individual characters

```rust
let mut text: String = String::new();
text.push('H');
text.push('e');
text.push('e');
text.push('o');
text.push(' ');
text.push('W');
text.push('o');
text.push('r');
text.push('l');
text.push('d');
text.push('!');
println!("{:?}", text);
```

## using +

```rust
let string1: String = String::from("Hello");
let string2: String = String::from(" World");

let combined: String = string1 + &string2;
println!("{}", combined); 
println!("{}", string2); 

// This would cause an error, value used after move:
// println!("{}", string1); 
```

## &str

```rust
// 
let text_slice: &str = "Hello, World!";
println!("{}", text_slice); // Hello, World!

// inference
let text_slice = "Hello, World!";
println!("{}", text_slice); // Hello, World!

// mut
let mut text_slice = "Initial text";
println!("{}", text_slice);

text_slice = "New text"; 
println!("{}", text_slice); 
```

## Concatenation always produces a new String

```rust
let part1 = "Hello";
let part2 = " World";
let part3 = "!";

// concatenate
let result1: String = part1.to_string() + part2 + part3;
println!("{}", result1); // Hello World!

// format!
let result2: String = format!("{}{}{}", part1, part2, part3);
println!("{}", result2); // Hello World!

// to_owned()
let result3: String = part1.to_owned() + part2 + part3;
println!("{}", result3); // Hello World!
```
