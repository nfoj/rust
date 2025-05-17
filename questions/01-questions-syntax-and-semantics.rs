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
    print("Print: whihout adding a newline at the end!");
}

// Which macro would you use to print text and automatically add a newline at the end? Give an example.
// println!();
fn main () {
    println!("Print: add newline at the end!");
}

// line-break
// How do you insert an explicit newline within a string that is being printed with println!?
fn main () {
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

    let number = 42;
    let formated = format!("The value is: {}", number);
    println!("{}", formated);
}

// How can you format the number 7 so that it is displayer as "0007" using format!?
// What is the utily of "{:?}" in the format! macro? Give an example of when you would use it.
// What does the specifier "{:#?}" do differently from "{:?}" when formatting a tuple or struct?

Variáveis

    Como você declara uma variável chamada pontuacao e atribui a ela o valor 100?
    Escreva uma linha de código que declare uma variável cidade com o valor "Paris" e, em seguida, a imprima na tela.

Mutabilidade

    Qual palavra-chave é usada para declarar uma variável que pode ter seu valor alterado após a inicialização?
    Declare uma variável mutável chamada contador inicializada com 0. Em seguida, escreva o código para alterar o valor de contador para 5 e imprimi-lo.

Constantes

    Como você declara uma constante chamada VELOCIDADE_MAXIMA com o valor 9000 do tipo i32?
    É possível alterar o valor de uma constante após sua declaração?

Shadowing (Sombreamento)

    O que é "shadowing" em Rust? Explique com um exemplo de código e qual seria a saída.
    Qual será a saída do seguinte código?
    Rust

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("O valor de x interno é: {}", x);
    }
    println!("O valor de x externo é: {}", x);

Escopo

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

Tipos Inteiros (u e i)

    Qual é o intervalo de valores que um tipo u8 pode armazenar?
    Qual é o intervalo de valores que um tipo i8 pode armazenar?
    Como você declara uma variável idade_maxima do tipo u16 com o valor 150?
    Como você pode imprimir os valores mínimo e máximo para o tipo u32 usando as constantes associadas ao tipo?

Tipos de Ponto Flutuante (f)

    Declare uma variável chamada preco do tipo f32 com o valor 19.99.
    Qual dos tipos f32 ou f64 oferece maior precisão?

Tipo char

    Como você declara uma variável chamada inicial que armazena o caractere 'P'?
    Um tipo char em Rust pode armazenar apenas caracteres ASCII? Explique.

Tipo bool

    Declare uma variável chamada esta_chovendo e atribua a ela o valor true.
    Quais são os dois únicos valores possíveis para uma variável do tipo bool?

Tuplas

    Como você declara uma tupla chamada produto que armazena o nome de um item (string), sua quantidade (inteiro) e seu preço (ponto flutuante)? Por exemplo: ("Caneta", 10, 1.50).
    Dado a tupla let coordenadas = (10, 20, 30);, como você desestruturaria essa tupla em três variáveis separadas x, y, e z?
    Como você acessaria o segundo elemento da tupla let rgb = (255, 0, 128);?

Arrays

    Declare um array chamado notas que pode armazenar 5 números do tipo f32. Inicialize-o com alguns valores.
    Qual é a principal diferença entre um array e uma tupla em termos de tipos de dados que eles podem armazenar?
    Como você acessaria o primeiro elemento do array let cores = ["vermelho", "verde", "azul"];?

Operadores Aritméticos e Precedência

    Qual é o resultado da expressão 10 % 3?
    Qual é o resultado da expressão 5.0 / 2.0?
    Qual será o resultado da seguinte expressão em Rust, e por quê? println!("{}", 20 - 5 * 2);
    Qual será o resultado da seguinte expressão em Rust, e por quê? println!("{}", (20 - 5) * 2);
