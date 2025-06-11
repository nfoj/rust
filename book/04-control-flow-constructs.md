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
```
