# Questions - Data Structures

---
01 - Declare a variable positive_only of type u32 with the value 100. Declare another variable can_be_negative of type i32 with the value -100. Print both. Now, try to assign -5 to the positive_only variable. What happens when you try to compile and why?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=60ee836e6ff396e3ac5a69bbb0e00927)

02 - Write Rust code to declare a variable max_u8 of type u8 and assign it the largest possible value for this type. Print this value. Then, in your code, try to assign max_u8 + 1 to a new u8 variable. What happens during compilation or execution (especially in debug vs. release mode, if you know)?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6274ca481f8f0fd52a451f05a82a61b4)
    
03 - Declare a variable named world_population of type u64 and assign it the value 7800000000. Then, print the variable's value formatted with thousand separators (research how to do this if necessary, or just print the number).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=832da0159cb82e6a068efd9b923b24f0)
    
04 - Suppose you are modeling a system that uses extremely large unique IDs that will never be negative. Declare a variable super_large_id of type u128 and assign it the value 250000000000000000000000000000000000000 (25 followed by 36 zeros). Print this variable.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3a8862c672f6d93355f4e6b2f78258f2)

05 - You need to store the number of video views, which can reach billions but will never be negative. Choose the most appropriate unsigned type (u32 or u64). Declare a variable video_views with this type, assign 2500000000 to it, and print.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=133cef87057e36e26a2df2b2b0b91765)

06 - Declare two variables, min_val_i8 and max_val_i8, both of type i8. Assign them the smallest an largest possible value, respectively, for the i8 type. Print both values.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ba79725b1daeebae28b1f8eff37f3ef6)

07 - Declare a variable named current_altitude_change of type i16 to represent an altitude change in meters. Assign it the value -350 (a descent of 350 meters). Print this value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e2634beea3fd8c7958ad78107fdccf0c)
  
08 - You are processing financial transactions where the values can be very large and represent both credits and debits (in cents, to avoid floating-point issues). Declare a variable transaction_value_cents of type i64 and assign it a value like -12345678900 (representing a debit of over 123million). Print it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3fbd70f6487531637264b9ee3280ea1d)
  
09 - Declare a variable value_a of type i8 with 120. Declare value_b of type u8 with 120. Now, try to declare value_c of type i8 with -10 and value_d of type u8 attempting to assign -10 (literally). What happens to value_d during compilation? Explain.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d4a06055b3c5af0c8761fed7b89da839)
  
10 - Write a code snippet that declares two variables score_team_a and score_team_b (both i32). Assign values to them and calculate the score_difference (which can be negative). Print the difference.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=fa61758c7b217653ce8e3963b6b33eb3)
  
11 - Declare a variable ratio_f32 of type f32 with the value 2.0 / 7.0. Declare another variable ratio_f64 of type f64 with the same value 2.0 / 7.0. Print both using println!("{:.18}", variable_name); to show several decimal places. Compare the printed results.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=040c8c98e78c386078bef987a65b03be)
  
12 - Declare a variable precise_measurement of type f64 with the value 123.456789123456. Print it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9ccc624dcf4dcde8ba60cad9cd694932)

13 - Declare a variable gravity_force with the value 9.80665 without specifying the type, letting Rust infer it. Then, use std::any::type_name_of_val(&gravity_force) to print the inferred type. What is printed and why?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=4c0396a98e0a06d4247ea9b5cfe5d34b)

14 - Create a variable result_f64 of type f64 and assign it the expression 0.1 + 0.2. Print the result with at least 17 decimal places. Is the result exactly 0.3? Write a small code snippet to check if result_f64 == 0.3 and print "Equal" or "Different".

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=204dd78761c165d2ef11e616d875b5fb)

15 - In an embedded system with limited memory, you need to store a sensor reading that ranges from -10.0 to +10.0 with two decimal places of precision. Declare a variable sensor_reading using f32 and assign 7.89. Print it. Justify why f32 might be suitable here.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a42032c341e71ba8e776fdec44592f51)
  
16 - Declare a character char_ascii = 'Z'; and a Unicode character char_unicode = 'Ω'; (Greek letter Omega). Use std::mem::size_of_val(&char_ascii) and std::mem::size_of_val(&char_unicode) to print the size in bytes of each. What do you observe?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f9e7bfb04bce6059ef65d775c6a47bed)

17 - Declare a variable my_initial of type char and assign it the first letter of your name. Print it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a5ff5a70e060980ad52a8efa53d4a03b)
  
18 - Declare three char variables: emoji_char with '😊', math_symbol with '∑', and arrow_char with '→'. Print all of them.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=30486e7e3a0fdc86a2ec84adda9a9140)
  
19 - Try to declare a char variable with more than one character, for example: let not_a_char: char = 'ab';. What happens when you try to compile?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=45efd1bdc0da80b32f1543e839e98faa)

20 - Declare a variable is_file_loaded and assign it true. Declare has_errors and assign false. Print both. Then, try to assign the integer 1 to a boolean variable. What does the Rust compiler say?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=df1360079b5d4efb448a2d3cff84f6cd)

21 - Declare a variable user_is_premium of type bool. Use an if to print "Full access granted!" if true, or "Consider going premium!" if false. Test with both values.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d4e1c190dcce6432fa52dc23efc42e8d)

22 - Declare is_online = true and has_new_messages = false. Create a variable should_notify that is true if is_online AND has_new_messages are both true. Print should_notify. Then, change has_new_messages to true and recalculate/print should_notify.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ace1e16773e2f3777430517cf39dc8df)

23 - Create a tuple record that contains a name (&str), an age (u8), and a passing grade (f32). For example: ("Maria", 22, 7.5). Print the entire tuple.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d2aa6a5fe5a2c228a0c4cdf01a986637)

24 - Create a tuple called server_response that contains an HTTP status code (u16) and a response message (&str), such as (404, "Not Found"). Print the tuple.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b2df2d26c5e24946753e76295327c42e)

25 - Given the tuple let product_info = ("Laptop XPTO", 1250.99, 15); (name, price, quantity in stock), access and print the product price and the quantity in stock using tuple indexing (e.g., product_info.1).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9f27f40f83d95ebebb192508b357c23f)

26 - Given the tuple let color_rgb = (255, 128, 0); (representing Orange), destructure it into the variables red, green, and blue. Print each variable separately.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f50c766a98c72e95fbd4aaa7e3d3120a)

27 - Create a tuple complex_data = ('X', vec![1,2,3], ("nested", true));. Print it. What does this demonstrate about the types a tuple can contain?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8f7d0587dddda6e1c6388e64ed55b20a)

28 - Declare a tuple api_result with a boolean indicating success, a u64 for an ID, and a String for a message. Ex: (true, 1234567890, String::from("Successful operation")). Print using {:#?}.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b8880d9ebc342019b093dca67f56f9c2)
    
29 - Declare a tuple let config = ("localhost", 8080);. Try to modify the second element to 8081 (e.g., config.1 = 8081;). What happens when compiling? Now, redeclare it as let mut config = ("localhost", 8080); and try the same modification. Print config.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=12452da20c411cf36530f1ee4e16e5ce)
    
30 - Declare a mutable tuple player_stats to store name (&str), score (i32), and lives (u8). Initialize with ("Hero", 0, 3). Then, modify the score to 1500 and the lives to 2. Print the updated tuple.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=728088262a4b76b297ec133ab582d176)
    
31 - Create a mutable tuple let mut point = (10.0, 20.0);. Modify the first element to 15.5 and the second to 25.0 using index access syntax. Imprint the tuple.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=56d79e590ba12fc230dfda7b382e1444)
    
32 - Create a mutable tuple file_details containing file name (String), size (u64), and whether it's editable (bool). Initialize it. Then, modify the file name (appending "_v2" to the original name) and change the editable status. Print the tuple.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6b5e3c0a5e0c5f98b6c0fb95a097ed66)
    
33 - Create an array temperatures that stores the following f32 temperature readings: [22.5, 23.1, 21.9, 22.8, 23.5]. Print the entire array. Try adding a &str to this array. What happens?

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=69669f162999a4891e2330a098733e81)

34 - Declare an array months containing the names of the first three months of the year as string slices. Print the name of the second month (remember zero-basedindexing).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=53a62f786fc4470c6e2b3dca6917b82e)
    
35 - Declare an array powers_of_two of 6 elements of type u32. Initialize it with the values [1, 2, 4, 8, 16, 32]. Print the last element of the array using indexing.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=092ca765f10e5f1653747f806d02dd5a)
  
36 - Declare an array grades with 5 u8 scores all initialized to the value 0 using the repetition syntax (e.g., [0; 5]). Print the array and its size using .len().

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=29c28a00e5a862741f1b53cc520fb50e)
  
37 - Declare an array let fixed_scores = [100, 90, 80];. Try to modify the second element to 95 (e.g., fixed_scores[1] = 95;). What happens? Now, declare it as let mut fixed_scores = [100, 90, 80];, make the modification, and print it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=af8c4e2b513d6360eaa771836c3675c1)

38 - Declare a mutable array inventory_counts of 4 elements of type u16, initialized with [10, 25, 5, 30]. Modify the count of the first item to 12 and the thirdto 8. Print the updated array.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6a12c6b14485e4d9bdef556132eee612)

39 - Create a mutable array active_services: [bool; 3] initialized as [true, false, true]. Modify the second service to true and the last to false. Print the array.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8be14543c295e538007025d9270d9005)

40 - Create a mutable array pixel_colors: [[u8; 3]; 2] to represent two pixels, each with R, G, B components. Initialize it as [[255, 0, 0], [0, 255, 0]] (one red pixel, one green). Modify the first pixel to be blue ([0, 0, 255]) and the second to be yellow ([255, 255, 0]). Print the pixel array.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a7f882b0c2e13c1e6c15c743714d21e1)
