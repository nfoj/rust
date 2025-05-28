# Data Structures - Intergers, Floats, Boolean, Characters, Tuple and Array

- u
```rust
// u8 = 0 - 255
let number_u8: u8 = 255;
println!("{}", number_u8);

// u16 = 0 - 65.535
let number_u16: u16 = 255;
println!("{}", number_u16);

// u32 = 0 - 4.294.967.295
let number_u32: u32 = 255;
println!("{}", number_u32);

// u64 = 0 - 18.446.744.073.709.551.615
let number_u64: u64 = 255;
println!("{}", number_u64);

// u128 = 0 - ...
let number_u128: u128 = 255;
println!("{}", number_u128);
```

- i
```rust
// i8 = -128 - 127
let number_i8: i8 =  127;
println!("{}", number_i8);

// i16 = -32.768 - 32.767 (-32_768 até 32_767)
let number_i16: i16 =  127;
println!("{}", number_i16);

// i32 = -2.147.483.648 - 2.147.483.647
let number_i32: i32 =  127;
println!("{}", number_i32);

// i64 = -9.223.372.036.854.775.808 - 9.223.372.036.854.775.807
let number_i64: i64 =  127;
println!("{}", number_i64);

// i128 =  ... and ...
let number_i128: i128 =  127;
println!("{}", number_i128);
```

- f (32 and 64)
```rust
// f32 = approx. -3.4e+38 a +3.4e+38
let number_f32: f32 =  179.76;
println!("{}", number_f32);

// f64 = approx. -1.8e+308 a +1.8e+308
let number_f64: f64 =  179.76;
println!("{}", number_f64);
```

- char
```rust
let character: char = 'a';
println!("{}", character);
let symbol: char = ' '
println!("{}", symbol);
```

- bool
```rust
let checked: bool = true;
println!("The data was checked? {}", checked);
let checked: bool = false;
println!("The data was checked? {}", checked);
```

- tuples
```rust
let data_types = ("a", 1);
println!("{:?}", data_types);
      
//
let data_types = ("b", 2);
println!("{:#?}", data_types);
  
//
let data_types : (char, u8);
data_types = ('c', 3);
println!("{:?}", data_types);

//
let (name, num) : (&str, i8);
(name, num) = ("Alice", 4);
println!("{} and {}", name, num);

//
let (letter, yes) : (char, bool) = ('d', true);
let test =  (letter, yes);
println!("{:?}", test);

//
let person = ("Gregor", 64, 1.82);
let (x, y, z) = person;
println!("My name is {x}, i'm {y} years old and my height is {z}");
```

- tuples and mut
```rust
//
let mut person = ("Roberto", 90, 1.98);
println!("{:#?}", person);

person.0 = "Moana";
person.1 = 58;
person.2 = 1.62;
println!("{:#?}", person);

//
let mut person = ("Helena", 67, 2.10);
println!("{:?}", person);

person.0 = "Arle";
person.1 = 64;
person.2 = 1.71;
println!("{:?}", person);

//
let (name, weight, height) = person;
println!("Name: {} \nWeight: {} \nHeight: {}", name, weight, height);
```

- array
```rust
//
let list: [u8; 4] = [1, 2, 3 , 4];
println!("{:?}", list);

//
let list: [u8; 4] = [1, 2, 3 , 4];
println!("{:#?}", list);

//
let list = ['a', 'b', 'c' , 'd'];
println!("{:?}", list);

//
let list : [&str; 3];
list = ["ab", "cd", "de"];
println!("{:?}", list);

//
let [a, b, c] : [&str; 3] = ["xy", "yz", "zw"]; 
let list = [a, b, c];
println!("{:?}", list);

//
let list = ["12", "23", "34"];
let [a, b, c] = list;
println!("{:?} {:?} {:?}", a, b, c);
```

- array and mut
```rust
// 
let mut number : [i32; 5] = [10, 20, 30, 40, 50];
println!("{:?}", number);

number[0] = 100;
number[2] = 300;
println!("{:?}", number);

//
let mut names: [&str; 3] = ["Alice", "Bob", "Charlie"];
println!("{:?}", names);

names[0] = "Ane";
names[1] = "Bren";
println!("{:?}", names);

//
let mut scores : [u8; 4];
scores = [80, 40, 90, 73];
let [s1, s2, s3, s4] = scores;
println!("{}, {}, {} and {}", s1, s2, s3, s4);

scores[2] = 85;
println!("{:?}", scores);
println!("{}", s3);
```
