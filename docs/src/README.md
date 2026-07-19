# Curve25519: Rust Implementation of the Elliptic Curve
==============================================

### Tagline

Efficient and secure Curve25519 implementation in Rust

### Overview

The Curve25519 elliptic curve is a state-of-the-art cryptographic primitive, widely used in various applications such as key exchange, digital signatures, and more. This project provides a Rust implementation of the Curve25519 curve, focusing on production-quality code, proper error handling, and clean architecture.

### Features

* Key generation using the Curve25519 public key format
* Signature generation using the Ed25519 signature scheme
* Scalar multiplication for efficient point operations

### Quick Start

1. Clone the repository: `git clone https://github.com/samyalderson/curve25519`
2. Install the required dependencies: `cargo build`
3. Run the unit tests: `cargo test`

### Usage

1. Import the `curve25519` crate in your Rust project: `extern crate curve25519;`
2. Use the `Key` and `Signature` types to generate and verify keys and signatures
   ```rust
use curve25519::Key;
use curve25519::Signature;

// Generate a new key pair
let key = Key::generate();
let signature = Signature::generate(&key, b"message");

// Verify the signature
let is_valid = signature.verify(&key, b"message");
```
### Building

1. Make sure you have Rust installed on your system
2. Run `cargo build` to build the library
3. Run `cargo test` to run the unit tests

### Architecture

The project consists of the following components:

* `lib.rs`: Main library implementation, containing the `Key` and `Signature` types
* `utils.rs`: Utility functions, including scalar multiplication and key generation
* `test_lib.rs`: Unit tests for the library

### Testing

The project includes unit tests for the library, covering key generation, signature generation, and scalar multiplication. Run `cargo test` to execute the tests.

### Contributing

Contributions are welcome! Please submit pull requests or issues with any suggestions, bug reports, or feature requests.

### License

This project is licensed under the MIT License. See the LICENSE file for details.

### Credits

This project was developed by Samy Alderson, with inspiration from various open-source projects and cryptographic libraries.