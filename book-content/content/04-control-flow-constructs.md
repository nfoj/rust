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
for i in 

```


loop

Use loop quando você precisa de um loop infinito que continua executando até que você explicitamente o pare com uma palavra-chave break.

    Exemplo de uso: Jogos que precisam de um "game loop" contínuo, ou quando você está esperando por um evento específico para acontecer antes de sair.

    Quando usar: Para repetições contínuas e indefinidas que só terminam com uma condição interna.

while

Use while quando você precisa que o loop continue enquanto uma condição específica for verdadeira.

    Exemplo de uso: Contagem regressiva, esperando por um arquivo ser criado, ou processando itens em uma fila até que ela esteja vazia.

    Quando usar: Para repetições que dependem de uma condição booleana externa que pode mudar a cada iteração.

for

Use for quando você quer iterar sobre uma coleção de itens (como um array, vetor ou range) ou um determinado número de vezes.

    Exemplo de uso: Percorrer todos os elementos de uma lista, iterar por um range de números (ex: de 1 a 10), ou processar cada linha de um arquivo.

    Quando usar: Para iteração previsível sobre uma sequência de itens, tornando o código mais conciso e menos propenso a erros de índice.

Em resumo:

    loop: Para repetições infinitas, parando apenas com break.

    while: Para repetições enquanto uma condição for verdadeira.

    for: Para iterar sobre coleções ou um número definido de vezes.
