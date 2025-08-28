// Questions - Methods

// Crie um codigo que leia a quantidade de letras de um texto digitado pelo usuario.
use std::io;

fn main() {
    //
    println!("Enter text:");
    let mut input_text: String = String::new();
    io::stdin().read_line(&mut input_text).err();

    let text_test = input_text.trim().len();
    println!("Text size = {}", input_text);
}
