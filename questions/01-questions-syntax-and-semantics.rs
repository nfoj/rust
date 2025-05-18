// Questions - Syntax-and-semantics

// comments
// How would you write a single-line comment in rust?
// single-line comment

// What is the syntax for a block comments in rust?
/* block comments */

// print
// Which macro would you use to print text on the same line, whihout adding a newline at the end?  Give an example.
// print();
fn main () {
    //
    print("Print: whihout adding a newline at the end!");
}

// Which macro would you use to print text and automatically add a newline at the end? Give an example.
// println!();
fn main () {
    //
    println!("Print: add newline at the end!");
}

// line-break
// How do you insert an explicit newline within a string that is being printed with println!?
fn main () {
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
fn main () {
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
    x:i32,
    y:i32,
}

fn main () {
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

fn main () {
    let rect = Rectangle { width: 10, height: 20 };
    println!("Debug format: {:?}", rect);
    println!(" ");
    println!("Debug format: {:#?}", rect);
}

// variables
// How do you declare a variable named score and assign in the value 100?
fn main () {

    //
    let score: u8 = 100;
    println!("Value: {}", score);
    
}

// Escreva uma linha de código que declare uma variável cidade com o valor "Paris" e, em seguida, a imprima na tela.
// Write a line of code that declares a variable city with the value "Paris" and then prints it to the screen.
fn main () {

    //
    let city: &str = "Paris";
    println!("City: {}!", city); // output = City: Paris
    println!("City: {:?}!", city); // output = City: "Paris"
    
}

// mutability
// Which keyword is used to declare a variable that can have its value changed after initialization?
// mut

// Declare a mutable variable named counter initialized with 0. Then, write the code to change the value of counter to 5 and print it.
fn main () {

      //
      let mut counter: u8 = 0;
      println!("Counter = {:?}", counter);
      
      counter = 5;
      println!("Counter = {:?}", counter);

}

// constants
// How do you declare a constant named MAX_SPEED with the value 9000 of type i32?
fn main () {

    //
    const MAX_SPEED: i32 = 9000;
    println!("Max Speed: {:?}", MAX_SPEED);

}

// Is it possible to change the value of a constant after its declaration?
// no
 
// shadowing
// What is "shadowing" in Rust? Explain with a code example and what the output would be.
// Shadowing in Rust is the ability to declare a new variable with the same name as a previous variable within the same scope.
fn main () {

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


// Escopo

    Qual será a saída do seguinte código? Explique por quê.
    Rust

    let a = 10;
    {
        let b = 20;
        println!("Dentro do bloco: a = {}, b = {}", a, b);
    }
    // println!("Fora do bloco: a = {}, b = {}", a, b); // Linha comentada
    println!("Fora do bloco: a = {}", a);

    O que aconteceria se a linha comentada fosse descomentada e compilada?



// Tipos Inteiros (u e i)
// Qual é o intervalo de valores que um tipo u8 pode armazenar?
// Qual é o intervalo de valores que um tipo i8 pode armazenar?
// Como você declara uma variável idade_maxima do tipo u16 com o valor 150?
// Como você pode imprimir os valores mínimo e máximo para o tipo u32 usando as constantes associadas ao tipo?


// Tipos de Ponto Flutuante (f)
// Declare uma variável chamada preco do tipo f32 com o valor 19.99.
// Qual dos tipos f32 ou f64 oferece maior precisão?


// Tipo char
// Como você declara uma variável chamada inicial que armazena o caractere 'P'?
// Um tipo char em Rust pode armazenar apenas caracteres ASCII? Explique.


// Tipo bool
// Declare uma variável chamada esta_chovendo e atribua a ela o valor true.
// Quais são os dois únicos valores possíveis para uma variável do tipo bool?


// Tuplas
// Como você declara uma tupla chamada produto que armazena o nome de um item (string), sua quantidade (inteiro) e seu preço (ponto flutuante)? Por exemplo: ("Caneta", 10, 1.50).
// Dado a tupla let coordenadas = (10, 20, 30);, como você desestruturaria essa tupla em três variáveis separadas x, y, e z?
// Como você acessaria o segundo elemento da tupla let rgb = (255, 0, 128);?


// Arrays
// Declare um array chamado notas que pode armazenar 5 números do tipo f32. Inicialize-o com alguns valores.
// Qual é a principal diferença entre um array e uma tupla em termos de tipos de dados que eles podem armazenar?
// Como você acessaria o primeiro elemento do array let cores = ["vermelho", "verde", "azul"];?


// Operadores Aritméticos e Precedência
// Qual é o resultado da expressão 10 % 3?
// Qual é o resultado da expressão 5.0 / 2.0?
// Qual será o resultado da seguinte expressão em Rust, e por quê? println!("{}", 20 - 5 * 2);
// Qual será o resultado da seguinte expressão em Rust, e por quê? println!("{}", (20 - 5) * 2);
