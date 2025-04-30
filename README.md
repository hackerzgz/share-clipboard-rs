# share-clipboard-rs

[![Crates.io](https://img.shields.io/crates/v/share-clipboard-rs.svg)](https://crates.io/crates/share-clipboard-rs) <!-- Optional: Add if published -->
[![Build Status](https://github.com/YOUR_USERNAME/share-clipboard-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/YOUR_USERNAME/share-clipboard-rs/actions) <!-- Optional: Replace with your CI link -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) <!-- Optional: Choose your license -->

A lightweight, cross-platform utility written in Rust to seamlessly share your clipboard content across multiple devices on your local network.

## Inspiration and Compatibility

This project is heavily inspired by the original [share-clipboard](https://github.com/coralw/share-clipboard) created by [coralw](https://github.com/coralw).

A primary goal of `share-clipboard-rs` is to maintain **protocol compatibility** with the original implementation. This means you should be able to use `share-clipboard-rs` instances alongside clients or servers based on the original `share-clipboard` project, allowing for interoperability within the same network.

## Why Rust?

This version was created to leverage the benefits of Rust, including:

*   **Performance:** Efficient execution and low resource usage.
*   **Memory Safety:** Reduced risk of crashes and vulnerabilities.
*   **Cross-Platform:** Easier compilation for Windows, macOS, and Linux from a single codebase.

## Features

*   **Cross-Platform:** Runs on Windows, macOS, and Linux.
*   **Lightweight:** Minimal impact on system resources.
*   **Protocol Compatible:** Interoperates with the original `share-clipboard` ecosystem.
*   **Simple Usage:** Designed to be easy to set up and run.

## Installation

### Option 1: Pre-compiled Binaries (Recommended)

Download the latest pre-compiled binary for your operating system from the [**Releases**](https://github.com/YOUR_USERNAME/share-clipboard-rs/releases) page.

### Option 2: Using Cargo

If you have the Rust toolchain installed (`rustup`), you can install directly from Crates.io (if published):

```bash
cargo install share-clipboard-rs
```

Or, you can build from source:

1. Clone the repository:

```bash

git clone https://github.com/YOUR_USERNAME/share-clipboard-rs.git
cd share-clipboard-rs
```

2. Build the release binary:
```bash
cargo build --release
```

The executable will be located at `target/release/share-clipboard-rs`.
