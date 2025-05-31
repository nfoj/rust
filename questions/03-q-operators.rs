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

// attribution
// Declare a mutable variable named counter with an initial value of 5. Increment it by 3 using the addition assignment operator and print the new value.
fn main() {
    //
    let mut counter: u8 = 5;
    counter += 3;

    //
    println!("{:?}", counter);
}

// Create a mutable variable pontuacao initialized with 20. Decrement it by 5 using the subtraction assignment operator and display the final value.
fn main() {
    //
    let mut score: u8 = 20;
    score -= 5;

    //
    println!("{:?}", score);
}

// Declare a mutable variable fator with the value 2. Multiply it by 4 using the multiplication assignment operator and print the result.
fn main() {
    //
    let mut value: u8 = 2;
    value *= 4;

    //
    println!("{:?}", value);
}

// Initialize a mutable variable size with 30. Divide it by 6 using the division assignment operator and show the resulting value.
fn main() {
    //
    let mut size: u8 = 30;
    size /= 6;

    //
    println!("{:?}", size);
}

// Declare a mutable variable mod with the value 17. Calculate the remainder of the division by 5 using the remainder assignment operator and print the value.
fn main() {
    let mut mode: u8 = 17;
    mode %= 5;

    //
    println!("{:?}", mode);
}

// string and &str
// Declare two variables of type &str, part1 with "Hello, " and part2 with "Rust!". Concatenate them and print the result.
fn main() {
    //
    let part1: &str = "Hello, ";
    let part2: &str = "Rust!";

    //
    println!("{:?}", part1.to_owned() + part2);
}

// Create two String variables, s1 with "Language" and s2 with "Rust.". Concatenate them and print the resulting string.
fn main() {
    //
    let s1: String = String::from("Language ");
    let s2: String = String::from("Rust.");

    //
    println!("{:?}", s1 + &s2);
}

// Declare a variable of type &str called prefix with the value "Number: ". Create a variable of type String called number_str with the value "42". Concatenate them and print the result.
fn main() {
    //
    let number: &str = "Number: ";
    let number_str: String = String::from("42");

    //
    println!("{:?}", number.to_owned() + &number_str);
}

// Create a String variable called initial_message with "Welcome ". Declare a &str variable called name with "User". Concatenate them and print the complete message.
fn main() {
    //
    let initial_message = "Welcome ";
    let name: &str = "User";

    //
    println!("{:?}", initial_message.to_owned() + name);
}

// Declare two variables greeting of type String with "Good " and period of type &str with "day!". Concatenate them and print the complete greeting.
fn main() {
    //
    let greeting: String = String::from("Good ");
    let period: &str = "day!";

    //
    println!("{:#?}", greeting.to_owned() + &period);
}

// tuples
// Create a tuple with three u8: (10, 20, 30). Access the first and second elements, add them and print the result.
fn main() {
    //
    let tup: (u8, u8, u8) = (10, 20, 30);
    println!("{}, {}", tup.0, tup.1);
}

// Declare a tuple of two i32s. Initialize it with the values (5, -2). Print the multiplication of these two values.
fn main() {
    //
    let tup: (i32, i32) = (5, -2);
    println!("{:?}", tup.0 * tup.1);
}

// Create a nested tuple: ((1, 2), (3, 4)). Add the first element of the first tuple to the second element of the second tuple and print the result.
fn main() {
    //
    let tup: ((u8, u8), (u8, u8)) = ((1, 2), (3, 4));
    println!("{:?}", tup.0 .0 + tup.1 .1);
}

// Declare two tuples, t1 with (2, 5) and t2 with (8, 1). Add all the elements of the two tuples and print the total.
fn main() {
    //
    let t1: (u8, u8) = (2, 5);
    let t2: (u8, u8) = (8, 1);

    //
    println!("{:?}", t1.0 + t1.1 + t2.0 + t2.1);
}

// Create a tuple with an f64 and an i32. Print each element separately.
fn main() {
    //
    let mut t1: (f32, i32) = (0.0, 0);
    t1.0 = -2.;
    t1.1 = 2;

    //
    println!("{:#?}", t1);
}

// arrays
// Declare an array of 5 i32 with the values [1, 3, 5, 7, 9]. Print the sum of the first and last elements.
fn main() {
    //
    let arr: [i32; 5] = [1, 3, 5, 7, 9];
    println!("{}, {}", arr[0], arr[4]);
}

// Create an array of 3 u16. Initialize it with the values [10, 20, 30]. Print the product of all elements.
fn main() {
    //
    let arr: [i32; 3];
    arr = [10, 20, 30];

    //
    println!("{:?}", arr[0] * arr[1] * arr[2]);
}

// Declare a 2x2 multidimensional array with integers. Sum all elements and print the result.
fn main() {
    //
    let arr: [[i8; 2]; 2];
    arr = [[2, 2], [4, 4]];
    //
    println!("{:?}", arr[0][0] * arr[0][1] * arr[1][0] * arr[1][1]);
}

// Crie dois arrays de dois f32. Some os elementos correspondentes dos dois arrays e imprima os resultados das somas.
// Create two arrays of two f32. Sum the corresponding elements of the two arrays and print the results of the sums.
fn main() {
    //
    let arr1: [i8; 2] = [1, 2];
    let arr2: [i8; 2] = [1, 2];

    //
    println!("{:?}", arr1[0] * arr1[1] * arr2[0] * arr2[1]);
}

// Declare two arrays a1 with [4, 2] and a2 with [3, 6]. Calculate and print the sum of all elements from both arrays.
fn main() {
    //
    let a1: [u8; 2] = [4, 2];
    let a2: [u8; 2] = [3, 6];

    //
    println!("{:?}, {:?}", a1[0] + a1[1], a2[0] + a2[1]);
}

// scope
// Declare a global variable with value 5. Inside a block {} declare another variable with the same name and value 10. Print the value of both variables (the global and the block variable) outside and inside the block.
fn main() {
    //
    let var: char = 'a';
    println!("{:?}", var);
    //
    {
        let var: char = 'b';
        println!("{:?}", var);
    }
    println!("{:?}", var);
}

// Declare a variable x with value 1. Create an inner block where you declare a variable y with value 2. Inside the block, print the sum of x and y. Outside the block, try to print y (what will happen?).
fn main() {
    //
    let x: u8 = 1;
    {
        let y: u8 = 2;
        println!("{:?}", x + y);
    }
    println!("{:?}", x);
    // println!("{:?}", y); cannot find value `y` in this scope
}

// Declare a variable level1 with value 10. Open a new block and declare a variable level2 with value 20. Inside this block, create another block and declare level3 with value 30. Print the sum of level1 and level3 inside the innermost block.
fn main() {
    //
    let x: u8 = 1;
    {
        //
        let y: u8 = 2;
        {
            //
            let z: u8 = 3;
            println!("{:?}", x + y + z);
        }
        println!("{:?}", y);
    }
    println!("{:?}", x);
}

// Declare a main variable with value 100. Create a block where you declare a variable with the same name and assign the value of main plus 50 to it. Print the value of the variable inside the block and outside the block.
fn main() {
    //
    let num: u8 = 100;
    println!("{:?}", num);
    {
        let mut num: u8 = 100;
        println!("{:?}", num);
        //
        {
            num += 50;
            println!("{:?}", num);
        }
        println!("{:?}", num);
    }
    println!("{:?}", num);
}

// precedence
// Given the array values with [2, 3], write code that calculates and prints the result of values[0]+values[1]∗values[1].
fn main() {
    //
    let var: [u8; 2] = [2, 3];
    println!("{:?}", var[0] + var[1] * var[1]);
}

// Using the same array values, write code that calculates and prints the result of (values[0]+values[1])∗values[1].
fn main() {
    //
    let var: [u8; 2] = [2, 3];

    //
    println!("{:?}", (var[0] + var[1]) * var[1]);
}

// Still with values, calculate and print values[0]∗values[1]/values[1]%values[1].
fn main() {
    //
    let var: [u8; 2] = [2, 3];

    //
    println!("{:?}", var[0] * var[1] / var[1] % var[1]);
}

// Calculate and print values[0]+values[1]−values[0]+values[0]∗values[0].
fn main() {
    //
    let var: [u8; 2] = [2, 3];

    //
    println!("{:?}", var[0] + var[1] - var[0] + var[0] * var[0]);
}

// Calculate and print ((values[0]+values[1])−(values[0]+values[0])∗values[0]).
fn main() {
    //
    let var: [i8; 2] = [2, 3];

    //
    println!("{:?}", (var[0] + var[1]) - (var[0] + var[0]) * var[0]);
}

// Comparison
// Declare duas variáveis inteiras, num1 com valor 10 e num2 com valor 10. Escreva um código que imprima se num1 é igual a num2.
// Declare two integer variables, num1 with value 10 and num2 with value 10. Write code that prints if num1 is equal to num2.

// Declare duas variáveis de ponto flutuante, f1 com 3.14 e f2 com 2.71. Escreva um código que imprima se f1 é diferente de f2.
// Declare two floating-point variables, f1 with 3.14 and f2 with 2.71. Write code that prints if f1 is different from f2.

// Declare duas variáveis inteiras, idade1 com 25 e idade2 com 30. Escreva um código que imprima se idade1 é maior que idade2.
// Declare two integer variables, age1 with 25 and age2 with 30. Write code that prints if age1 is greater than age2.

// Declare duas variáveis inteiras, ponto1 com 5 e ponto2 com 8. Escreva um código que imprima se ponto1 é menor que ponto2.
// Declare two integer variables, point1 with 5 and point2 with 8. Write code that prints if point1 is less than point2.

// Declare duas variáveis inteiras, nota1 com 7 e nota2 com 7. Escreva um código que imprima se nota1 é maior ou igual a nota2.
// Declare two integer variables, grade1 with 7 and grade2 with 7. Write code that prints if grade1 is greater than or equal to grade2.

// Declare duas variáveis inteiras, altura1 com 170 e altura2 com 165. Escreva um código que imprima se altura1 é menor ou igual a altura2.
// Declare two integer variables, height1 with 170 and height2 with 165. Write code that prints if height1 is less than or equal to height2.

// Declare duas variáveis booleanas, verdadeiro1 com true e verdadeiro2 com true. Compare se são iguais e imprima o resultado.
// Declare two boolean variables, true1 with true and true2 with true. Compare if they are equal and print the result.

// Declare uma variável inteira x com 5 e outra y com 10. Verifique se x não é igual a y e imprima o resultado.
// Declare an integer variable x with 5 and another y with 10. Check if x is not equal to y and print the result.

// Declare uma variável temp1 com 22.5 e temp2 com 20.0. Verifique se temp1 é maior que temp2 e imprima.
// Declare a variable temp1 with 22.5 and temp2 with 20.0. Check if temp1 is greater than temp2 and print.

// Declare uma variável count1 com 100 e count2 com 99. Verifique se count1 é menor que count2 e imprima.
// Declare a variable count1 with 100 and count2 with 99. Check if count1 is less than count2 and print.

// Logical
// Declare duas variáveis booleanas, cond1 como true e cond2 como true. Imprima o resultado da operação lógica AND (&&) entre elas.
// Declare two boolean variables, cond1 as true and cond2 as true. Print the result of the logical AND (&&) operation between them.

// Declare flag1 como false e flag2 como false. Imprima o resultado da operação lógica AND entre elas.
// Declare flag1 as false and flag2 as false. Print the result of the logical AND operation between them.

// Declare ativo como true e permitido como false. Imprima o resultado de ativo && permitido.
// Declare active as true and allowed as false. Print the result of active && allowed.

// Declare tem_permissao como false e is_admin como true. Imprima o resultado de tem_permissao && is_admin.
// Declare has_permission as false and is_admin as true. Print the result of has_permission && is_admin.

// Declare duas variáveis booleanas, opcao1 como true e opcao2 como true. Imprima o resultado da operação lógica OR (||) entre elas.
// Declare two boolean variables, option1 as true and option2 as true. Print the result of the logical OR (||) operation between them.

// Declare erro1 como false e erro2 como false. Imprima o resultado de erro1 || erro2.
// Declare error1 as false and error2 as false. Print the result of error1 || error2.

// Declare conectado como true e tem_dados como false. Imprima o resultado de conectado || tem_dados.
// Declare connected as true and has_data as false. Print the result of connected || has_data.

// Declare sucesso como false e falha como true. Imprima o resultado de sucesso || falha.
// Declare success as false and failure as true. Print the result of success || failure.

// Declare uma variável booleana estado como true. Imprima o resultado da negação lógica (!) de estado.
// Declare a boolean variable state as true. Print the result of the logical NOT (!) of state.

// Declare uma variável booleana invalido como false. Imprima o resultado de !invalido.
// Declare a boolean variable invalid as false. Print the result of !invalid.

// Arithmetic (Extra), Assignment, String/&str, Tuples, Arrays (combining concepts)
// Declare duas variáveis u8, val1 com 8 e val2 com 3. Calcule a soma, subtraia 1 do resultado e imprima.
// Declare two u8 variables, val1 with 8 and val2 with 3. Calculate the sum, subtract 1 from the result, and print.

// Declare uma variável mutável texto com "&str" "Olá". Adicione a ela a String ", Mundo!". Imprima o resultado.
// Declare a mutable variable text with "&str" "Hello". Append to it the String ", World!". Print the result.

// Crie uma tupla (u32, u32) com valores (5, 10). Multiplique os dois elementos e atribua o resultado a uma nova variável, imprimindo-a.
// Create a tuple (u32, u32) with values (5, 10). Multiply the two elements and assign the result to a new variable, then print it.

// Declare um array [i32; 3] com valores [2, 4, 6]. Divida cada elemento por 2 (usando atribuição de divisão se aplicável) e imprima o array resultante.
// Declare an array [i32; 3] with values [2, 4, 6]. Divide each element by 2 (using division assignment if applicable) and print the resulting array.

// Declare duas variáveis n_a como 7 e n_b como 4. Calcule o resto da divisão de n_a por n_b e imprima se o resto é igual a 3.
// Declare two variables n_a as 7 and n_b as 4. Calculate the remainder of the division of n_a by n_b and print if the remainder is equal to 3.

// Crie uma String "Número: ". Concatene com o resultado da soma de dois u8 (por exemplo, 5 + 7) convertido para String. Imprima o resultado.
// Create a String "Number: ". Concatenate with the result of the sum of two u8s (e.g., 5 + 7) converted to String. Print the result.

// Declare uma tupla (f64, f64) com (2.5, 3.5). Some os elementos e imprima o resultado formatado com duas casas decimais.
// Declare a tuple (f64, f64) with (2.5, 3.5). Sum the elements and print the result formatted to two decimal places.

// Declare um array [u16; 2] com [100, 200]. Multiplique o primeiro elemento por 2 e o segundo por 3, depois some os resultados e imprima.
// Declare an array [u16; 2] with [100, 200]. Multiply the first element by 2 and the second by 3, then sum the results and print.

// Declare uma variável mutável mensagem com "&str" "Inicio". Adicione a ela a String " -> Fim". Imprima o valor final de mensagem.
// Declare a mutable variable message with "&str" "Start". Append to it the String " -> End". Print the final value of message.

// Crie uma tupla ((i8, i8), i8) com ((1, 2), 3). Some todos os três números e imprima o resultado.
// Create a tuple ((i8, i8), i8) with ((1, 2), 3). Sum all three numbers and print the result.

// Scope and Precedence (combining)
// Declare uma variável nivel_externo com 5. Dentro de um bloco, declare nivel_interno com 10. Imprima a soma de ambas dentro do bloco. Fora do bloco, tente imprimir nivel_interno (o que acontece?).
// Declare an outer_level variable with 5. Inside a block, declare inner_level with 10. Print the sum of both inside the block. Outside the block, try to print inner_level (what happens?).

// Dado um array dados com [3, 2], calcule e imprima o resultado de dados[0]+dados[1]∗5. Em seguida, calcule e imprima (dados[0]+dados[1])∗5.
// Given an array data with [3, 2], calculate and print the result of data[0]+data[1]∗5. Then, calculate and print (data[0]+data[1])∗5.

// Declare uma variável base com 4. Dentro de um bloco, declare expoente com 3. Calcule e imprima baseexpoente dentro do bloco (sem usar pow, apenas multiplicação repetida).
// Declare a base variable with 4. Inside a block, declare exponent with 3. Calculate and print baseexponent inside the block (without using pow, just repeated multiplication).

// Dado um array valores_calc com [10, 3, 2], calcule e imprima valores_calc[0]/valores_calc[1]+valores_calc[2]. Em seguida, calcule e imprima valores_calc[0]/(valores_calc[1]+valores_calc[2]).
// Given an array calc_values with [10, 3, 2], calculate and print calc_values[0]/calc_values[1]+calc_values[2]. Then, calculate and print calc_values[0]/(calc_values[1]+calc_values[2]).

// Declare uma variável fator_global com 2. Dentro de um bloco, declare um array nums com [1, 2, 3]. Some os elementos do array e multiplique o resultado por fator_global, imprimindo o final dentro do bloco.
// Declare a global_factor variable with 2. Inside a block, declare an array nums with [1, 2, 3]. Sum the array elements and multiply the result by global_factor, printing the final result inside the block.
