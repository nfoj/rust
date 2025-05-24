// Questions - Data Structures

// Declare a variable positive_only of type u32 with the value 100. Declare another variable can_be_negative of type i32 with the value -100. Print both. Now, try to assign -5 to the positive_only variable. What happens when you try to compile and why?
fn main() {
    //
    let positive_only: u32 = 100;
    println!("{:?}", positive_only);

    //
    let can_be_negative: i32 = -100;
    println!("{:?}", can_be_negative);

    // error: attempt to compute `u8::MAX + 1_u8`, which would overflow
    let positive_only: u32 = -5;
    println!("{:?}", positive_only);
}

// Write Rust code to declare a variable max_u8 of type u8 and assign it the largest possible value for this type. Print this value. Then, in your code, try to assign max_u8 + 1 to a new u8 variable. What happens during compilation or execution (especially in debug vs. release mode, if you know)?
fn main() {
    //
    let max_u8: u8 = u8::MAX;
    println!("{:?}", max_u8);

    // error: attempt to compute `u8::MAX + 1_u8`, which would overflow
    let max_u8: u8 = u8::MAX + 1;
    println!("{:?}");
}

// Declare a variable named world_population of type u64 and assign it the value 7800000000. Then, print the variable's value formatted with thousand separators (research how to do this if necessary, or just print the number).
//
fn main() {
    //
    let world_population: u64 = 7_800_000_000;
    println!("{:?}", world_population);
}
// output: 7800000000

// dp
// [dependecies]
// thousands = "0.2.0"
fn main() {
    //
    let world_population: u64 = 7_800_000_000;
    let formatted_population = thousands_with_separator(world_population, '.');
    println!("{:?}", formatted_population);
}
// output: 7.800.000.000

// Suppose you are modeling a system that uses extremely large unique IDs that will never be negative. Declare a variable super_large_id of type u128 and assign it the value 250000000000000000000000000000000000000 (25 followed by 36 zeros). Print this variable.
fn main() {
    //
    let super_large_id: u128 = 250_000_000_000_000_000_000_000_000_000_000_000_000;
    println!("{:?}", super_large_id);
}

/* Debugging: The code below causes an error. Identify the error, explain why it occurs, and correct the code to make it work, while maintaining the intention of using a small type if the value allows.

fn main() {
    let small_number: u8 = 260;
    println!("{}", small_number);
}

*/

// literal out of range for `u8`  the literal `260` does not fit into the type `u8` whose range is `0..= 255
fn main() {
    let small_number: u16 = 260;
    println!("{}", small_number);
}

// You need to store the number of video views, which can reach billions but will never be negative. Choose the most appropriate unsigned type (u32 or u64). Declare a variable video_views with this type, assign 2500000000 to it, and print.
fn main() {
    //
    let video_views: u64 = 2_500_000_000;
    println!("{:?}", video_views);
}

// Declare two variables, min_val_i8 and max_val_i8, both of type i8. Assign them the smallest and largest possible value, respectively, for the i8 type. Print both values.
fn main() {
    //
    let min_val_i8: i8 = i8::MIN;
    println!("{:?}", min_val_i8);

    //
    let max_val_i8: i8 = i8::MAX;
    println!("{:?}", max_val_i8);
}

// Declare a variable named current_altitude_change of type i16 to represent an altitude change in meters. Assign it the value -350 (a descent of 350 meters). Print this value.
fn main() {
    //
    let current_altitude_change: i16 = -350;
    println!("Altitude: {:?} meters", current_altitude_change);
}

// You are processing financial transactions where the values can be very large and represent both credits and debits (in cents, to avoid floating-point issues). Declare a variable transaction_value_cents of type i64 and assign it a value like -12345678900 (representing a debit of over 123 million). Print it.
fn main() {
    //
    let transaction_value_cents: i64 = -123_456_789_000;
    println!("Debit = {:.3} million", transaction_value_cents);
}

/* Debugging: The code below attempts to assign a value that exceeds the limit of i32. Correct the code by choosing a larger i type that can accommodate the value 2_200_000_000.

fn main() {
    let large_positive: i32 = 2_200_000_000; // Exceeds the limit of i32
    println!("{}", large_positive);
}
*/
// the literal `2_200_000_000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`

// Declare a variable value_a of type i8 with 120. Declare value_b of type u8 with 120. Now, try to declare value_c of type i8 with -10 and value_d of type u8 attempting to assign -10 (literally). What happens to value_d during compilation? Explain.
fn main() {
    //
    let value_a: i8 = 120;
    println!("{:?}", value_a);

    //
    let value_b: u8 = 120;
    println!("{:?}", value_b);

    //
    let value_c: i8 = -10;
    println!("{:?}", value_c);

    //
    let value_d = -10;
    println!("{:?}", value_d);
}

// Write a code snippet that declares two variables score_team_a and score_team_b (both i32). Assign values to them and calculate the score_difference (which can be negative). Print the difference.
fn main() {
    //
    let score_team_a: i32;
    let score_team_b: i32;

    //
    score_team_a = 20;
    score_team_b = 24;

    //
    let score_difference = score_team_a - score_team_b;
    println!("Score difference: {:?}", score_difference);
}

// Declare a variable ratio_f32 of type f32 with the value 2.0 / 7.0. Declare another variable ratio_f64 of type f64 with the same value 2.0 / 7.0. Print both using println!("{:.18}", variable_name); to show several decimal places. Compare the printed results.
fn main() {
    //
    let ratio_f32: f32 = 2.0 / 7.0;
    let ratio_f64: f64 = 2.0 / 7.0;

    //
    println!("{:?}", ratio_f32);
    println!("{:?}", ratio_f64);
}
// 0.2857143
// 0.2857142857142857

fn main() {
    //
    let ratio_f32: f32 = 2.0 / 7.0;
    let ratio_f64: f64 = 2.0 / 7.0;

    //
    println!("{:.18}", ratio_f32);
    println!("{:.18}", ratio_f64);
}
// 0.285714298486709595
// 0.285714285714285698

// Declare a variable precise_measurement of type f64 with the value 123.456789123456. Print it.
fn main() {
    //
    let precise_measurement: f64 = 123.456_789_123_456;
    println!("{:?}", precise_measurement);
}

// Declare a variable gravity_force with the value 9.80665 without specifying the type, letting Rust infer it. Then, use std::any::type_name_of_val(&gravity_force) to print the inferred type. What is printed and why?
fn main() {
    //
    let gravity_force = 9.80665;
    println!("{:?}", std::any::type_name_of_val(&gravity_force));

    //
    let gravity_force: f32 = 9.80665;
    println!("{:?}", gravity_force);
    println!("{:?}", std::any::type_name_of_val(&gravity_force));
}

// Create a variable result_f64 of type f64 and assign it the expression 0.1 + 0.2. Print the result with at least 17 decimal places. Is the result exactly 0.3? Write a small code snippet to check if result_f64 == 0.3 and print "Equal" or "Different".
fn main() {
    //
    let result_f64: f64 = 0.1 + 0.2;
    println!("{:.18}", result_f64);

    //
    if (result_f64 - 0.3).abs() < 1e-10 {
        println!("Equal");
    } else {
        println!("Different");
    }

    //
    if result_f64 == 0.3 {
        println!("Equal");
    } else {
        println!("Different");
    }
}

// In an embedded system with limited memory, you need to store a sensor reading that ranges from -10.0 to +10.0 with two decimal places of precision. Declare a variable sensor_reading using f32 and assign 7.89. Print it. Justify why f32 might be suitable here.
fn main() {
    //
    let limited_memory: f32 = 10.00;
    println!("{:?}", limited_memory);

    //
    let limited_memory: f32 = -10.00;
    println!("{:?}", limited_memory);

    //
    let sensor_reading: f32 = 7.89;
    println!("{:?}", sensor_reading);
}

// Declare a character char_ascii = 'Z'; and a Unicode character char_unicode = 'Ω'; (Greek letter Omega). Use std::mem::size_of_val(&char_ascii) and std::mem::size_of_val(&char_unicode) to print the size in bytes of each. What do you observe?
fn main() {
    //
    let char_ascii: char = 'Z';
    println!("{:?}", char_ascii);

    //
    //let char_unicode: char = ' ';
    //println!("{:?}", char_unicode);

    //
    let char_unicode: char = 'Ω';
    println!("{:?}", char_unicode);

    //
    println!("{:?}", std::mem::size_of_val(&char_ascii));
    println!("{:?}", std::mem::size_of_val(&char_unicode));
}

// Declare a variable my_initial of type char and assign it the first letter of your name. Print it.
fn main() {
    //
    let my_ini: char = 'f';
    let my_init: char = 'o';
    let my_initi: char = 'j';

    //
    println!("{}{}{}", my_ini, my_init, my_initi);
}

// Declare three char variables: emoji_char with '😊', math_symbol with '∑', and arrow_char with '→'. Print all of them.
fn main() {
    //
    let emoji_char: char = '😊';
    let math_symbol: char = '∑';
    let arrow_char: char = '→';

    //
    println!("{}\n{}\n{}", emoji_char, math_symbol, arrow_char);
}

// Try to declare a char variable with more than one character, for example: let not_a_char: char = 'ab';. What happens when you try to compile?
fn main() {
      
    // syntax Error: Literal must be one character long
    let not_a_char : char = 'ab';    
}

// Declare a variable is_file_loaded and assign it true. Declare has_errors and assign false. Print both. Then, try to assign the integer 1 to a boolean variable. What does the Rust compiler say?
fn main() {
    //
    let is_fiele_loaded: bool = true;
    println!("{:?}", is_fiele_loaded);

    //
    let has_errors: bool = false;
    println!("{:?}", has_errors);

    // mismatched types  expected `bool`, found integer
    // let assing: bool = 1;
    // println!("{:?}", assing);
}

// Declare a variable user_is_premium of type bool. Use an if to print "Full access granted!" if true, or "Consider going premium!" if false. Test with both values.
fn main() {
    //
    let user_is_premium: bool = true;

    //
    if user_is_premium == true {
        println!("Full access granted!");
    } else {
        println!("Consider going premium!");
    }

    //
    let user_is_premium: bool = false;

    //
    if user_is_premium == true {
        println!("Full access granted!");
    } else {
        println!("Consider going premium!");
    }
}


// Write a simple function can_vote(age: u8, is_citizen: bool) -> bool that returns true if age is 18 or greater AND is_citizen is true. Call it with some values and print the results.

// Declare is_online = true and has_new_messages = false. Create a variable should_notify that is true if is_online AND has_new_messages are both true. Print should_notify. Then, change has_new_messages to true and recalculate/print should_notify.

// Create a tuple record that contains a name (&str), an age (u8), and a passing grade (f32). For example: ("Maria", 22, 7.5). Print the entire tuple using {:?}.

// Create a tuple called server_response that contains an HTTP status code (u16) and a response message (&str), such as (404, "Not Found"). Print the tuple.

// Given the tuple let product_info = ("Laptop XPTO", 1250.99, 15); (name, price, quantity in stock), access and print the product price and the quantity in stock using tuple indexing (e.g., product_info.1).

// Given the tuple let color_rgb = (255, 128, 0); (representing Orange), destructure it into the variables red, green, and blue. Print each variable separately.

// Create a tuple complex_data = ('X', vec![1,2,3], ("nested", true));. Print it. What does this demonstrate about the types a tuple can contain?

// Declare a tuple api_result with a boolean indicating success, a u64 for an ID, and a String for a message. Ex: (true, 1234567890, String::from("Successful operation")). Print using {:#?}.

// Declare a tuple let config = ("localhost", 8080);. Try to modify the second element to 8081 (e.g., config.1 = 8081;). What happens when compiling? Now, redeclare it as let mut config = ("localhost", 8080); and try the same modification. Print config.

// Declare a mutable tuple player_stats to store name (&str), score (i32), and lives (u8). Initialize with ("Hero", 0, 3). Then, modify the score to 1500 and the lives to 2. Print the updated tuple.

// Create a mutable tuple let mut point = (10.0, 20.0);. Modify the first element to 15.5 and the second to 25.0 using index access syntax. Imprint the tuple.

// Create a mutable tuple file_details containing file name (String), size (u64), and whether it's editable (bool). Initialize it. Then, modify the file name (appending "_v2" to the original name) and change the editable status. Print the tuple.

// Create an array temperatures that stores the following f32 temperature readings: [22.5, 23.1, 21.9, 22.8, 23.5]. Print the entire array. Try adding a &str to this array. What happens?

// Declare an array months containing the names of the first three months of the year as string slices. Print the name of the second month (remember zero-basedindexing).

// Try to create an array let mixed_data = [1, "dois", 3.0];. What does the Rust compiler report? How does this differ from a tuple with elements of different types?

/* The following code has an error related to array size and initializers. Correct it in two different ways (one by adjusting the size, the other by adjusting the initializers) and show both solutions.

fn main() {
    // Original error:
    let some_numbers: [i32; 4] = [10, 20, 30];
    println!("{:?}", some_numbers);
}
*/

// Declare an array powers_of_two of 6 elements of type u32. Initialize it with the values [1, 2, 4, 8, 16, 32]. Print the last element of the array using indexing.

// Declare an array grades with 5 u8 scores all initialized to the value 0 using the repetition syntax (e.g., [0; 5]). Print the array and its size using .len().

// Declare an array let fixed_scores = [100, 90, 80];. Try to modify the second element to 95 (e.g., fixed_scores[1] = 95;). What happens? Now, declare it as let mut fixed_scores = [100, 90, 80];, make the modification, and print it.

// Declare a mutable array inventory_counts of 4 elements of type u16, initialized with [10, 25, 5, 30]. Modify the count of the first item to 12 and the thirdto 8. Print the updated array.

// Create a mutable array active_services: [bool; 3] initialized as [true, false, true]. Modify the second service to true and the last to false. Print the array.

// Given the array let mut values: [i32; 6] = [5, -2, 10, -8, 0, 3];, write a for loop that iterates over this array and, for each element, if it's negative, replace it with its absolute value (e.g., -2 becomes 2, -8 becomes 8). Print the modified array.

// Create a mutable array pixel_colors: [[u8; 3]; 2] to represent two pixels, each with R, G, B components. Initialize it as [[255, 0, 0], [0, 255, 0]] (one red pixel, one green). Modify the first pixel to be blue ([0, 0, 255]) and the second to be yellow ([255, 255, 0]). Print the pixel array.
