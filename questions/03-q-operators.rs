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

// comparison
// Declare two integer variables, num1 with value 10 and num2 with value 10. Write code that prints if num1 is equal to num2.
fn main() {
    //
    let num1: i8 = 10;
    let num2: i8 = 10;

    //
    println!("Num1 = Num2? {:?} ", num1 == num2);
}

// Declare two floating-point variables, f1 with 3.14 and f2 with 2.71. Write code that prints if f1 is different from f2.
fn main() {
    //
    let f1: f32 = 3.14;
    let f2: f32 = 2.71;

    //
    println!("f1 != f2? {}", f1 != f2);
}

// Declare two integer variables, age1 with 25 and age2 with 30. Write code that prints if age1 is greater than age2.
fn main() {
    //
    let age1: u8 = 25;
    let age2: u8 = 30;

    //
    println!("Age-1 > Age-2? {}", age1 > age2);
}

// Declare two integer variables, point1 with 5 and point2 with 8. Write code that prints if point1 is less than point2.
fn main() {
    //
    let point_1: u8 = 5;
    let point_2: u8 = 8;

    //
    println!("Point-1 < Point-2? {}", point_1 < point_2);
}

// Declare two integer variables, grade1 with 7 and grade2 with 7. Write code that prints if grade1 is greater than or equal to grade2.
fn main() {
    //
    let grade_1: u8 = 7;
    let grade_2: u8 = 7;

    //
    println!("Grade-1 >= Grade-2? {}", grade_1 >= grade_2);
}

// Declare two integer variables, height1 with 170 and height2 with 165. Write code that prints if height1 is less than or equal to height2.
fn main() {
    //
    let height_1: f32 = 1.70;
    let height_2: f32 = 1.65;

    //
    println!("Height-1 <= Height-2? {}", height_1 <= height_2);
}

// Declare two boolean variables, true1 with true and true2 with true. Compare if they are equal and print the result.
fn main() {
    //
    let true_1: bool = true;
    let true_2: bool = true;

    //
    println!("True-1 == True-2? {}", true_1 == true_2);
}

// Declare an integer variable x with 5 and another y with 10. Check if x is not equal to y and print the result.
fn main() {
    //
    let x: u8 = 5;
    let y: u8 = 10;

    //
    println!("x != y? {}", x != y);
}

// Declare a variable temp1 with 22.5 and temp2 with 20.0. Check if temp1 is greater than temp2 and print.
fn main() {
    //
    let temp_1: f32 = 22.5;
    let temp_2: f32 = 20.;

    //
    println!("Temp1 > Temp2? {}", temp_1 > temp_2);
}

// Declare a variable count1 with 100 and count2 with 99. Check if count1 is less than count2 and print.
fn main() {
    //
    let count_1: u8 = 100;
    let count_2: u8 = 99;

    //
    println!("Count 1 < Count 2? {}", count_1 < count_2);
}

// logical
// Declare two boolean variables, cond1 as true and cond2 as true. Print the result of the logical AND (&&) operation between them.
fn main() {
    //
    let cond_1: bool = true;
    let cond_2: bool = true;

    //
    println!("Cond 1 && Cond 2: {}", cond_1 && cond_2);
}

// Declare flag1 as false and flag2 as false. Print the result of the logical AND operation between them.
fn main() {
    //
    let flag_1: bool = false;
    let flag_2: bool = false;

    //
    println!("Flag 1 && Flag 2: {}", flag_1 && flag_2);
}

// Declare active as true and allowed as false. Print the result of active && allowed.
fn main() {
    //
    let active: bool = true;
    let allowed: bool = false;

    //
    println!("Active && Allowed: {}", active && allowed);
}

// Declare has_permission as false and is_admin as true. Print the result of has_permission && is_admin.
fn main() {
    //
    let has_permission: bool = false;
    let is_admin: bool = true;

    //
    println!(
        " Has permission && Is admin: {}",
        has_permission && is_admin
    );
}

// Declare two boolean variables, option1 as true and option2 as true. Print the result of the logical OR (||) operation between them.
fn main() {
    //
    let option_1: bool = true;
    let option_2: bool = true;

    //
    println!("Option 1 or Option 2: {}", option_1 || option_2);
}

// Declare error1 as false and error2 as false. Print the result of error1 || error2.
fn main() {
    //
    let error_1: bool = false;
    let error_2: bool = false;

    //
    println!("Error 1 or Error 2: {}", error_1 || error_2);
}

// Declare connected as true and has_data as false. Print the result of connected || has_data.
fn main() {
    //
    let connected: bool = true;
    let has_data: bool = false;

    //
    println!("Connected Or Has data: {}", connected && has_data);
}

// Declare success as false and failure as true. Print the result of success || failure.
fn main() {
    //
    let success: bool = false;
    let failure: bool = true;

    //
    println!("Success OR Failure: {}", success || failure);
}

// Declare a boolean variable state as true. Print the result of the logical NOT (!) of state.
fn main() {
    //
    let var: bool = true;
    println!("{:?}", !var);
}

// Declare a boolean variable invalid as false. Print the result of !invalid.
fn main() {
    //
    let invalid: bool = false;
    println!("{:?}", !invalid);
}

// Arithmetic (Extra), Assignment, String/&str, Tuples, Arrays (combining concepts)
// Declare two u8 variables, val1 with 8 and val2 with 3. Calculate the sum, subtract 1 from the result, and print.
fn main() {
    //
    let val_1: u8 = 8;
    let val_2: u8 = 3;

    //
    println!("{:?}", (val_1 + val_2) - 1);
}

// Create a tuple (u32, u32) with values (5, 10). Multiply the two elements and assign the result to a new variable, then print it.
fn main() {
    //
    let arr_1: (u32, u32) = (5, 10);
    let arr_2 = arr_1.0 * arr_1.1;

    //
    println!("{:?}", arr_2);
}

// Declare an array [i32; 3] with values [2, 4, 6]. Divide each element by 2 (using division assignment if applicable) and print the resulting array.
fn main() {
    //
    let mut arr: [i32; 3] = [2, 4, 6];
    arr[0] /= 2;
    arr[1] /= 2;
    arr[2] /= 2;

    //
    println!("{:#?}", arr);
}

// Declare two variables n_a as 7 and n_b as 4. Calculate the remainder of the division of n_a by n_b and print if the remainder is equal to 3.
fn main() {
    //
    let n_a: u8 = 7;
    let n_b: u8 = 4;

    //
    let rema: u8 = n_a % n_b;
    println!("Remainder: {:?} = 3? {:?}", rema, rema == 3);
}

// Create a String "Number: ". Concatenate with the result of the sum of two u8s (e.g., 5 + 7) converted to String. Print the result.
fn main() {
    //
    let num: String = String::from("Number: ");
    let sum: u8 = 5 + 7;

    //
    println!("{}{:?}", num, sum);
}

// Declare a tuple (f64, f64) with (2.5, 3.5). Sum the elements and print the result formatted to two decimal places.
fn main() {
    //
    let tup: (f64, f64) = (2.5, 3.5);
    let result: f64 = tup.0 + tup.1;

    //
    println!("{:.2}", result);
}

// Declare an array [u16; 2] with [100, 200]. Multiply the first element by 2 and the second by 3, then sum the results and print.
fn main() {
    //
    let arr: [u16; 2] = [100, 200];
    println!("{:?}", (arr[0] * 2) + (arr[1] * 3));
}

// Declare a mutable variable message with "&str" "Start". Append to it the String " -> End". Print the final value of message.
fn main() {
    //
    let message: &str = "Start";
    println!("{}", message.to_owned() + " -> End");
}

// string
fn main() {
    //
    let mut message: String = String::from("Start");
    message += " -> End";

    //
    println!("{}", message);
}

// Create a tuple (i8, i8, i8) with (1, 2, 3). Sum all three numbers and print the result.
fn main() {
    //
    let tup: (i8, i8, i8) = (1, 2, 3);
    println!("{:?}", tup.0 + tup.1 + tup.2);
}

// Declare an outer_level variable with 5. Inside a block, declare inner_level with 10. Print the sum of both inside the block. Outside the block, try to print inner_level (what happens?).
fn main() {
    //
    let outer_level: u8 = 5;
    {
        let inner_level: u8 = 10;
        println!("{:?}", outer_level + inner_level);
    }
    // println!("{:?}", outer_level + inner_level);
}

// Given an array data with [3, 2], calculate and print the result of data[0]+data[1]∗5. Then, calculate and print (data[0]+data[1])∗5.
fn main() {
    //
    let data: [u8; 2] = [3, 2];

    //
    println!("{:?}", data[0] + data[1] * 5);
    println!("{:?}", (data[0] + data[1]) * 5);
}

// Given an array calc_values with [10, 3, 2], calculate and print calc_values[0]/calc_values[1]+calc_values[2]. Then, calculate and print calc_values[0]/(calc_values[1]+calc_values[2]).
fn main() {
    //
    let calc_values: [u8; 3] = [10, 3, 2];

    //
    println!("{:?}", calc_values[0] / calc_values[1] + calc_values[2]);
    println!("{:?}", calc_values[0] / (calc_values[1] + calc_values[2]));
}

// Declare a global_factor variable with 2. Inside a block, declare an array nums with [1, 2, 3]. Sum the array elements and multiply the result by global_factor, printing the final result inside the block.
fn main() {
    //
    let global_factor: u8 = 2;
    {
        let block: [u8; 3] = [1, 2, 3];
        let result = (block[0] + block[1] + block[2]) * global_factor;
        println!("{:?}", result);
    }
}
