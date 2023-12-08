# Merkle Tree in Rust

This is a Rust program that implements a Merkle Tree.

## Prerequisites

Before running the program, make sure you have Rust installed on your machine. If you don't have it installed yet, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) to install Rust.

## Running the Program

To run the program, follow these steps:

1. Open a terminal.

2. Navigate to the root directory of the project, where the `main.rs` file is located.

3. Compile the Rust program using the `cargo build` command:

   ```bash
   cargo build
   ```

After a successful compilation, you can execute the program with the cargo run command:

```bash
cargo run
```

## How Merkle Tree Works
The Merkle Tree is a binary tree data structure primarily used in cryptography and data integrity verification. It is constructed from hashes of individual data items and can be used to verify if a specific piece of data belongs to a larger set of data without the need to transmit or store all the data. Each node in the tree contains a hash, and hashes are combined hierarchically until the root of the tree is reached.

In this project, we have implemented a Merkle Tree in Rust for educational and demonstrative purposes.

## Contribution
Feel free to contribute improvements to this project. You can open issues or submit pull requests with your changes.
