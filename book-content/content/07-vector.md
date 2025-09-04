# Vector

A vector in Rust is a homogeneous array that can be started empty, so values can be added, modified, or removed.

- inicialized

```rust
//
let my_vector_u8: Vec<u8> = vec![0, 1, 2, 3, 4];
println!("Vec<u8> = {:#?}", my_vector_u8);

//
let my_vector_str: Vec<&str> = vec!["a", "b", "c", "d"];
println!("Vec<&str> = {:?}", my_vector_str);

//
let my_vector_f32 = vec![0.1, 0.2];
println!("Vec<f32> = {:?}", my_vector_f32);
```

- index

```rust
//
let vector_index: Vec<char> = vec!['a', 'b', 'c'];
println!("i[0] = {:?}", vector_index[0]);
println!("i[1] = {:?}", vector_index[1]);
println!("i[2] = {:?}", vector_index[2]);

// let + index
let vector_index_alph: Vec<&str> = vec!["x", "y"];
let vector_alph: &str = vector_index_alph[1];
println!("Vector Alph = {}", vector_alph);
```

- mut

```rust
//
let mut vec: Vec<char> = vec!['a', 'b'];
println!("Vec<char> = {:?}", vec);

vec[0] = 'b';
println!("Vec<char> = {:?}", vec);

vec[1] = 'c';
println!("Vec<char> = {:?}", vec);
```

- push

```rust
//
let mut vector_push_add: Vec<u8> = Vec::new();
vector_push_add.push(0);
println!("Vector Add = {:?}", vector_push_add);

vector_push_add.push(1);
println!("Vector Add = {:?}", vector_push_add);

vector_push_add.push(2);
println!("Vector Add = {:?}", vector_push_add);
```

- insert

```rust
// vector.insert[index, element]
let mut vector_add: Vec<u8> = Vec::new();

vector_add.insert(0, 0);
println!("Vector Insert = {:?}", vector_add);

vector_add.insert(1, 1);
println!("Vector Insert = {:?}", vector_add);

vector_add.insert(0, 2);
println!("Vector Insert = {:?}", vector_add);

vector_add.insert(1, 3);
println!("Vector Insert = {:?}", vector_add);
```

- pop

```rust
//
let mut vector_pop_delete: Vec<char> = vec!['a', 'b', 'c', 'd'];
println!("Vector Pop = {:#?}", vector_pop_delete);

vector_pop_delete.pop();
println!("Vector Pop = {:#?}", vector_pop_delete);

vector_pop_delete.pop();
println!("Vector Pop = {:#?}", vector_pop_delete);
```

- remove

```rust
// vector.remove[index]
let mut vector_delete: Vec<&str> = vec!["A", "B", "C", "D", "E"];
println!("Vector Remove = {:?}", vector_delete);

vector_delete.remove(1);
println!("Vector Remove = {:?}", vector_delete);

vector_delete.remove(3);
println!("Vector Remove = {:?}", vector_delete);
```

- is_empty()

```rust
let mut vec: Vec<u8> = vec![];
println!("Vec.is_empty() = {:?}", vec.is_empty());

//
vec.push(24);
println!("\nVec.is_empty() = {:?}", vec);
println!("Vec.is_empty() = {:?}", vec.is_empty());
```

- len()

```rust
//
let vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
println!("Vec.len() = {:?}", vec.len());

//
let vec: Vec<&str> = vec!["Yellow", "Blue", "Green", "Red", "Gray", "Pink"];
println!("Vec.len() = {:?}", vec.len());
```

- repeat

```rust
//
let vector_repeat: Vec<u8> = vec![0; 5];
println!("Vector Repeat = {:#?}", vector_repeat);
```

- clear()

```rust
//
let mut vec_alph: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
println!("Vec_alph:Vec<cha> = {:?}", vec_alph);

//
vec_alph.clear();
println!("\nVec_alph:Vec<cha> = {:?}", vec_alph);
```
