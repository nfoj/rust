# Control Flow Constructs - if, else, else if, loop, while, for 

- if
```rust
// if
fn main () {
    //
    let num: u8 = 2;

    //
    if num > 1 {
        println!("Num > 1!");
    };
}

// if + var 
fn main() {
    //
    let code: bool = true;

    //
    let message = if code {
        println!("Code = true!");
    };
    
    // output = ()
    println!("{:?}", message);
}

// if + var + logical
fn main() {
    //
    let code: u8;
    code = 9;

    //
    let result = if code > 1 && code % 2 == 0 {
      println!("Code = {:?} even", code);
    };
}
```

- else
```rust
// if and else
fn main () {
    //
    let num: u8 = 5;

    //
    if num >= 6 {
        println!("Num > or = 6!");
    } else {
        println!("Num <= 5!");
    };
}

// if and else + var = expression
fn main () {
    //
    let code: bool = true;

    //
    let message = if code {
        println!("Code == true!");
    } else {
        println!("Code != true!");
    };

    // output = ()
    println!("{:?}", message);
}

// if and else + var = declaration
fn main () {
    //
    let code: bool = false;

    //
    let message = if code {
        "Code == true!"
    } else {
        "Code != true!"
    };

    // output = "Code != true!"
    println!("{:?}", message);
}

// if and else + var + logical
fn main() {
    //
    let num: u8;
    num = 24;
    
    //
    let result: String = if num > 20 || num <= 30 {
      String::from("Temp: > 20 or <= 30!")
    } else {
          String::from("Temp < 20 or >= 30!")  
    };
    
    //
    println!("{}\nTemp: {:?}", result, num);
}
```

- if, else and else if
```rust
// if ...
fn main () {
    //
    let temp: i8 = 34;
    
    //
    if temp < 0 {
        println!("Temp == 0°C");
    }
    else if temp >= 0 && temp < 10 {
        println!("Temp >= 0°C and < 10°C");
    }
    else if temp >= 10 && temp < 20 {
        println!("Temp >= 10°C and < 20°C");
    }
    else if temp >= 20 && temp < 30 {
        println!("Temp >= 20°C and < 30°C");
    } 
    else {
        println!("Temp > 30°C!");
    };
}
```

- loop and break
```rust
//
fn main() {
    loop {
        println!("Hello, world!");
        break;
    }
}

//
fn main () {
    //
    let mut count = 0;

    loop {
        prinlnt!("Cont: {}", count);
        count += 1;

        if count == 5 {
            break;
        }
    }

    println!("End of loop!");
}

// loop + breck + continue
fn main() {
    //
    let mut count = 0;
    let max_value = 4;

    loop {
        //
        count += 1;

        if count % 2 == 0 {
            println!("Count equal even! Value = {}", count);
            //continue;
        }
        if count > max_value {
            break;
        }

        println!("---")
    }
}

// + continue
fn main() {
    //
    let mut count = 0;
    let max_value = 4;

    loop {
        //
        count += 1;

        if count % 2 == 0 {
            println!("Count equal even! Value = {}", count);
            continue;
        }
        if count > max_value {
            break;
        }

        println!("---")
    }
}
```

- while
```rust
// while
fn main() {
      //
      let mut count = 0;

      while count <= 5 {
          println!("Count = {}", count);
          count += 1;
      }
  }

// while + continue
fn main() {
    //
    let mut count = 0;
    let max_value = 4;

    while count <= max_value {
        //
        count += 1;

        if count % 2 == 0 {
            println!("Count is even! Value = {}", count);
            continue;
        }
        println!("---");
    }
}    
```

- for
```rust
//
for element in collection {
    
}

// i
// 0 - 4
for i in 0..5 {
    println!("{}", i);
}

// 0 - 5
for i in 0..=5 {
    println!("{}", i);
}

// 4 - 0
for i in (0..5).rev() {
    println!("{}", i);
}

// 5 - 0
for i in (0..=5).rev() {
    println!("{}", i);
}

//
fn main() {
    //
    let name: &str = "Rust";
    for i in name.chars() {
        println!("{}", i);
    }
}

// 
let arr = ["apple", "banana", "orange", "guava"];
for i in arr {
    println!("{:#?}", i)
}

//
let arr: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

for i in arr {
    for j in i {
        print!("{}", j);
    }
    println!();
}
```

Note:
- loop: for infinite repetitions, stopping only with break;
- while: for repetitions as long as a condition is true;
- for: for iterating over collections or a defined number of times.
