// Questions - Functions and Method Syntax

// Crie uma funcao que imprima "Hello, world!" na tela.
fn func_print(text: &str) {
    //
    println!("{}", text);
}

fn main() {
    //
    func_print("Hello, world!");
}

// Crie uma funcao que some dois numeros.
fn func_sum(x: u8, y: u8) -> u8 {
    //
    x + y
}

fn main() {
    //
    println!("{}", func_sum(1, 2));
}

// Crie um funcao que possua as soma e subtracao.
fn func_operations(x: u8, y: u8) -> (u8, u8) {
    //
    let sum = x + y;
    let subt = x - y;
    (sum, subt)
}

fn main() {
    //
    let (sum, subt) = func_operations(8, 2);
    println!("Sum = {}\nSubtraction = {}", sum, subt);
}

// or
fn func_operations(x: u8, y: u8) -> (u8, u8) {
    //
    (x + y, x - y)
}

fn main() {
    //
    let (sum, subt) = func_operations(8, 2);
    println!("Sum = {}\nSubtraction = {}", sum, subt);
}

// Crie uma funcao que receba o valor de 2 numeros inteiros.
use std::io;

fn read_input(prompt: &str) -> u8 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    input.trim().parse().expect("Número inválido")
}

fn main() {
    let number_a = read_input("Enter number:");
    let number_b = read_input("\nEnter number:");

    println!("\nNumber A = {}\nNumber B = {}", number_a, number_b);
}

// Crie um codigo que possua uma funcao que receba o valor de 2 numeros inteiros e outra que faca as quatro operacoes basicas, depois imprima o valor na tela:
use std::io;

fn func_operations(x: u8, y: u8) -> (u8, u8, u8, u8) {
    let sum = x + y;
    let sub = x - y;
    let mul = x * y;
    let div = x / y;
    (sum, sub, mul, div)
}

fn read_input(prompt: &str) -> u8 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Data entry error!");

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid Number!");
            read_input(prompt)
        }
    }
}

fn main() {
    let number_a = read_input("Enter number:");
    let number_b = read_input("\nEnter number:");

    let (sum, sub, mul, div) = func_operations(number_a, number_b);

    println!("\nResultados:");
    println!("Sum = {}", sum);
    println!("Sub = {}", sub);
    println!("Mul = {}", mul);
    println!("Div = {}", div);
}

// Crie um codigo usando funcoes que possa mostrar se um numero digitado pelo usuario e par ou impar.
use std::io;

fn read_input(prompt: &str) -> u8 {
    //
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Data entry error!");

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid Number!");
            read_input(prompt)
        }
    }
}

fn func_even_or_odd(x: u8) {
    //
    if x % 2 == 0 {
        println!("Even!");
    } else {
        println!("Odd!");
    }
}

fn main() {
    //
    let num = read_input("Enter number:");
    func_even_or_odd(num);
}

// Crie um programa usando funcoes para identificar se o valor de entrada digitado pelo usuario e positivo, negativo ou zero.
use std::io;

fn read_input(prompt: &str) -> i8 {
    //
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).err();

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid Number!");
            read_input(prompt)
        }
    }
}

fn func_pos_neg_zer(x: i8) {
    //
    if x > 0 {
        print!("Positive!");
    } else if x < 0 {
        print!("Negative!");
    } else {
        print!("Zero!");
    }
}

fn main() {
    let num = read_input("Enter number:");
    func_pos_neg_zer(num);
}

// Crie um programa usando funcoes para receber o valor de 2 numeros digitados pelo usuario e exibir qual o maior
use std::io;

fn read_input(prompt: &str) -> i8 {
    //
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).err();

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid number!");
            read_input(prompt)
        }
    }
}

fn func_larger(x: i8, y: i8) {
    //
    if x > y {
        println!("\nNum A > Num B");
    } else if x < y {
        println!("\nNum A < Num B");
    } else {
        println!("\nNum A == Num B");
    }
}

fn main() {
    let num_a = read_input("Enter number (a):");
    let num_b = read_input("Enter number (b):");

    func_larger(num_a, num_b);
}

// Crie um programa usando funcoes que receba as 3 notas de um aluno e com base nas regras a seguir, mostre se ele passou, ficou de recuperacao ou reprovou.
use std::io;

fn read_input(prompt: &str) -> f32 {
    //
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).err();

    match input.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid number!");
            read_input(prompt)
        }
    }
}

fn func_avarage(x: f32, y: f32, z: f32) {
    //
    let ava: f32 = (x + y + z) / 3.;

    if ava < 5. {
        println!("\nYou failed!");
    } else if ava >= 5. && ava < 7. {
        println!("\nYou are in recovery!");
    } else {
        println!("\nYou passed!");
    }
}

fn main() {
    //
    let num_a = read_input("Enter number (a):");
    let num_b = read_input("Enter number (b):");
    let num_c = read_input("Enter number (c):");

    func_avarage(num_a, num_b, num_c);
}
