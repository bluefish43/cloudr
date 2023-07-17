//!! # Cloudr Documentation

//! Cloudr is a Rust library that provides an abstract data structure called DataCloud for storing and managing values without moving them. It offers efficient key-value insertion, retrieval, and removal operations, making it convenient to handle your data.

//! ## Table of Contents
//! - Installation
//! - Usage
//!   - Creating a DataCloud
//!   - Inserting and Retrieving Values
//!   - Removing Values
//!   - Checking for Key Existence
//!   - Iterating over Key-Value   - Pairs
//!   - Combining DataCloud Instances

//! - Examples
//! - Contributing
//! - License
//! ## Installation

//! You can add Cloudr as a dependency in your Cargo.toml file:

//! ```toml
//! [dependencies]
//! cloudr = "0.1.0"
//! ```

//! ## Usage
//! ### Creating a DataCloud
//! To start using Cloudr, you need to create an instance of DataCloud. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud: DataCloud<String, i32> = DataCloud::new();
//! ```
//! ### Inserting and Retrieving Values
//! You can insert key-value pairs into the DataCloud and retrieve values using the keys. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud: DataCloud<String, i32> = DataCloud::new();
//! cloud.insert("key".to_string(), 42);

//! if let Some(value) = cloud.get(&"key".to_string()) {
//!     println!("Value: {}", value); // Output: Value: 42
//! }
//! ```

//! ### Removing Values
//! Values can be removed from the DataCloud using the remove method. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud: DataCloud<String, i32> = DataCloud::new();
//! cloud.insert("key".to_string(), 42);

//! if let Some(value) = cloud.remove(&"key".to_string()) {
//!     println!("Removed value: {}", value); // Output: Removed value: 42
//! }
//! ```

//! ### Checking for Key Existence
//! You can check if a key exists in the DataCloud using the contains_key method. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud: DataCloud<String, i32> = DataCloud::new();
//! cloud.insert("key".to_string(), 42);

//! if cloud.contains_key(&"key".to_string()) {
//!     println!("The key exists in the DataCloud.");
//! }
//! ```

//! ### Iterating over Key-Value Pairs
//! You can iterate over the key-value pairs stored in the DataCloud using the into_pairs method. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud: DataCloud<String, i32> = DataCloud::new();
//! cloud.insert("key1".to_string(), 42);
//! cloud.insert("key2".to_string(), 24);

//! for (key, value) in cloud.into_pairs() {
//!     println!("Key: {}, Value: {}", key, value);
//! }
//! ```

//! ### Combining DataCloud Instances
//! You can combine multiple DataCloud instances into a single instance using the combine_with method. Here's an example:

//! ```rust
//! use cloudr::DataCloud;

//! let cloud1: DataCloud<String, i32> = DataCloud::new();
//! cloud1.insert("key1".to_string(), 42);

//! let cloud2: DataCloud<String, i32> = DataCloud::new();
//! cloud2.insert("key2".to_string(), 24);

//! let combined_cloud = cloud1.combine_with(vec![cloud2]);
//! ```

//! ## Examples
//! For more usage examples, please refer to the examples directory in the Cloudr repository.

//! ## Contributing
//! Contributions to Cloudr are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://!github.com/bluefish43/cloudr).

//! ## License
//! Cloudr is licensed under the MIT License.

mod cloud;
pub mod iter;
pub mod error;

pub use cloud::*;