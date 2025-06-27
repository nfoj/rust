# Control Flow Constructs - if, else, else if, loop, while, for break and continue

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

// loop + breck + return
fn main() {
    //
    println!("-- Start --");
    let mut count: u8 = 0;

    let result = loop {
        //
        count += 1;

        if count == 7 {
            break count;
        }

        println!("Count: {}", count);
    };

    println!("Result: {}", result);
}


```
