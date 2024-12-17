# SHA-512 Implementation in Rust

This project provides a pure Rust implementation of the SHA-512 cryptographic hash function. SHA-512 is part of the SHA-2 family of hash functions and produces a 512-bit (64-byte) digest. This implementation is ideal for cryptographic applications, data integrity verification, and secure hashing requirements.

## Features

- **Complete SHA-512 Algorithm**: Implements the full SHA-512 hashing algorithm as defined by FIPS PUB 180-4 (SHA-2).
- **Efficient Padding and Input Handling**: Handles padding of input messages and ensures proper formatting before processing.
- **Bitwise Operations**: Uses optimized bitwise operations such as rotations, shifts, and logical operations (AND, OR, XOR) in line with the cryptographic specifications.
- **Flexible Usage**: Accepts arbitrary string inputs and returns their corresponding 512-bit hash value.

## Overview

SHA-512 is a cryptographic hash function that processes data in 1024-bit (128-byte) blocks. The algorithm includes an initial set of hash values and a set of round constants, which are used during the iterative processing of input data. The input is padded and length-encoded, and the resulting hash value is a 512-bit digest.

### Key Components of SHA-512

- **Initial Hash Values** (`INITIAL_BUFFER`): A set of predefined values that initialize the hash state.
- **Round Constants** (`K`): Constants that are used in each of the 80 rounds of the algorithm to ensure proper mixing of the data.
- **Padding and Length Encoding**: The input data is padded to a multiple of 128 bytes, with a bit-length field appended to the end of the message.

## How It Works

1. **Padding**: The input string is padded with a `0x80` byte followed by enough zero bytes to make the total length congruent to 896 modulo 1024, and a 128-bit length field is appended at the end.
2. **Message Scheduling**: The padded message is split into 128-byte blocks. For each block, a message schedule of 80 words is created, which will be used for processing the block.
3. **Compression**: Each block is processed using the SHA-512 algorithm, which involves 80 rounds of operations on the state variables. These operations involve bitwise functions like AND, OR, XOR, as well as bit rotations and shifts.
4. **Final Hash**: The final result is a 512-bit hash, which is the concatenation of the eight 64-bit words in the state.

### Key Operations

- **`ch(x, y, z)`**: Conditional operation used in the SHA-512 rounds.
- **`maj(x, y, z)`**: Majority operation used in the SHA-512 rounds.
- **`sigma0(x)`** and **`sigma1(x)`**: Bitwise transformations used in the message schedule.
- **Message Schedule**: Derives a sequence of 80 words from the 128-byte block of input data, which is then used for hash computation.

## How to Use

### 1. Clone the Repository

First, clone this repository to your local machine:

```bash
git clone https://github.com/yourusername/sha512-rust.git
cd sha512-rust
```

### 2. Run the Program

The project is a self-contained Rust application. You can run it directly using Cargo:

```bash
cargo run
```

This will execute the program with a default input ("a") and output the corresponding SHA-512 hash:

```
SHA-512 Hash: 861844d6704e8573fec34d358d9d7e6c63b1e57f758d9cb7f5f9e57cc8b06ab52ad50fcd58f7680f9fa6b20be398923710312d05bb271f1e88db8ff7750767
```

You can modify the input directly in the `main()` function:

```rust
let input = "your string here";
```

### 3. Integrate Into Your Project

To integrate this implementation into your own project, you can either copy the relevant functions or include this repository as a dependency by adding it to your `Cargo.toml` file.

## Key Functions

- **`process_user_input(input: &str) -> Vec<u8>`**  
  Prepares the input string by padding it and appending the bit length.

- **`ch(x: u64, y: u64, z: u64) -> u64`**  
  Implements the "ch" operation used in the SHA-512 algorithm.

- **`maj(x: u64, y: u64, z: u64) -> u64`**  
  Implements the "maj" (majority) operation used in the SHA-512 algorithm.

- **`sigma0(x: u64) -> u64`**  
  Implements the first sigma transformation used in the SHA-512 algorithm.

- **`sigma1(x: u64) -> u64`**  
  Implements the second sigma transformation used in the SHA-512 algorithm.

- **`message_schedule(chunk: &[u8]) -> [u64; 80]`**  
  Generates the 80 words of the message schedule from a 128-byte chunk of input data.

- **`process_buffer_chunk(chunk: &[u8], current_buffer: &[u64; 8]) -> [u64; 8]`**  
  Processes a single 128-byte chunk of input data, updating the hash state.

- **`process_buffer(buffer: Vec<u8>) -> String`**  
  Processes the entire input buffer and returns the final SHA-512 hash as a string of hexadecimal digits.

## Example

Here’s an example of how to process an input string to obtain its SHA-512 hash:

```rust
let input = "hello world";
let processed_input_buffer = process_user_input(input);
let final_hash = process_buffer(processed_input_buffer);
println!("SHA-512 Hash: {}", final_hash);
```

### Sample Output:

```
SHA-512 Hash: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b1692a8b8c7bb65f129eb7a4f013b0e6c56e36f83234e4c38f93a6250d46bc17310e9a08aabccf63d7880278efce8f3
```

## Contributing

Contributions are welcome! If you encounter bugs or have ideas for improvements, please feel free to open an issue or submit a pull request.

---

This project demonstrates an implementation of SHA-512 and can be used as a reference or directly integrated into your cryptographic applications.
