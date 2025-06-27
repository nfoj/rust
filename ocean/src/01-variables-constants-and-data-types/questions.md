# Questions - Variables Constants and Data Types

---
01 - How would you write a single-line comment in rust?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=fa5635f01f8529974ad29fcad714a6ff)

02 - What is the syntax for a block comments in rust?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9eef6021ef3fd0cda684697b68fb29a3)

03 - Which macro would you use to print text on the same line, whihout adding a newline at the end?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ffa6c6172b2088f36a0bb092c1d69afd)

04 - Which macro would you use to print text and automatically add a newline at the end?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=bbf1b87290e47b4d63762d07be948e7e)

05 - How do you insert an explicit newline within a string that is being printed with println!?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c18cc1eb489ca7399c8d797f6af9e6a5)

06 - What would be printed by the following code?
```
println!("First line\\nSecond line"); 
```

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f9933474d4f54c4c5c50167fcb9ffa06)

07 - What will be the output of the following code?
``` 
let name = "Ana";
let age = 30;
let text = format!("Hello, {}! You are {} years old.", name, age);
println!("{}", text);
```

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=66b53067454a20b107933a77820f0c42)

08 - How would you use the format! macro to create the string "The value is: 42" from the number 42?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=1ed69cf700df24418c0c55c5cde8c4f4)

09 - How can you format the number 7 so that it is displayer as "0007" using format!?
 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0df831110a2b60915f120a25b1399c06)

10 - What is the utily of "{:?}" in the format! macro? Give an example of when you would use it.

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=670868879cb54931c2ff8bf98de968a9)

11 - What does the specifier "{:#?}" do differently from "{:?}" when formatting a tuple or struct?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b534638ecb6ea4cc4b269ae0802c1bfd)

12 - How do you declare a variable named score and assign in the value 100?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=487b3cbc4b51474715d196cf24a54899)

13 - Write a line of code that declares a variable city with the value "Paris" and then prints it to the screen.

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b8fcb114295f9dc6b824778683735d34)

15 - Which keyword is used to declare a variable that can have its value changed after initialization?
  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=1d26ec751414229f76fad50bf10b45e2)

16 - Declare a mutable variable named counter initialized with 0. Then, write the code to change the value of counter to 5 and print it.

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9d0dbd340798526b8af17861dac4c10f)

17 - How do you declare a constant named MAX_SPEED with the value 9000 of type i32?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9818b68a6204c70c9b9ff767ae640ccb)

18 - Is it possible to change the value of a constant after its declaration?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3e186423e1b02403a6489773539ab55d)

19 - What is "shadowing" in Rust? Explain with a code example and what the output would be.

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=aacfd95ca5035c54c55cc00cc5a35ef4)

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

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=29acc4265a22d22caa0e1e9c8ebec2a1)

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

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=20123a0e1fb08faaaa95fbe669b38fd5)

22 - What would happen if the commented line were uncommented and compiled?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b0fb33a31ace400c7958947b73f2bca4)

23 - What is the range of values that a u8 type can store?

 [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=cc7a5ff024c87dceaa7364920ed84d5c)

24 - What is the range of values that an i8 type can store?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=121241dc00402af3cba3f08f47996427)

25 - How do you declare a variable maximum_age of type u16 with the value 150?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=960a6771cb88056c79bfcb5e29aa9fa8)
  
26 - How can you print the minimum and maximum values for the u32 type using its associated constants?
  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=fa9ce766e67a11d4b2ec3f63d7e0383d)

27 - Declare a variable named 'price' of type f32 with the value 19.99.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e77d724638b13ea2ab70724acc9eb349)
    
28 - Which of the types f32 or f64 offers greater precision?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c0b536eb9115bacade017e9235e1390e)
    
29 - How do you declare a variable named 'initial' that stores the character 'P'?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0aa9d42ef8dfde633e562d54d8488249)
    
30 - Can a char type in Rust store only ASCII characters? Explain.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a2367bdb18135f5d79b06ecf3e0d75fa)
    
31 - Declare a variable named 'rain' and assign the value true to it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=cc276fc9404a4321d32bbbe299ea0d56)
    
32 - What are the only two possible values for a variable of type bool?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=aed5d71ff42bf8d07364d18d6e26f644)
    
33 - How do you declare a tuple named 'product' that stores an item's name (string), its quantity (integer), and its price (floating-point)? For example: ("Pen", 10, 1.50).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a5bf6056eb8f5be1f01aa1d61b065536)

34 - Given the tuple 'let coordinates = (10, 20, 30);', how would you destructure this tuple into three separate variables x, y, and z?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=5fd94401acad8e2e5818b2d262e445b1)
    
35 - How would you access the second element of the tuple 'let rgb = (255, 0, 128):'?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8c5e493c268b625e837a7ff63689eb77)
    
36 - Declare an array named 'notes' that can store 5 numbers of type f32. Initialize it with some values.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=20591a94ade8f226ee643ebdb4d37d82)
    
37 - What is the main difference between an array and a tuple in terms of the data types they can store?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=97f67c372ff407555be540707a5df1bc)
    
38 - How would you access the first element of the array: let cores = ["red", "green", "blue"];?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=783ac15d71fa170ba5166fd884d9fdcb)
    
39 - What is the result of the expression 10 % 3?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=7d3fdc6c7c4879c3854fbaa87d9f910f)
    
40 - What is the result of the expression 5.0 / 2.0?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=90ff979dab835a73d3d0af9376e0f359)
    
41 - What will be the result of the following expression in Rust, and why? println!("{}", 20 - 5 * 2);
  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3dc47883e25713d88cec14a9d35d0dde)

42 - What will be the result of the following expression in Rust, and why? println!("{}", (20 - 5) * 2);

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8c121f9ed99c9fae2097aaea52290c79)
