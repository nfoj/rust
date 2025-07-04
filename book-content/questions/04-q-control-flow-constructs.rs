// Questions - Control Flow Constructs - if, else, else if, loop, while, for break and continue

// Write a program that asks the user for an integer and determines whether it's even or odd.
use std::io;

fn main() {
    //
    println!("Enter a number:");

    //
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error in the value entered!");

    //
    let num: i32 = input.trim().parse().expect("Error converting data!");

    //
    if num % 2 == 0 {
        println!("Num = {}\nEven!", num);
    } else {
        println!("Num = {}\nOdd!", num);
    }
}

// Create a program that reads a number and prints whether it's positive, negative, or zero.
use std::io;

fn main() {
    //
    println!("Enter a number: ");

    //
    let mut input_num: String = String::new();
    io::stdin()
        .read_line(&mut input_num)
        .expect("Error in the value entered!");

    let num: i8 = input_num.trim().parse().expect("Error converting data!");

    //
    if num > 0 {
        println!("Num = {}!\nPositive", num);
    } else if num < 0 {
        println!("Num = {}!\nNegative", num);
    } else {
        println!("Num = {}!\nZero", num);
    }
}

// Develop a program that receives two numbers and shows which one is larger.
use std::io;

fn main() {
    //
    println!("Enter a number (a):");
    let mut input_num_a: String = String::new();
    io::stdin()
        .read_line(&mut input_num_a)
        .expect("Error in the value entered!");

    //
    println!("Enter a number (b):");
    let mut input_num_b: String = String::new();
    io::stdin()
        .read_line(&mut input_num_b)
        .expect("Error in the value entered!");

    //
    let num_a: i8 = input_num_a.trim().parse().expect("Error converting data!");
    let num_b: i8 = input_num_b.trim().parse().expect("Error converting data!");

    //
    if num_a > num_b {
        println!("Num A > Num B");
    } else if num_a < num_b {
        println!("Num A < Num B");
    } else {
        println!("Num A == Num B");
    }
}

// Create a program that receives a student's grade (from 0 to 10) and reports if they passed (grade >= 7), are in recovery (grade >= 5 and < 7), or failed (grade < 5).
use std::io;

fn main() {
    //
    println!("Enter you grade for your first test:");
    let mut input_test_a: String = String::new();
    io::stdin()
        .read_line(&mut input_test_a)
        .expect("Error entering the first grade!");

    //
    println!("Enter the grade for your second test:");
    let mut input_test_b: String = String::new();
    io::stdin()
        .read_line(&mut input_test_b)
        .expect("Error entering the second grade!");

    //
    println!("Enter the grade for your third test:");
    let mut input_test_c: String = String::new();
    io::stdin()
        .read_line(&mut input_test_c)
        .expect("Error entering the third test!");

    //
    let test_a: f32 = input_test_a
        .trim()
        .parse()
        .expect("Error converting Test A!");
    let test_b: f32 = input_test_b
        .trim()
        .parse()
        .expect("Error converting Test B!");
    let test_c: f32 = input_test_c
        .trim()
        .parse()
        .expect("Error converting Test C!");

    let ava: f32 = (test_a + test_b + test_c) / 3.;

    //
    if ava < 5. {
        println!("You failed!");
    } else if ava >= 5. && ava < 7. {
        println!("You are in recovery!");
    } else {
        println!("You passed!");
    }
}

// Write a program that reads a person's age and determines if they can vote (16 years or older).
use std::io;

fn main() {
    //
    println!("Enter your age:");

    let mut input_age: String = String::new();
    io::stdin()
        .read_line(&mut input_age)
        .expect("Error in the entered!");

    //
    let age: u8 = input_age.trim().parse().expect("Error converting age!");

    //
    if age >= 16 && age <= 59 {
        println!("You are required to vote!");
    } else if age >= 60 && age <= 100 {
        println!("Your vote is optional!");
    } else {
        println!("You cannot vote!");
    }
}

// Develop a program that asks for three numbers and determines which one is the largest.
use std::io;

fn main() {
    // a
    println!("Enter with number! A: ");
    let mut input_a: String = String::new();
    io::stdin()
        .read_line(&mut input_a)
        .expect("Error in the entered!");

    // b
    println!("Enter with number! B: ");
    let mut input_b: String = String::new();
    io::stdin()
        .read_line(&mut input_b)
        .expect("Error in the entered!");

    // c
    println!("Enter with number! C: ");
    let mut input_c: String = String::new();
    io::stdin()
        .read_line(&mut input_c)
        .expect("Error in the entered!");

    //
    let num_a: i8 = input_a.trim().parse().expect("Error converting num A!");
    let num_b: i8 = input_b.trim().parse().expect("Error converting num B!");
    let num_c: i8 = input_c.trim().parse().expect("Error converting num C!");

    //
    if num_a == num_b && num_a == num_c {
        println!("a == b == c");
    }

    if num_a == num_b && num_a > num_c {
        println!("a == b > c");
    }

    if num_a == num_b && num_a < num_c {
        println!("a == b < c");
    }

    if num_a == num_c && num_a > num_b {
        println!("a == c > b");
    }

    if num_a == num_c && num_a < num_b {
        println!("a == c < b");
    }

    if num_b == num_c && num_b > num_a {
        println!("b == c > a");
    }

    if num_b == num_c && num_b < num_a {
        println!("b == c < a");
    }

    if num_a > num_b && num_b > num_c {
        println!("a > b > c");
    }

    if num_a > num_c && num_c > num_b {
        println!("a > c > b");
    }

    if num_b > num_a && num_a > num_c {
        println!("b > a > c");
    }

    if num_c > num_a && num_a > num_b {
        println!("c > a > b");
    }

    if num_c > num_b && num_b > num_a {
        println!("c > b > a");
    }
}

/*

Create a program that converts a numerical score (0 to 100) to a letter grade, following these rules:
 A: 90-100
 B: 80-89
 C: 70-79
 D: 60-69
 F: 0-59

*/

//
use std::io;

fn main() {
    //
    println!("Enter with number (0 - 100):");
    let mut input_num: String = String::new();
    io::stdin()
        .read_line(&mut input_num)
        .expect("Error in the entered!");
    let num: u8 = input_num.trim().parse().expect("Error converting num:");

    //
    if num >= 0 && num <= 59 {
        println!("F");
    } else if num >= 60 && num <= 69 {
        println!("D");
    } else if num >= 70 && num <= 79 {
        println!("C");
    } else if num >= 80 && num <= 89 {
        println!("B");
    } else if num >= 90 && num <= 100 {
        println!("A");
    } else {
        println!("Error!");
    }
}

// Write a program that receives the lengths of the three sides of a triangle and determines if it is equilateral (all sides equal), isosceles (two sides equal), or scalene (all sides different).
use std::io;

fn main() {
    // a
    println!("Enter the size of side A:");
    let mut input_a: String = String::new();
    io::stdin()
        .read_line(&mut input_a)
        .expect("Error in the entered!");

    // b
    println!("Enter the size of side B:");
    let mut input_b: String = String::new();
    io::stdin()
        .read_line(&mut input_b)
        .expect("Error in the entered!");

    // c
    println!("Enter the size of side C:");
    let mut input_c: String = String::new();
    io::stdin()
        .read_line(&mut input_c)
        .expect("Error in the entered!");

    //
    let a: f32 = input_a.trim().parse().expect("Error converting num A:");
    let b: f32 = input_b.trim().parse().expect("Error converting num B:");
    let c: f32 = input_c.trim().parse().expect("Error converting num C:");

    //
    if a == b && b == c {
        println!("Equilateral");
    } else if a == b || b == c {
        println!("Isosceles");
    } else {
        println!("Scalene");
    }
}

// Write a program that receives a number from 1 to 7 and prints the corresponding day of the week (1 for Sunday, 2 for Monday, etc.), also indicating if it's a "Weekday" or "Weekend."use std::io;
use std::io;

fn main() {
    //
    println!("Enter a number from 1 to 7:");
    let mut input_day: String = String::new();
    io::stdin()
        .read_line(&mut input_day)
        .expect("Error in the entered!");
    let day: u8 = input_day.trim().parse().expect("Error converting day!");

    //
    if day == 1 || day == 7 {
        if day == 1 {
            println!("Sunday - Weekend!");
        } else {
            println!("Saturday - Weekend!");
        }
    } else if day == 2 {
        println!("Monday - Weekday!");
    } else if day == 3 {
        println!("Tuesday - Weekday!");
    } else if day == 4 {
        println!("Wednesday - Weekday!");
    } else if day == 5 {
        println!("Thursday - Weekday!");
    } else if day == 6 {
        println!("Friday - Weekday!");
    } else {
        println!("Error!");
    }
}

/*

Write a program that reads a person's age and classifies them into the following categories:
 Child: 0-12 years
 Teenager: 13-17 years
 Adult: 18-59 years
 Elderly: 60 years or more

*/

//
use std::io;

fn main() {
    //
    println!("Enter you age:");
    let mut input_age: String = String::new();
    io::stdin()
        .read_line(&mut input_age)
        .expect("Error in the entered!");
    let age: u8 = input_age.trim().parse().expect("Error converting age!");

    //
    if age >= 1 && age <= 12 {
        println!("Child!");
    } else if age >= 13 && age <= 17 {
        println!("Teenager!");
    } else if age >= 18 && age <= 59 {
        println!("Adult");
    } else if age >= 60 && age <= 112 {
        println!("Elderly!");
    } else {
        println!("Error: Age > 112!");
    }
}

// Create a program that functions as a calculator. It should receive two numbers and an operator (+, -, *, /). The program should perform the corresponding operation and show the result. Also, handle division by zero.
use std::io;

fn main() {
    //
    println!("Enter the first number:");
    let mut input_first_number: String = String::new();
    io::stdin()
        .read_line(&mut input_first_number)
        .expect("Error receiving first number!");

    //
    println!("Enter the second number:");
    let mut input_second_number: String = String::new();
    io::stdin()
        .read_line(&mut input_second_number)
        .expect("Error receiving first number!");

    //
    println!("Insert the mathematical operator:");
    let mut input_operator: String = String::new();
    io::stdin()
        .read_line(&mut input_operator)
        .expect("Error receiving mathematical operator!");

    //
    let first_number: f32 = input_first_number
        .trim()
        .parse()
        .expect("Error converting first number!");

    //
    let second_number: f32 = input_second_number
        .trim()
        .parse()
        .expect("Error converting second number!");

    //
    let operator: char = input_operator
        .trim()
        .parse()
        .expect("Error converting mathematical operator!");

    //
    if operator == '+' {
        println!(
            "{} {} {} = {}",
            first_number,
            operator,
            second_number,
            first_number + second_number
        );
    } else if operator == '-' {
        println!(
            "{} {} {} = {}",
            first_number,
            operator,
            second_number,
            first_number - second_number
        );
    } else if operator == '*' {
        println!(
            "{} {} {} = {}",
            first_number,
            operator,
            second_number,
            first_number * second_number
        );
    } else if operator == '/' {
        if second_number == 0. {
            println!("[Error] Second number equal 0.");
        } else {
            println!(
                "{} {} {} = {}",
                first_number,
                operator,
                second_number,
                first_number / second_number
            );
        }
    } else {
        println!("[Error] Mathematical operator not allowed!");
    }
}

/*

Develop a program that calculates a discount based on the purchase amount:
 Purchases below R$ 100: no discount.
 Purchases from R$ 100 to R$ 500: 10% discount.
 Purchases above R$ 500: 20% discount.

*/

//
use std::io;

fn main() {
    //
    println!("Enter the value:");
    let mut input_value: String = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Error receiving value!");
    let value: f32 = input_value.trim().parse().expect("Error converting value!");

    //
    if value < 100. {
        println!("Value: {} - No discount!", value);
    } else if value >= 100. && value <= 500. {
        let discount = value - ((value * 10.) / 100.);
        println!("Value: {} - Discount 10%\nNew value: {}", value, discount);
    } else if value > 500. {
        let discount = value - ((value * 20.) / 100.);
        println!("Value: {} - Discount 20%\nNew value: {}", value, discount);
    } else {
        println!("Error!");
    }
}

/*

Write a program that calculates a person's BMI (weight / height²) and classifies the result:
 Below 18.5: Underweight
 18.5 - 24.9: Normal weight
 25.0 - 29.9: Overweight
 30.0 or more: Obesity

*/

//
use std::io;

fn main() {
    const ERROR_INPUT: &str = "Error receiving valeu!";
    const ERROR_CONVERTING: &str = "Error converting value!";

    //
    println!("Enter the Weight:");
    let mut input_weight: String = String::new();
    io::stdin().read_line(&mut input_weight).expect(ERROR_INPUT);
    let weight: f32 = input_weight.trim().parse().expect(ERROR_CONVERTING);

    //
    println!("Enter the Height:");
    let mut input_height: String = String::new();
    io::stdin().read_line(&mut input_height).expect(ERROR_INPUT);
    let height: f32 = input_height.trim().parse().expect(ERROR_CONVERTING);

    //
    let bmi: f32 = weight / (height * height);

    if bmi <= 18.5 {
        println!("Underweight!");
    } else if bmi > 18.5 && bmi <= 24.9 {
        println!("Normal weight!");
    } else if bmi >= 25. && bmi <= 29.0 {
        println!("Overweight!");
    } else if bmi >= 30.0 {
        println!("Obesity!");
    } else {
        println!("Error calculating BMI or invalid range!");
    }
}

// Create a program that asks for a username and a password. If the username is "admin" and the password is "1234", display "Access granted". Otherwise, display "Access denied".
use std::io;

const ERROR_INPUT: &str = "Data entry error!";

fn main() {
    //
    println!("Username:");
    let mut input_username: String = String::new();
    io::stdin()
        .read_line(&mut input_username)
        .expect(ERROR_INPUT);
    let username: &str = input_username.trim();

    //
    println!("Password:");
    let mut input_password: String = String::new();
    io::stdin()
        .read_line(&mut input_password)
        .expect(ERROR_INPUT);
    let password: &str = input_password.trim();

    //
    if username == "admin" && password == "12345" {
        println!("Access Granted!");
    } else {
        println!("Access Denied!");
    }
}

// Create the game "Rock, Paper, Scissors." Ask for the choices of two players and determine the winner based on the classic rules.
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    //
    println!("--- Start ---");
    println!("Player 1");
    println!("Computer");
    println!("");

    //
    println!("-- Select --");
    println!("  Player 1  ");
    println!("");
    println!("[1] Rock");
    println!("[2] Paper");
    println!("[3] Scissors");
    println!("");
    println!("1 | 2 | 3");

    //
    let mut input_player1: String = String::new();
    io::stdin()
        .read_line(&mut input_player1)
        .expect("Data entry error!");
    let mut player1: &str = input_player1.trim();

    //
    if player1 == "1" {
        player1 = "Rock";
    } else if player1 == "2" {
        player1 = "Paper";
    } else {
        player1 = "Scissors";
    }

    //
    println!("");
    println!("-- Select --");
    println!("  Player 2  ");
    println!("");
    println!("-- Raffling --");
    println!("[1] Rock");
    println!("[2] Paper");
    println!("[3] Scissors");
    println!("");

    //
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error")
        .as_nanos();

    let nano_computer = (nanos % 3 + 1) as u8;
    let computer: &str;

    if nano_computer == 1 {
        computer = "Rock";
    } else if nano_computer == 2 {
        computer = "Paper";
    } else {
        computer = "Scissors";
    }

    //
    if player1 == computer {
        println!("Player 1: {} and Computer: {}", player1, computer);
        println!("The game is a draw!");
    } else if player1 == "Rock" {
        if computer == "Paper" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Loser!");
        }
        if computer == "Scissors" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Win!");
        }
    } else if player1 == "Paper" {
        if computer == "Rock" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Win!");
        }
        if computer == "Scissors" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Loser!");
        }
    } else {
        if computer == "Rock" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Loser!");
        }
        if computer == "Paper" {
            println!("Player 1: {} and Computer: {}", player1, computer);
            println!("Player 1: You Win!");
        }
    }
}

// LOOP
// Create a program that uses a loop to count from 1 to 5. For each number, print the counter's value. The loop should stop when the counter reaches 6.
fn main() {
    //
    let mut count_number: u8 = 0;

    //
    loop {
        //
        count_number += 1;
        println!("Count = {}", count_number);
        if count_number >= 5 {
            break;
        }
    }
}

// Define a constant LIMIT with the value 100. Use a loop to add numbers to a mutable variable sum starting from 1. Stop the loop when the value of sum exceeds LIMIT and print the final value of sum.
const LIMIT: u8 = 100;

fn main() {
    //
    let mut start_sum: u8 = 1;

    //
    loop {
        println!("{}", start_sum);

        if start_sum >= LIMIT {
            break;
        }

        start_sum += start_sum;
    }
}

// Write a loop that repeatedly prints the message "I'm stuck in a loop!". Use a counter variable and an if statement to break the loop after the 3rd iteration.
fn main() {
    //
    let mut count_num: u8 = 0;

    //
    loop {
        //
        count_num += 1;

        if count_num >= 4 {
            break;
        }

        println!("I'm stuck in a loop!");
    }
}

// Use a loop to iterate from 1 to 10. Inside the loop, check if the current number is even or odd using the modulo operator (%). Print the formatted result, such as "The number 3 is odd.". The loop should stop after the number 10.
fn main() {
    //
    let mut count_number: u8 = 0;

    loop {
        count_number += 1;

        if count_number % 2 == 0 {
            println!("The number {} is even.", count_number);
        } else {
            println!("The number {} is odd.", count_number);
        }

        if count_number >= 10 {
            break;
        }
    }
}

// Initialize a mutable variable of type f32 with 0.0. In a loop, add 0.5 to this variable in each iteration. Print the value at each step and stop the loop when the value is greater than or equal to 5.0.
fn main() {
    //
    let mut count_number: f32 = 0.;

    //
    loop {
        //
        count_number += 0.5;
        println!("{}", count_number);

        if count_number >= 5. {
            break;
        }
    }
}

/*

Create a turn-based game between player and monster:
  - HP: 100
  - Attack: 10
  - Defense: 10

- Player Attack and Monster Attack: Double damage for both
- Player Attack and Monster Defend: Player takes half damage and monster takes no damage
- Player Defend and Monster Attack: Player takes no damage and monster takes half damage
- Player Defend and Monster Defend: Both block

Using only loop, if/else/else if and rand (SystemTime, UNIX_EPOCH)

*/

//
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const DAMAGE: i8 = 10;

fn main() {
    //
    let mut health_player: i8 = 100;
    let mut health_monster: i8 = 100;

    //
    let mut round = 0;

    //
    println!("--- Start Game ---");
    println!("Player vs Monster");
    println!("");

    //
    loop {
        //
        round += 1;

        println!("---- Statistics ----");
        println!(
            "Player:\n HP: {}\n Damage: 10\n Defense: 10\n",
            health_player
        );
        println!(
            "Monster:\n HP: {}\n Damage: 10\n Defense: 10\n",
            health_monster
        );

        println!("--- Player ---");
        println!("[1] Attack!\n[2] Defense\n");

        // player 1
        let mut input_select_player: String = String::new();
        io::stdin()
            .read_line(&mut input_select_player)
            .expect("Error entered data!");
        let select_player: i8 = input_select_player
            .trim()
            .parse()
            .expect("Error converting data!");

        // monster
        let input_select_monster = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error time!")
            .as_nanos();

        // random
        let select_monster = (input_select_monster % 2 + 1) as i8;

        if select_player == 1 && select_monster == 1 {
            health_player = health_player - (DAMAGE * 2);
            health_monster = health_monster - (DAMAGE * 2);
            println!("---- Round {} ----", round);
            println!("Player: Attack\nMonster: Attack\n");
            println!("*** SUPER DAMAGE! ***\n");
        } else if select_player == 1 && select_monster == 2 {
            health_player = health_player - (DAMAGE / 2);
            println!("---- Round {} ----", round);
            println!("Player: Attack\nMonster: Defend\n");
            println!("*** PLAYER BLOCK! ***\n");
        } else if select_player == 2 && select_monster == 1 {
            health_monster = health_monster - (DAMAGE / 2);
            println!("---- Round {} ----", round);
            println!("Player: Defend\nMonster: Attack\n");
            println!("*** MONSTER BLOCK! ***\n");
        } else if select_player == 2 && select_monster == 2 {
            health_player = health_player - (DAMAGE * 2);
            health_monster = health_monster - (DAMAGE * 2);
            println!("---- Round {} ----", round);
            println!("Player: Defend\nMonster: Defend\n");
            println!("*** SUPER BLOCK! ***\n");
        }

        //
        if health_player <= 0 {
            println!("---- Statistics ----");
            println!(
                "Player:\n HP: {}\n Damage: 10\n Defense: 10\n",
                health_player
            );
            println!(
                "Monster:\n HP: {}\n Damage: 10\n Defense: 10\n",
                health_monster
            );
            println!("--- Player ---\n*** YOU LOSE! ***");
            break;
        }
        if health_monster <= 0 {
            println!("---- Statistics ----");
            println!(
                "Player:\n HP: {}\n Damage: 10\n Defense: 10\n",
                health_player
            );
            println!(
                "Monster:\n HP: {}\n Damage: 10\n Defense: 10\n",
                health_monster
            );
            println!("--- Player ---\n*** YOU WIN! ***");
            break;
        }
    }
}

/*

Create a counter that starts from 1 and goes onwards. Inside a loop, check the following conditions:
  If the number is divisible by both 3 and 5, print "FizzBuzz".
  If it's only divisible by 3, print "Fizz".
  If it's only divisible by 5, print "Buzz".
  Otherwise, print the number.
  Break the loop when the counter reaches 100.

*/

//
fn main() {
    //
    println!("Count 1 .. 100!");
    let mut count_number: u8 = 0;

    //
    loop {
        //
        count_number += 1;

        if count_number % 3 == 0 && count_number % 5 == 0 {
            println!("FizzBuzz");
        } else if count_number % 3 == 0 {
            println!("Fizz");
        } else if count_number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count_number);
        }

        if count_number > 100 {
            break;
        }
    }
}

// Start with capital = 1000.0 and goal = 2000.0. Each "year" (loop iteration), the capital increases by 7% (capital *= 1.07;). The loop should count how many years it takes for the capital to reach or exceed the goal. When the goal is reached, the loop should break, returning the number of years. Print the result.
fn main() {
    //
    let mut cap: f32 = 1000.;
    let mut year: u8 = 0;

    loop {
        //
        cap *= 1.07;
        year += 1;
        println!("Cap = {} and Year = {}", cap, year);

        if cap >= 2000. {
            println!("Cap. = {}", cap);
            println!("Year = {}", year);
            break;
        }
    }
}

// Create a loop that starts with the character 'a' and prints each character up to 'f'. Use a mutable variable of type char and increment it. To increment, you can convert the char to u8, add 1, and convert back to char. Stop when the character is greater than 'f'.
fn main() {
    //
    let mut input_char: char = 'a';
    println!("-- Start --");

    //
    loop {
        //
        println!("Char: {}", input_char);
        let char_init = input_char as u8;
        let char_next = char_init + 1;
        input_char = char_next as char;

        if input_char > 'f' {
            break;
        }
    }
}

// Declare a mutable tuple let mut data = (0, false);. Create a loop that increments the first element of the tuple by 1 in each iteration. When the first element reaches the value 5, change the second element to true and break the loop. Print the final tuple outside the loop.
fn main() {
    //
    let mut data: (u8, bool) = (0, false);

    loop {
        println!("{:?}", data);
        data.0 += 1;

        //
        if data.0 > 5 {
            data.1 = true;
            println!("{:?}", data);
            break;
        }
    }
}

// Create a tuple named person with the values (Person, 0, 0.0). Then, change its data 3 times using a loop, prompting the user for the name, age, and height, and print the values.
use std::io;
fn main() {
    //
    let mut person: (String, u8, f32) = ("Person".to_string(), 0, 0.);
    println!("{:#?}", person);

    let mut amount: u8 = 0;

    //
    loop {
        //
        println!("Whats your name?");
        let mut input_person: String = String::new();
        io::stdin()
            .read_line(&mut input_person)
            .expect("Error entered data!");
        let name: String = input_person.trim().to_string();

        //
        println!("Whats your age?");
        let mut input_age: String = String::new();
        io::stdin()
            .read_line(&mut input_age)
            .expect("Error entered data!");
        let age: u8 = input_age.trim().parse().expect("Error convert data!");

        //
        println!("Whats your height?");
        let mut input_height: String = String::new();
        io::stdin()
            .read_line(&mut input_height)
            .expect("Error entered data!");
        let height: f32 = input_height.trim().parse().expect("Error convert data!");

        //
        amount += 1;

        //
        person = (name, age, height);
        println!("{:#?}", person);

        if amount >= 3 {
            break;
        }
    }
}

// Use a loop to display the elements of the following array: [u8; 5] = [10, 20, 30, 40, 50]
fn main() {
    //
    let init_array: [u8; 5] = [10, 20, 30, 40, 50];
    let mut i = 0;

    //
    loop {
        //
        if i >= init_array.len() {
            break;
        }

        //
        println!("{}", init_array[i]);
        i += 1;
    }
}

// Use a loop to display the elements in reverse order of the following array: [u8; 5] = [1, 2, 3, 4, 5]
fn main() {
    //
    let init_array: [u8; 5] = [1, 2, 3, 4, 5];
    let mut i = init_array.len() - 1;

    loop {
        //
        if i <= 0 {
            println!("{}", init_array[i]);
            break;
        }

        println!("{}", init_array[i]);
        i -= 1;
    }
}

// Could you create code to print the following four arrays: [u8; 2] = [1, 2];, [f32; 3] = [2.5, 7.3, 9.2];, [char; 4] = ['A', 'B', 'C', 'D'];, and [&str; 5] = ["Ana", "Bruno", "Carla", "Daniel", "Elza"];"
fn main() {
    //
    let init_array_int: [u8; 2] = [1, 2];
    let init_array_float: [f32; 3] = [2.5, 7.3, 9.2];
    let init_array_char: [char; 4] = ['A', 'B', 'C', 'D'];
    let init_array_str: [&str; 5] = ["Ana", "Bruno", "Carla", "Daniel", "Elza"];

    //
    let mut i = 0;
    loop {
        //
        if i >= init_array_int.len() {
            break;
        }

        println!("{}", init_array_int[i]);
        i += 1;
    }
    println!("");

    //
    i = 0;
    loop {
        //
        if i >= init_array_float.len() {
            break;
        }

        println!("{}", init_array_float[i]);
        i += 1;
    }
    println!("");

    //
    i = 0;
    loop {
        //
        if i >= init_array_char.len() {
            break;
        }

        println!("{}", init_array_char[i]);
        i += 1;
    }
    println!("");

    //
    i = 0;
    loop {
        //
        if i >= init_array_str.len() {
            break;
        }

        println!("{}", init_array_str[i]);
        i += 1;
    }
}

// Create a code that sums the data in an array using a loop, here is an array: [u8; 4] = [1, 2, 4, 5]"
fn main() {
    //
    let init_array: [u8; 4] = [1, 2, 4, 5];

    //
    let mut i = 0;
    let mut sum = 0;

    loop {
        if i >= init_array.len() {
            break;
        }

        println!("Values = {:?}", init_array[i]);
        sum = sum + init_array[i];
        i += 1;
    }
    println!("Sum = {}", sum);
}

// Declare a mutable array of 5 positions of type i32, initialized with zeros: let mut my_array = [0; 5];. Use a loop and an index variable to fill the array with the values [10, 20, 30, 40, 50]. The loop should stop when the array is complete. Print the final array.
fn main() {
    //
    let mut init_array: [i32; 5] = [0; 5];
    let mut i = 0;

    //
    loop {
        //
        if i >= init_array.len() {
            break;
        }

        init_array[i] = (i as i32 + 1) * 10;
        i += 1;
    }

    println!("Array = {:?}", init_array);
}

// Calculate the factorial of 5 using a loop. You will need two mutable variables: one for the counter (from 1 to 5) and another to store the factorial result. Print the final result.
fn main() {
    //
    let mut count: u8 = 1;
    let mut factorial: u8 = 1;

    loop {
        //
        println!("Facto = {}", factorial);

        factorial *= count;
        count += 1;

        if count > 5 {
            break;
        }
    }

    println!("Factorial = {}", factorial);
}

// Start with a variable number equal to 123. In a loop, divide the number by 2 and print the division result and the remainder (%). Continue the loop until the number is less than 1.
fn main() {
    //
    let mut number: f32 = 123.;

    loop {
        //
        let division = number / 2.;
        let remainder = number % 2.;

        //
        number = division;

        //
        println!("123 / 2 = {:.2}\n123 % 2 = {:.2}\n", division, remainder);

        //
        if number < 1. {
            break;
        }
    }
}

// Inside a loop, calculate and print the result of the expression 5+counter∗2, where counter is a variable that starts at 0 and is incremented in each iteration. Use parentheses to change the precedence and calculate (5 + counter)∗2 in a second print. Stop the loop when the counter reaches 5.
fn main() {
    //
    let mut count: u8 = 0;

    //
    loop {
        //
        println!("Count = {} | 5 + count * 2 = {}", count, 5 + count * 2);
        println!("Count = {} | (5 + count) * 2 = {}", count, (5 + count) * 2);

        count += 1;
        if count > 5 {
            break;
        }
    }
}

// Define a constant SECRET_NUMBER: u8 = 42;. Create a loop that simulates guessing attempts. An attempt variable should be incremented in each iteration. Use if/else if/else to print "Too low", "Too high", or "You got it!" when the attempt is equal to SECRET_NUMBER. Break the loop when you guess correctly.
use std::io;

const SECRET_NUMBER: u8 = 42;

fn main() {
    println!("------- Start -------");
    println!("Player 1 VS Computer");
    println!("");
    println!("---------------------");
    println!("You have 3 attempts!");

    let mut attempts: u8 = 0;

    loop {
        println!("\nEnter a number:");
        let mut input_number: String = String::new();
        io::stdin()
            .read_line(&mut input_number)
            .expect("Error entered data!");
        let player_number: u8 = input_number.trim().parse().expect("Error converting data!");

        if player_number > SECRET_NUMBER {
            println!("Too High!");
        } else if player_number < SECRET_NUMBER {
            println!("Too Low!");
        } else {
            println!("\nYou Win!");
            break;
        }

        attempts += 1;
        if attempts == 1 {
            println!("You have 2 attempts remaining!");
        } else if attempts == 2 {
            println!("You have 1 attempt remaining!");
        } else if attempts == 3 {
            println!("YOU LOSE!");
            break;
        }
    }
}

//Count down from 10 to 1. Use a loop and the format! macro to print messages like "Countdown: 10", "Countdown: 9", etc. When the counter reaches 0, print "Launch!" and break the loop.
fn main() {
    //
    println!("-- Start --");
    let mut init_count: i8 = 10;

    //
    loop {
        println!("Countdown: {}", init_count);

        init_count -= 1;
        if init_count < 1 {
            println!("Lauch!");
            break;
        }
    }
}

//Declare a mutable array of 10 positions ([0; 10]). Use a loop with a counter to fill it. If the array index is even, fill it with the index value itself. If it's odd, fill it with twice the index value. Stop the loop when the array is complete.
fn main() {
    //
    let mut init_array: [i8; 10] = [0; 10];
    let mut i = 0;

    loop {
        //
        if i >= init_array.len() {
            break;
        }

        //
        if i % 2 == 0 {
            init_array[i] = i as i8;
        } else {
            init_array[i] = i as i8 * 2;
        }

        //
        println!("Array = {:?}", init_array);
        i += 1;
    }
}
