# Answers - Variables Constants and Data Types

---

01 - Declare a variable positive_only of type u32 with the value 100. Declare another variable can_be_negative of type i32 with the value -100. Print both. Now, try to assign -5 to the positive_only variable. What happens when you try to compile and why?

```rust
fn main() {
    //
    let positive_only: u32 = 100;
    println!("{:?}", positive_only);

    //
    let can_be_negative: i32 = -100;
    println!("{:?}", can_be_negative);

    // error: attempt to compute `u8::MAX + 1_u8`, which would overflow
    // let positive_only: u32 = -5;
    // println!("{:?}", positive_only);
}
```

02 - Write Rust code to declare a variable max_u8 of type u8 and assign it the largest possible value for this type. Print this value. Then, in your code, try to assign max_u8 + 1 to a new u8 variable. What happens during compilation or execution (especially in debug vs release mode, if you know)?

```rust
fn main() {
    //
    let max_u8: u8 = u8::MAX;
    println!("{:?}", max_u8);

    // error: attempt to compute `u8::MAX + 1_u8`, which would overflow
    // let max_u8: u8 = u8::MAX + 1;
    // println!("{:?}");
}
```

03 - Declare a variable named world_population of type u64 and assign it the value 7800000000. Then, print the variable's value formatted with thousand separators (research how to do this if necessary, or just print the number).

```rust
fn main() {
    //
    let world_population: u64 = 7_800_000_000;
    println!("World Population: {world_population:?}");
}
```
  
04 - Suppose you are modeling a system that uses extremely large unique IDs that will never be negative. Declare a variable super_large_id of type u128 and assign it the value 250000000000000000000000000000000000000 (25 followed by 36 zeros). Print this variable.

```rust
fn main() {
    //
    let super_large_id: u128 = 250_000_000_000_000_000_000_000_000_000_000_000_000;
    println!("{:?}", super_large_id);     
}

/*
    Debugging: The code below attempts to assign a value that exceeds the limit of i32. Correct the code by choosing a larger i type that can accommodate the value 2_200_000_000.

    fn main() {
        let large_positive: i32 = 2_200_000_000; // Exceeds the limit of i32
        println!("{}", large_positive);
    }
*/

// the literal `2_200_000_000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`
```

05 - You need to store the number of video views, which can reach billions but will never be negative. Choose the most appropriate unsigned type (u32 or u64). Declare a variable video_views with this type, assign 2500000000 to it, and print.

```rust
fn main() {
    //
    let video_views: u64 = 2_500_000_000;
    println!("{:?}", video_views);
}
```

06 - Declare two variables, min_val_i8 and max_val_i8, both of type i8. Assign them the smallest an largest possible value, respectively, for the i8 type. Print both values.

```rust
fn main() {
    //
    let min_val_i8: i8 = i8::MIN;
    println!("{:?}", min_val_i8);

    //
    let max_val_i8: i8 = i8::MAX;
    println!("{:?}", max_val_i8);
}
```

07 - Declare a variable named current_altitude_change of type i16 to represent an altitude change in meters. Assign it the value -350 (a descent of 350 meters). Print this value.

```rust
fn main() {
    //
    let current_altitude_change: i16 = -350;
    println!("Altitude: {:?} meters", current_altitude_change);
}
```

08 - You are processing financial transactions where the values can be very large and represent both credits and debits (in cents, to avoid floating-point issues). Declare a variable transaction_value_cents of type i64 and assign it a value like -12345678900 (representing a debit of over 123million). Print it.

```rust
fn main() {
    //
    let transaction_value_cents: i64 = -123_456_789_000;
    println!("Debit = {:.3} million", transaction_value_cents);
}

/*
    Debugging: The code below attempts to assign a value that exceeds the limit of i32. Correct the code by choosing a larger i type that can accommodate the value 2_200_000_000.

    fn main() {
        let large_positive: i32 = 2_200_000_000; // Exceeds the limit of i32 println!("{}", large_positive);
    }
*/

// the literal `2_200_000_000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`
```

09 - Declare a variable value_a of type i8 with 120. Declare value_b of type u8 with 120. Now, try to declare value_c of type i8 with -10 and value_d of type u8 attempting to assign -10 (literally). What happens to value_d during compilation? Explain.

```rust
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
```

10 - Write a code snippet that declares two variables score_team_a and score_team_b (both i32). Assign values to them and calculate the score_difference (which can be negative). Print the difference.

```rust
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
```

11 - Declare a variable ratio_f32 of type f32 with the value 2.0 / 7.0. Declare another variable ratio_f64 of type f64 with the same value 2.0 / 7.0. Print both using println!("{:.18}", variable_name); to show several decimal places. Compare the printed results.

```rust
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
```

12 - Declare a variable precise_measurement of type f64 with the value 123.456789123456. Print it.

```rust
fn main() {
    //
    let precise_measurement: f64 = 123.456_789_123_456;
    println!("{:?}", precise_measurement);
}
```

13 - Declare a variable gravity_force with the value 9.80665 without specifying the type, letting Rust infer it. Then, use std::any::type_name_of_val(&gravity_force) to print the inferred type. What is printed and why?

```rust
fn main() {
    //
    let gravity_force = 9.80665;
    println!("{:?}", std::any::type_name_of_val(&gravity_force));

    //
    let gravity_force: f32 = 9.80665;
    println!("{:?}", gravity_force);
    println!("{:?}", std::any::type_name_of_val(&gravity_force));
}
```

14 - Create a variable result_f64 of type f64 and assign it the expression 0.1 + 0.2. Print the result with at least 17 decimal places. Is the result exactly 0.3? Write a small code snippet to check if result_f64 == 0.3 and print "Equal" or "Different".

```rust
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
```

15 - In an embedded system with limited memory, you need to store a sensor reading that ranges from -10.0 to +10.0 with two decimal places of precision. Declare a variable sensor_reading using f32 and assign 7.89. Print it. Justify why f32 might be suitable here.

```rust
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
```

16 - Declare a character char_ascii = 'Z'; and a Unicode character char_unicode = 'Ω'; (Greek letter Omega). Use std::mem::size_of_val(&char_ascii) and std::mem::size_of_val(&char_unicode) to print the size in bytes of each. What do you observe?

```rust
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
```

17 - Declare a variable my_initial of type char and assign it the first letter of your name. Print it.

```rust
fn main() {
    //
    let my_ini: char = 'f';
    let my_init: char = 'o';
    let my_initi: char = 'j';

    //
    println!("{}{}{}", my_ini, my_init, my_initi);
}
```

18 - Declare three char variables: emoji_char with '😊', math_symbol with '∑', and arrow_char with '→'. Print all of them.

```rust
fn main() {
    //
    let emoji_char: char = '😊';
    let math_symbol: char = '∑';
    let arrow_char: char = '→';

    //
    println!("{}\n{}\n{}", emoji_char, math_symbol, arrow_char);
}
```

19 - Try to declare a char variable with more than one character, for example: let not_a_char: char = 'ab';. What happens when you try to compile?

```rust
fn main() {
    // syntax Error: Literal must be one character long
    let not_a_char : char = 'ab';
}
```

20 - Declare a variable is_file_loaded and assign it true. Declare has_errors and assign false. Print both. Then, try to assign the integer 1 to a boolean variable. What does the Rust compiler say?

```rust
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
```

21 - Declare a variable user_is_premium of type bool. Use an if to print "Full access granted!" if true, or "Consider going premium!" if false. Test with both values.

```rust
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
```

22 - Declare is_online = true and has_new_messages = false. Create a variable should_notify that is true if is_online AND has_new_messages are both true. Print should_notify. Then, change has_new_messages to true and recalculate/print should_notify.

```rust
fn main() {
    //
    let is_online: bool = true;
    let mut has_new_messages: bool = false;

    //
    let should_notify = is_online && has_new_messages;
    println!("Should notify (initial): {}", should_notify);

    //
    has_new_messages = true;
    let should_notify = is_online && has_new_messages;
    println!("Should notify (after new message): {}", should_notify);
}
```

23 - Create a tuple record that contains a name (&str), an age (u8), and a passing grade (f32). For example: ("Maria", 22, 7.5). Print the entire tuple.

```rust
fn main() {
    //
    let record: (&str, u8, f32) = ("Maria", 22, 7.5);
    println!("{:#?}", record);
}
```

24 - Create a tuple called server_response that contains an HTTP status code (u16) and a response message (&str), such as (404, "Not Found"). Print the tuple.

```rust
fn main() {
    //
    let server_response: (u16, &str) = (404, "Not Found");
    println!("{:?}", server_response);
}
```

25 - Given the tuple let product_info = ("Laptop XPTO", 1250.99, 15); (name, price, quantity in stock), access and print the product price and the quantity in stock using tuple indexing (e.g., product_info.1).

```rust
fn main() {
    //
    let product_info: (&str, f32, u16) = ("Laptop XPTO", 1.250_000, 15);
    let (name, price, quantity_and_stock) = product_info;

    //
    println!(
        "Product: {} \nQuantity and Stock: {}",
        product_info.0, product_info.2
    );

    //
    println!("{:#?}", product_info);

    //
    println!(
        "Product: {} \nPrice: {:.3} \nQuantity and Stock: {}",
        name, price, quantity_and_stock
    );
}
```

26 - Given the tuple let color_rgb = (255, 128, 0); (representing Orange), destructure it into the variables red, green, and blue. Print each variable separately.

```rust
fn main() {
    //
    let color_rgb: (u8, u8, u8) = (255, 128, 0);
    let (red, green, blue) = color_rgb;

    //
    println!("Red: {} \nGreen: {} \nBlue: {}", red, green, blue);
}
```

27 - Create a tuple complex_data = ('X', vec![1,2,3], ("nested", true));. Print it. What does this demonstrate about the types a tuple can contain?

```rust
fn main() {
    //
    let complex_data = ('X', vec![1, 2, 3], ("nested", true));
    println!("{:?}", complex_data);
    println!("{:?}", std::any::type_name_of_val(&complex_data));
}
```

28 - Declare a tuple api_result with a boolean indicating success, a u64 for an ID, and a String for a message. Ex: (true, 1234567890, String::from("Successful operation")). Print using {:#?}.

```rust
fn main() {
    //
    let api_result: (bool, u64, String) = (true, 1234567890, String::from("Successful operation"));
    println!("{:#?}", api_result);
}
```

29 - Declare a tuple let config = ("localhost", 8080);. Try to modify the second element to 8081 (e.g., config.1 = 8081;). What happens when compiling? Now, redeclare it as let mut config = ("localhost", 8080); and try the same modification. Print config.

```rust
fn main() {
    // let config: (&str, u16) = ("localhost", 8080);
    //println!("{:?}", config);
    //config.1 = 8180; // cannot assign to `config.1`, as `config` is not declared as mutable  cannot assign

    //
    let mut config: (&str, u16) = ("localhost", 8080);
    println!("{:?}", config);

    config.1 = 8180;
    println!("{:?}", config);
}
```

30 - Declare a mutable tuple player_stats to store name (&str), score (i32), and lives (u8). Initialize with ("Hero", 0, 3). Then, modify the score to 1500 and the lives to 2. Print the updated tuple.

```rust
fn main() {
    //
    let mut player_stats: (&str, i32, u8) = ("Hero", 0, 3);
    println!("{:#?}", player_stats);

    //
    player_stats.1 = 1500;
    player_stats.2 = 2;
    println!("{:#?}", player_stats);
}
```

31 - Create a mutable tuple let mut point = (10.0, 20.0);. Modify the first element to 15.5 and the second to 25.0 using index access syntax. Imprint the tuple.

```rust
fn main() {
    //
    let mut point: (f32, f32) = (10., 20.);
    println!("{:#?}", point);

    //
    point = (15.5, 25.);
    println!("{:#?}", point);
}
```

32 - Create a mutable tuple file_details containing file name (String), size (u64), and whether it's editable (bool). Initialize it. Then, modify the file name (appending "_v2" to the original name) and change the editable status. Print the tuple.

```rust
fn main () {
    //
    let file_details: (String, u64, bool) = (String::from("test.txt"), 127, true);
    println!("{:#?}", file_details);

    //
    let mut file_details_v2 = file_details;
    file_details_v2.0 = String::from("teste_v2.txt");
    file_details_v2.1 = 256;

    //
    println!("{:#?}", file_details_v2);
}
```

33 - Create an array temperatures that stores the following f32 temperature readings: [22.5, 23.1, 21.9, 22.8, 23.5]. Print the entire array. Try adding a &str to this array. What happens?

```rust
fn main () {
    //
    let temperatures: [f32; 5] = [22.5, 23.1, 21.9, 22.8, 23.5];
    println!("{:#?}", temperatures);

    // expected `f32`, found `&str`
    let array_test: [f32; 2] = [22.5, "Test"];
    println!("{:#?}", array_test);
}
```

34 - Declare an array months containing the names of the first three months of the year as string slices. Print the name of the second month (remember zero-basedindexing).

```rust
fn main () {
    //
    let months: [&str; 3] = ["January", "February", "March"];
    println!("{:#?}", months);

    //
    println!("{:#?}", months[1])
}
```

35 - Declare an array powers_of_two of 6 elements of type u32. Initialize it with the values [1, 2, 4, 8, 16, 32]. Print the last element of the array using indexing.

```rust
fn main() {
    //
    let powers_of_two: [u32; 6] = [1, 2, 4, 8, 16, 32];
    println!("{:?}, {:?}, {:?}", powers_of_two[3], powers_of_two[4], powers_of_two[5]);
}
```

36 - Declare an array grades with 5 u8 scores all initialized to the value 0 using the repetition syntax (e.g., [0; 5]). Print the array and its size using .len().

```rust
fn main () {
    //
    let grades: [u8; 5] = [0; 5];
    println!("{:?}", grades);

    //
    println!("{:?}", grades.len());
}
```

37 - Declare an array let fixed_scores = [100, 90, 80];. Try to modify the second element to 95 (e.g., fixed_scores[1] = 95;). What happens? Now, declare it as let mut fixed_scores = [100, 90, 80];, make the modification, and print it.

```rust
fn main () {
    //
    let fixed_scores = [100, 90, 80];
    println!("{:?}", fixed_scores);

    // fixed_scores[1] = 95;
    // println!("{:?}", fixed_scores);

    //
    let mut fixed_scores = [10, 30, 60];
    println!("{:?}", fixed_scores);

    fixed_scores[1] = 0;
    println!("{:?}", fixed_scores);
}
```

38 - Declare a mutable array inventory_counts of 4 elements of type u16, initialized with [10, 25, 5, 30]. Modify the count of the first item to 12 and the thirdto 8. Print the updated array.

```rust
fn main () {
    //
    let mut inventory_counts: [u8; 4] = [10, 25, 5, 30];

    //
    inventory_counts[0] = 12;
    inventory_counts[3] = 8;

    //
    println!("{:?}", inventory_counts);
}
```

39 - Create a mutable array active_services: [bool; 3] initialized as [true, false, true]. Modify the second service to true and the last to false. Print the array.

```rust
fn main () {
    //
    let mut active_services:[bool; 3] = [true, false, true];
    println!("{:?}", active_services);

    //
    active_services[1] = true;
    println!("{:?}", active_services);

    //
    println!("{:?}, {:?}", active_services[0], active_services[2]);
}
```

40 - Create a mutable array pixel_colors: [[u8; 3]; 2] to represent two pixels, each with R, G, B components. Initialize it as [[255, 0, 0], [0, 255, 0]] (one red pixel, one green). Modify the first pixel to be blue ([0, 0, 255]) and the second to be yellow ([255, 255, 0]). Print the pixel array.

```rust
fn main () {
    //
    let mut pixel_colors: [[u8; 3]; 3] = [[255, 0, 0], [0, 255, 0], [0, 0, 255]];
    println!("{:?}", pixel_colors);

    //
    pixel_colors[0][1] = 255;
    println!("{:?}", pixel_colors);
}
```
