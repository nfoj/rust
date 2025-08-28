# Methods

- .len(): used to get the size of a collection.

```rust
let method_len: &str = "Method Len";
println!("{:?}", method_len.len());
```

- .chars(): creates an iterator that returns each character

```rust
let method_chars: &str = "Method Chars";
println!("{:?}", method_chars.chars());
```

- iter(): 

```rust
let method_iter: [&str; 3] = ["M", "N", "O"];
  for i in method_iter.iter() {
    println!("{:?}", i);
}
```

- enumerate(): 

```rust
let method_iter_enumerate: [&str; 3] = ["A", "B", "C"];
for (i, e) in method_iter_enumerate.iter().enumerate() {
    println!("Index = {:?} and Element = {:?}", i, e);
}
```

- .contains()

```rust
let text_contain: &str = "Hello, world!";
println!("Contain: 'o'?= {}", text_contain.contains("o"));
```

- .reverse()

```rust
let mut list_reverse: [u8; 5] = [1, 2, 3, 4, 5];
list_reverse.reverse();
println!("{:#?}", list_reverse);
```

- .trim()

```rust
//
let text_trim: &str = "   A B C D E   ";
println!("{:?}", text_trim);

// .trim() 
println!("{:?}", text_trim.trim());

// .trim_start()
println!("{:?}", text_trim.trim_start());

// .trim_end()
println!("{:?}", text_trim.trim_end());
```

- .replace

```rust
//
let text_replace: &str = "   A B C D E   ";
let mut text = text_replace.replace("A", "X");

// more replace
let text_more_replace = text_replace
  .replace("B", "2")
  .replace("C", "Y")
  .replace("E", "7");
println!("{:?}", text_more_replace);
```

- .lowercase() and .uppercase()

```rust
// .to_lowercase()
let text_lower: &str = "   A 1 C 2 E   ";
println!("{:?}", text_lower.to_lowercase());

// .to_uppercase()
let text_upper: &str = "   3 b 4 d 6   ";
println!("{:?}", text_upper.to_uppercase());
```

- .repeat() 

```rust
let text_repeat: &str = "   A 1 C 2 E   ";
let text = text_repeat.repeat(6);
println!("{:?}", text);
```

- .pow

```rust
// .pow
let num_pow: i8 = 2;
println!("{:?}", num_pow.pow(4));

// .powi
let num_pow: f32 = 2.;
println!("{:?}", num_pow.powi(4));

// .powf
let num_pow: f32 = 2.;
println!("{:?}", num_pow.powf(4.));
```

- .sqrt() and .isqrt()

```rust
// sqrt > f32 and f64
let num_sqrt: f32 = 25.;
println!("Sqrt >  {} = {}", num_sqrt, num_sqrt.sqrt());

// isqrt > u8, i8, ...
let num_isqrt: u8 = 25;
println!("Isqrt > {} = {}", num_isqrt, num_isqrt.isqrt());
```

- .sort()

```rust
let mut list_sort: Vec<u8> = vec![9, 2, 7, 3, 8, 4, 3, 3];
list_sort.sort();
println!("{:#?}", list_sort);
```

- .max() and .min() 

```rust
// max
let num_max: [u8; 4] = [1, 2, 4, 6];
println!("{:?}", num_max.iter().max());

// min
let num_min: [u8; 4] = [1, 2, 4, 1];
println!("{:?}", num_min.iter().min());

// .max(.min)
println!("{:?}", num_max.max(num_min));
```

