# Cloudr 🌤️

Cloudr is a Rust library that provides an abstract data structure for storing values without moving them. It offers a flexible and convenient way to store and access data in a cloud-like structure.

## Features ✨

- Store values without moving them 📦
- Insert, retrieve, and remove values based on keys 🔑
- Check for the existence of keys or values ✅
- Iterate over key-value pairs 🔄
- Combine multiple instances of the data cloud 🌐
- Convert into an owned `FxHashMap` 🔄
- Iteration and mapping utilities 🚀

## Installation 🚀

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
cloudr = "0.1.0"
```

## Usage 🛠️

Here's a simple example demonstrating the basic usage of the `DataCloud`:
```rust
use cloudr::DataCloud;

fn main() {
    let data: DataCloud<String, i32> = DataCloud::new();
    
    data.insert("x".to_string(), &42);
    data.insert("y".to_string(), &123);
    
    if let Some(value) = data.get(&"x".to_string()) {
        println!("Value of x: {}", value);
    }
    
    data.remove(&"y".to_string());
    
    println!("Data cloud: {}", data);
}
```

For more detailed examples and documentation, please refer to the API documentation. 📚

## Contributing 🤝

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request. ❤️

## License 📝

This project is licensed under the MIT license. 📜