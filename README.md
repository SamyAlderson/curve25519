# Curve25519
[![Rust](https://img.shields.io/badge/Language-Rust-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/samyalderson/curve25519/actions/workflows/rust.yml/badge.svg)](https://github.com/samyalderson/curve25519/actions/workflows/rust.yml)

## One-line tagline
Efficient and secure implementation of the Curve25519 elliptic curve.

## Overview
The Curve25519 project provides a robust and reliable implementation of the Curve25519 elliptic curve in Rust. This library offers features for key generation, signature generation, and scalar multiplication. It is designed for use in cryptographic applications requiring high-security elliptic curve mathematics.

## Features
* Key generation
* Signature generation
* Scalar multiplication

## Prerequisites
* Rust (stable) installed on your system
* Cargo (Rust package manager)

## Installation
To install Curve25519, simply run the following command in your terminal:
```bash
cargo install --git https://github.com/samyalderson/curve25519.git
```
Or, if you prefer, clone the repository and build it manually:
```bash
git clone https://github.com/samyalderson/curve25519.git
cd curve25519
cargo build
```
## Usage
Here's an example of generating a key pair and generating a signature:
```rust
use curve25519::{PrivateKey, Signature};

let secret_key = PrivateKey::generate();
let public_key = secret_key.to_public_key();
let message = b"Hello, World!";
let signature = Signature::generate(message, &public_key);
```
## Project Architecture / Structure
The Curve25519 library is structured into the following components:
* `src/lib.rs`: Main library implementation
* `src/utils.rs`: Utility functions for curve operations
* `tests/test_lib.rs`: Unit tests for the library
* `docs/src/README.md`: Project documentation

## Building from Source
To build the Curve25519 library from source, run the following command:
```bash
cargo build
```
## Testing
The Curve25519 library comes with a suite of unit tests to ensure correctness. You can run the tests using:
```bash
cargo test
```
## Contributing Guidelines
Contributions are welcome! If you'd like to contribute to the Curve25519 library, please follow these guidelines:
* Fork the repository and create a new branch for your changes
* Make your changes and commit them to the branch
* Create a pull request for review

## License
This project is licensed under the MIT License.