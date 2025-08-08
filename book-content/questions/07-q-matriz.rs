// Questions - Matriz

/*

[1, 2, 3],
[4, 5, 6],
[7, 8, 9]

*/

// Crie uma matriz 3x3.
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
}

// Imprima todos os valores da primeira linha
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("{:?}", arr_3x3[0]);
}

// Imprima todos os valores da segunda coluna
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for i in arr_3x3.iter() {
        println!("{:?}", i[1]);
    }
}

// Crie uma matriz 3x3 e imprima os valores da posicao a seguir: A(1,2), B(2, 2), C(0,2).
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let a = arr_3x3[1][2];
    let b = arr_3x3[2][2];
    let c = arr_3x3[0][2];

    println!("A(1,2) = {}\nB(3,3) = {}\nC(0,3) = {}", a, b, c);
}

// Crie um programa que leia a 3 linha am contrario
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    //
    let mut arr_3x3_reverse = arr_3x3[2];
    arr_3x3_reverse.reverse();
    println!("{:?}", arr_3x3_reverse);
}

// Imprima todos os valores de uma matriz 3x3, usando for.
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    //
    for i in arr_3x3.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        println!("");
    }
}

// Imprima a linha diagonal da matriz
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for i in 0..arr_3x3.len() {
        println!("{:?}", arr_3x3[i][i]);
    }
}

// imprima a diagonal inversa de uma matriz 3x3
fn main() {
    //
    let arr_3x3: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let size = arr_3x3.len();

    for i in 0..size {
        let j = size - 1 - i;
        println!("{}", arr_3x3[i][j]);
    }
}
