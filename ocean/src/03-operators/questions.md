# Questions - Arithmetic Operators, Assignment Operators, String and &str, Tuples, Array, Scope, Precedence, Comparison and Logical

---
01 - Write a program that declares two integer variables, 'a' with the value 15 and 'b' with the value 7, and prints their sum.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ba79725b1daeebae28b1f8eff37f3ef6)

02 - Create code that subtracts the value 3 from a variable called 'total' that initially equals 10, and displays the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ce6ae9b7cf5ba78de17480e4ee126b7c)

03 - Develop a program that multiplies two variables, 'x' equals 6 and 'y' equals 8, and shows the product.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=70e75c196a74104dadd8c0b447c8b12e)

04 - Implement code that divides the number 20 by 5 and prints the quotient.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3310a0de603425918e6581d5b3721e2b)

05 - Write a program that calculates the remainder of the division of 23 by 4 and prints the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d4c5c8dc43b052c33a57bd45d9d11a77)

06 - Declare two variables num1 (12) and num2 (5) and print the result of their sum directly within the println! macro.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=076ca45ba4bdd549e14c4559e5800ab3)

07 - Create an immutable variable called 'value' that is the sum of 9 and 2, and print its value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a9bc729353358ead87d8369738d196de)

08 - Declare two variables 'first' (7) and 'second' (3) and print the result of their multiplication.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=633d7e426d95211f924fcd808a332ac3)

09 - Declare two variables 'dividend' (18) and 'divisor' (3) and store the result of the division in a variable called 'result', printing it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=30f37a965c65eb08fc76b3751ccfd95e)

10 - Declare two variables n1 (25) and n2 (7). Calculate and print the remainder of the division of n1 by n2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9880d7191e4c260e6a7cbd0f7c053b50)

11 - Declare a mutable variable named counter with an initial value of 5. Increment it by 3 using the addition assignment operator and print the new value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=7a8d2f1a19fccdcb70eeec71f7ab0fde)

12 - Create a mutable variable pontuacao initialized with 20. Decrement it by 5 using the subtraction assignment operator and display the final value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f4009497a1a81deb2ad17f177ea2b4d2)

13 - Declare a mutable variable fator with the value 2. Multiply it by 4 using the multiplication assignment operator and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=82ac11dee9d48d3431f72878863f0d61)

14 - Initialize a mutable variable size with 30. Divide it by 6 using the division assignment operator and show the resulting value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=36e65843521161312365a71e69c8851f)

15 - Declare a mutable variable mod with the value 17. Calculate the remainder of the division by 5 using the remainder assignment operator and print the value.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=804489e2d5f2345e5760616564e55316)

16 - Declare two variables of type &str, part1 with "Hello, " and part2 with "Rust!". Concatenate them and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=19e51491cd2f3f2cbbf21143d21e9bcc)

17 - Create two String variables, s1 with "Language" and s2 with "Rust.". Concatenate them and print the resulting string.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=43490545ac9f27c31901a3d3bbdede66)

18 - Declare a variable of type &str called prefix with the value "Number: ". Create a variable of type String called number_str with the value "42". Concatenate them and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9d65adac0d80ea1a47b4a7ac17defe50)

18 - Create a String variable called initial_message with "Welcome ". Declare a &str variable called name with "User". Concatenate them and print the complete message.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=2295091d660ea2b9685cade8d09d6ccb)

19 - Declare two variables greeting of type String with "Good " and period of type &str with "day!". Concatenate them and print the complete greeting.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=238b5e930de685cb06ca20e55c3576d3)

20 - Create a tuple with three u8: (10, 20, 30). Access the first and second elements, add them and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=85d4b821bcfc7f29c3750cc50b189c57)

21 - Declare a tuple of two i32s. Initialize it with the values (5, -2). Print the multiplication of these two values.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e0722f9856fd6420bdf854db51bda447)

22 - Create a nested tuple: ((1, 2), (3, 4)). Add the first element of the first tuple to the second element of the second tuple and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=843964881eecfdfb804108086bf46102)

23 - Declare two tuples, t1 with (2, 5) and t2 with (8, 1). Add all the elements of the two tuples and print the total.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8a22e77af9d0b7a18ac6012a7390804b)

24 - Create a tuple with an f64 and an i32. Print each element separately.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=49be9b658f06dea9adab5077cb43985c)

25 - Declare an array of 5 i32 with the values [1, 3, 5, 7, 9]. Print the sum of the first and last elements.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e3342896a1b63feee95a878111a8daf9)

26 - Create an array of 3 u16. Initialize it with the values [10, 20, 30]. Print the product of all elements.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=02446a71b12a6262b64e2f2faabd4b94)

27 - Declare a 2x2 multidimensional array with integers. Sum all elements and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=7b8c08f95d85e9bc254de6a437464b5b)

28 - Create two arrays of two f32. Sum the corresponding elements of the two arrays and print the results of the sums.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=fd34ce0713943da9e41688fbef7424c8)

29 - Declare two arrays a1 with [4, 2] and a2 with [3, 6]. Calculate and print the sum of all elements from both arrays.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=1db2d55f7dc5fc32a20faed41d71452b)

30 - Declare a global variable with value 5. Inside a block {} declare another variable with the same name and value 10. Print the value of both variables (the global and the block variable) outside and inside the block.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c933168e0df2e0c4546b1d3cf5ef1699)

31 - Declare a variable x with value 1. Create an inner block where you declare a variable y with value 2. Inside the block, print the sum of x and y. Outside the block, try to print y (what will happen?).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d66b4b4a5e33c872d866cda67c0d0e59)

32 - Declare a variable level1 with value 10. Open a new block and declare a variable level2 with value 20. Inside this block, create another block and declare level3 with value 30. Print the sum of level1 and level3 inside the innermost block.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=95cf3f933752f8804abfc3e217d158d3)

33 - Declare a main variable with value 100. Create a block where you declare a variable with the same name and assign the value of main plus 50 to it. Print the value of the variable inside the block and outside the block.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=01c3267516f1340e4a042108ca1c94ea)

34 - Given the array values with [2, 3], write code that calculates and prints the result of values[0]+values[1]∗values[1].

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=4fc38c10f2e9f8af4b059f1d979c4f7b)

35 - Using the same array values, write code that calculates and prints the result of (values[0]+values[1])∗values[1].

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=2ce6bcd5b4a70b7bd1efaf2da5fd145f)

36 - Still with values, calculate and print values[0]∗values[1]/values[1]%values[1].

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=efeb476b82c5936b385067cc7ec87415)

37 - Calculate and print values[0]+values[1]−values[0]+values[0]∗values[0].

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=619694774b3dfb7d2417801e69a205cb)

38 - Calculate and print ((values[0]+values[1])−(values[0]+values[0])∗values[0]).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a731a6979c96bbc53019ed0fa1d4211b)

39 - Declare two integer variables, num1 with value 10 and num2 with value 10. Write code that prints if num1 is equal to num2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6987411834f9118f7707e0bc7d943035)

40 - Declare two floating-point variables, f1 with 3.14 and f2 with 2.71. Write code that prints if f1 is different from f2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a530ea305ad2927dd433e555d4ffb182)

41 - Declare two integer variables, age1 with 25 and age2 with 30. Write code that prints if age1 is greater than age2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a07816f8ff04db0f7df863401becda56)

42 - Declare two integer variables, point1 with 5 and point2 with 8. Write code that prints if point1 is less than point2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=7364e64dfa1a5b22f1db1e57328f12d5)

43 - Declare two integer variables, grade1 with 7 and grade2 with 7. Write code that prints if grade1 is greater than or equal to grade2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c48944d03da1a64548da872e0dff14e9)

44 - Declare two integer variables, height1 with 170 and height2 with 165. Write code that prints if height1 is less than or equal to height2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0520bbba63e5a59593f891d267f13360)

45 - Declare two boolean variables, true1 with true and true2 with true. Compare if they are equal and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9ebbbc8008bd6823590bbf1c736797ab)

46 - Declare an integer variable x with 5 and another y with 10. Check if x is not equal to y and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e5f8a0a1835258198a69e7bd6c7f4158)

47 - Declare a variable temp1 with 22.5 and temp2 with 20.0. Check if temp1 is greater than temp2 and print.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d6b51da7eba11107f6adbd7fb619c605)

48 - Declare a variable count1 with 100 and count2 with 99. Check if count1 is less than count2 and print.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8e6d65a541d1156ea3dd5c4269b341df)

49 - Declare two boolean variables, cond1 as true and cond2 as true. Print the result of the logical AND (&&) operation between them.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=924c7f88d30b8c1acbe05c2d0d6f4d78)

50 - Declare flag1 as false and flag2 as false. Print the result of the logical AND operation between them.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=da4bcfbc81c52e6935eec8c759da69de)

51 - Declare active as true and allowed as false. Print the result of active && allowed.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=21a1c0c52c595a3399ed1c296e339e39)

52 - Declare has_permission as false and is_admin as true. Print the result of has_permission && is_admin.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3e17966d1ed8415ace580de87215a10d)

53 - Declare two boolean variables, option1 as true and option2 as true. Print the result of the logical OR (||) operation between them.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=9eff9a52575a2fd3e9151daaac2b8376)

54 - Declare error1 as false and error2 as false. Print the result of error1 || error2.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8af7eb80343f05502db36ec440b062c7)

55 - Declare connected as true and has_data as false. Print the result of connected || has_data.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b5def9ce05b5f8a5930c93a572a882b6)

56 - Declare success as false and failure as true. Print the result of success || failure.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8919aa4a8524f66baba5766ef9ab0764)

57 - Declare a boolean variable state as true. Print the result of the logical NOT (!) of state.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f32b6d58be42cb535c0a3b281c64d39a)

58 - Declare a boolean variable invalid as false. Print the result of !invalid.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=34e20811739c8c8a827baf970810a7ab)

59 - Declare two u8 variables, val1 with 8 and val2 with 3. Calculate the sum, subtract 1 from the result, and print.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f1744e9392a330122cb5c76c885d29e0)

60 - Create a tuple (u32, u32) with values (5, 10). Multiply the two elements and assign the result to a new variable, then print it.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=679eca9783df8cd9e363b431b44f26ad)

61 - Declare an array [i32; 3] with values [2, 4, 6]. Divide each element by 2 (using division assignment if applicable) and print the resulting array.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=376d7b8b7c42cc5208fc6e923e7c7240)

62 - Declare two variables n_a as 7 and n_b as 4. Calculate the remainder of the division of n_a by n_b and print if the remainder is equal to 3.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=bac43e837ffd593cee60160406896560)

63 - Create a String "Number: ". Concatenate with the result of the sum of two u8s (e.g., 5 + 7) converted to String. Print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b884f435db8a5e41ca8d7990e2608510)

64 - Declare a tuple (f64, f64) with (2.5, 3.5). Sum the elements and print the result formatted to two decimal places.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c5c8f9a3d00d3ae7d5ce8d56bd4ed91d)

65 - Declare an array [u16; 2] with [100, 200]. Multiply the first element by 2 and the second by 3, then sum the results and print.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=69132860ad5bc19dca7f98bbf5680b72)

66 - Declare a mutable variable message with "&str" "Start". Append to it the String " -> End". Print the final value of message.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=5bc36877ec0c3c3d971220e0650843cd)

67 - Create a tuple (i8, i8, i8) with (1, 2, 3). Sum all three numbers and print the result.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6b1cc95dbb15e089aceb517bb8388b84)

68 - Declare an outer_level variable with 5. Inside a block, declare inner_level with 10. Print the sum of both inside the block. Outside the block, try to print inner_level (what happens?).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f74d8944f2f63befc98254f2362534e1)

69 - Given an array data with [3, 2], calculate and print the result of data[0]+data[1]∗5. Then, calculate and print (data[0]+data[1])∗5.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=2be4208055486cc4d4bd3ee059a6fc07)

70 - Given an array calc_values with [10, 3, 2], calculate and print calc_values[0]/calc_values[1]+calc_values[2]. Then, calculate and print calc_values[0]/(calc_values[1]+calc_values[2]).

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3cdb6800da4f9c29b9b9101772b46918)

71 - Declare a global_factor variable with 2. Inside a block, declare an array nums with [1, 2, 3]. Sum the array elements and multiply the result by global_factor, printing the final result inside the block.

  [Playground!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=523af4bb8f140db9108f9a7a6162fea4)
