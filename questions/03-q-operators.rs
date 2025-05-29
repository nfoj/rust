// Questions - Operators: arithmetic, assignment, precedence, comparison and logical

// arithmetic
// Write a program that declares two integer variables, 'a' with the value 15 and 'b' with the value 7, and prints their sum.
fn main() {
    //
    let a: u8 = 15;
    let b: u8 = 7;

    //
    println!("{:?}", a + b);
}

// Create code that subtracts the value 3 from a variable called 'total' that initially equals 10, and displays the result.
fn main() {
    //
    let total = 10;
    let sub = total - 3;
    println!("{:?}", sub);

    //
    let mut total = 10;
    total -= 3;
    println!("{:?}", total);
}

// Develop a program that multiplies two variables, 'x' equals 6 and 'y' equals 8, and shows the product.
fn main() {
    //
    let x: u8 = 6;
    let y: u8 = 8;

    //
    let result: u8 = x * y;

    //
    println!("{:?}", result);
}

// Implement code that divides the number 20 by 5 and prints the quotient.
fn main() {
    //
    let mut number: f32 = 20.;
    number /= 5.;

    //
    println!("{:?}", number);
}

// Write a program that calculates the remainder of the division of 23 by 4 and prints the result.
fn main() {
    //
    let mut remainder: u8 = 23;
    remainder %= 4;

    //
    println!("{:?}", remainder);
}

// Declare two variables num1 (12) and num2 (5) and print the result of their sum directly within the println! macro.
fn main() {
    //
    let num1: u8 = 12;
    let num2: u8 = 5;

    //
    println!("{:?}", num1 + num2);
}

// Create an immutable variable called 'value' that is the sum of 9 and 2, and print its value.
fn main() {
    //
    let value: u8 = 9 + 2;
    println!("{:?}", value);
}

// Declare two variables 'first' (7) and 'second' (3) and print the result of their multiplication.
fn main() {
    //
    let first: u8 = 7;
    let second: u8 = 3;

    //
    println!("{:?}", first * second);
}

// Declare two variables 'dividend' (18) and 'divisor' (3) and store the result of the division in a variable called 'result', printing it.
fn main() {
    //
    let dividend: u8 = 18;
    let divisor: u8 = 3;

    //
    let result: u8 = dividend / divisor;

    //
    println!("{:?}", result);
}

// Declare two variables n1 (25) and n2 (7). Calculate and print the remainder of the division of n1 by n2.
fn main() {
    //
    let n1: u8 = 25;
    let n2: u8 = 7;

    //
    println!("{:?}", n1 % n2);
}

/*

Atribuição

    Declare uma variável mutável chamada contador com o valor inicial de 5. Incremente-a em 3 usando o operador de atribuição de adição e imprima o novo valor.
    Crie uma variável mutável pontuacao iniciada com 20. Decremente-a em 5 usando o operador de atribuição de subtração e exiba o valor final.
    Declare uma variável mutável fator com o valor 2. Multiplique-a por 4 usando o operador de atribuição de multiplicação e imprima o resultado.
    Inicialize uma variável mutável tamanho com 30. Divida-a por 6 usando o operador de atribuição de divisão e mostre o valor resultante.
    Declare uma variável mutável mod com o valor 17. Calcule o resto da divisão por 5 usando o operador de atribuição de resto e imprima o valor.

String e &str

    Declare duas variáveis do tipo &str, parte1 com "Olá, " e parte2 com "Rust!". Concatene-as e imprima o resultado.
    Crie duas variáveis do tipo String, s1 com "Linguagem " e s2 com "Rust.". Concatene-as e imprima a string resultante.
    Declare uma variável do tipo &str chamada prefixo com o valor "Número: ". Crie uma variável do tipo String chamada numero_str com o valor "42". Concatene-as e imprima o resultado.
    Crie uma variável String chamada mensagem_inicial com "Bem-vindo ". Declare uma variável &str chamada nome com "Usuário". Concatene-as e imprima a mensagem completa.
    Declare duas variáveis saudacao do tipo String com "Bom " e periodo do tipo &str com "dia!". Concatene-as e imprima a saudação completa.

Tuplas

    Crie uma tupla com três u8: (10, 20, 30). Acesse o primeiro e o segundo elemento, some-os e imprima o resultado.
    Declare uma tupla de dois i32. Inicialize-a com os valores (5, -2). Imprima a multiplicação desses dois valores.
    Crie uma tupla aninhada: ((1, 2), (3, 4)). Some o primeiro elemento da primeira tupla com o segundo elemento da segunda tupla e imprima o resultado.
    Declare duas tuplas, t1 com (2, 5) e t2 com (8, 1). Some todos os elementos das duas tuplas e imprima o total.
    Crie uma tupla com um f64 e um i32. Imprima cada elemento separadamente.

Arrays

    Declare um array de 5 i32 com os valores [1, 3, 5, 7, 9]. Imprima a soma do primeiro e do último elemento.
    Crie um array de 3 u16. Inicialize-o com os valores [10, 20, 30]. Imprima o produto de todos os elementos.
    Declare um array bidimensional 2x2 com inteiros. Some todos os elementos e imprima o resultado.
    Crie dois arrays de dois f32. Some os elementos correspondentes dos dois arrays e imprima os resultados das somas.
    Declare dois arrays a1 com [4, 2] e a2 com [3, 6]. Calcule e imprima a soma de todos os elementos de ambos os arrays.

Escopo

    Declare uma variável global com valor 5. Dentro de um bloco {} declare outra variável com o mesmo nome e valor 10. Imprima o valor de ambas as variáveis (a global e a do bloco) fora e dentro do bloco.
    Declare uma variável x com valor 1. Crie um bloco interno onde você declara uma variável y com valor 2. Dentro do bloco, imprima a soma de x e y. Fora do bloco, tente imprimir y (o que acontecerá?).
    Declare uma variável nivel1 com valor 10. Abra um novo bloco e declare uma variável nivel2 com valor 20. Dentro deste bloco, crie outro bloco e declare nivel3 com valor 30. Imprima a soma de nivel1 e nivel3 dentro do bloco mais interno.
    Declare uma variável mutável contador_externo com valor 0. Dentro de um bloco, crie um loop que incrementa uma variável local contador_interno de 1 a 5. Após o loop (ainda dentro do bloco), adicione o valor final de contador_interno a contador_externo. Imprima o valor de contador_externo fora do bloco.
    Declare uma variável principal com valor 100. Crie um bloco onde você declara uma variável com o mesmo nome e atribui o valor de principal mais 50 a ela. Imprima o valor da variável dentro do bloco e fora do bloco.

Precedência

    Dado o array valores com [2, 3], escreva um código que calcule e imprima o resultado de valores[0]+valores[1]∗valores[1].
    Usando o mesmo array valores, escreva um código que calcule e imprima o resultado de (valores[0]+valores[1])∗valores[1].
    Ainda com valores, calcule e imprima valores[0]∗valores[1]/valores[1]%valores[1].
    Calcule e imprima valores[0]+valores[1]−valores[0]+valores[0]∗valores[0].
    Calcule e imprima ((valores[0]+valores[1])−(valores[0]+valores[0])∗valores[0]).

Comparação
    Declare duas variáveis inteiras, num1 com valor 10 e num2 com valor 10. Escreva um código que imprima se num1 é igual a num2.
    Declare duas variáveis de ponto flutuante, f1 com 3.14 e f2 com 2.71. Escreva um código que imprima se f1 é diferente de f2.
    Declare duas variáveis inteiras, idade1 com 25 e idade2 com 30. Escreva um código que imprima se idade1 é maior que idade2.
    Declare duas variáveis inteiras, ponto1 com 5 e ponto2 com 8. Escreva um código que imprima se ponto1 é menor que ponto2.
    Declare duas variáveis inteiras, nota1 com 7 e nota2 com 7. Escreva um código que imprima se nota1 é maior ou igual a nota2.
    Declare duas variáveis inteiras, altura1 com 170 e altura2 com 165. Escreva um código que imprima se altura1 é menor ou igual a altura2.
    Declare duas variáveis booleanas, verdadeiro1 com true e verdadeiro2 com true. Compare se são iguais e imprima o resultado.
    Declare uma variável inteira x com 5 e outra y com 10. Verifique se x não é igual a y e imprima o resultado.
    Declare uma variável temp1 com 22.5 e temp2 com 20.0. Verifique se temp1 é maior que temp2 e imprima.
    Declare uma variável count1 com 100 e count2 com 99. Verifique se count1 é menor que count2 e imprima.

Lógico

    Declare duas variáveis booleanas, cond1 como true e cond2 como true. Imprima o resultado da operação lógica AND (&&) entre elas.
    Declare flag1 como false e flag2 como false. Imprima o resultado da operação lógica AND entre elas.
    Declare ativo como true e permitido como false. Imprima o resultado de ativo && permitido.
    Declare tem_permissao como false e is_admin como true. Imprima o resultado de tem_permissao && is_admin.
    Declare duas variáveis booleanas, opcao1 como true e opcao2 como true. Imprima o resultado da operação lógica OR (||) entre elas.
    Declare erro1 como false e erro2 como false. Imprima o resultado de erro1 || erro2.
    Declare conectado como true e tem_dados como false. Imprima o resultado de conectado || tem_dados.
    Declare sucesso como false e falha como true. Imprima o resultado de sucesso || falha.
    Declare uma variável booleana estado como true. Imprima o resultado da negação lógica (!) de estado.
    Declare uma variável booleana invalido como false. Imprima o resultado de !invalido.

Aritmética (Extra), Atribuição, String/&str, Tuplas, Arrays (combinando conceitos)

    Declare duas variáveis u8, val1 com 8 e val2 com 3. Calcule a soma, subtraia 1 do resultado e imprima.
    Declare uma variável mutável texto com "&str" "Olá". Adicione a ela a String ", Mundo!". Imprima o resultado.
    Crie uma tupla (u32, u32) com valores (5, 10). Multiplique os dois elementos e atribua o resultado a uma nova variável, imprimindo-a.
    Declare um array [i32; 3] com valores [2, 4, 6]. Divida cada elemento por 2 (usando atribuição de divisão se aplicável) e imprima o array resultante.
    Declare duas variáveis n_a como 7 e n_b como 4. Calcule o resto da divisão de n_a por n_b e imprima se o resto é igual a 3.
    Crie uma String "Número: ". Concatene com o resultado da soma de dois u8 (por exemplo, 5 + 7) convertido para String. Imprima o resultado.
    Declare uma tupla (f64, f64) com (2.5, 3.5). Some os elementos e imprima o resultado formatado com duas casas decimais.
    Declare um array [u16; 2] com [100, 200]. Multiplique o primeiro elemento por 2 e o segundo por 3, depois some os resultados e imprima.
    Declare uma variável mutável mensagem com "&str" "Inicio". Adicione a ela a String " -> Fim". Imprima o valor final de mensagem.
    Crie uma tupla ((i8, i8), i8) com ((1, 2), 3). Some todos os três números e imprima o resultado.

Escopo e Precedência (combinando)

    Declare uma variável nivel_externo com 5. Dentro de um bloco, declare nivel_interno com 10. Imprima a soma de ambas dentro do bloco. Fora do bloco, tente imprimir nivel_interno (o que acontece?).
    Dado um array dados com [3, 2], calcule e imprima o resultado de dados[0]+dados[1]∗5. Em seguida, calcule e imprima (dados[0]+dados[1])∗5.
    Declare uma variável base com 4. Dentro de um bloco, declare expoente com 3. Calcule e imprima baseexpoente dentro do bloco (sem usar pow, apenas multiplicação repetida).
    Dado um array valores_calc com [10, 3, 2], calcule e imprima valores_calc[0]/valores_calc[1]+valores_calc[2]. Em seguida, calcule e imprima valores_calc[0]/(valores_calc[1]+valores_calc[2]).
    Declare uma variável fator_global com 2. Dentro de um bloco, declare um array nums com [1, 2, 3]. Some os elementos do array e multiplique o resultado por fator_global, imprimindo o final dentro do bloco.
*/
