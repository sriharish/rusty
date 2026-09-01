# 🦀 Rusty

A repository dedicated to tracking my daily progress, notes, and code as I learn Rust.

## 📊 Progress Tracker

* **Start Date:** September 1, 2026
* **Current Status:** Day 1
* **Total Days Active:** 1 Day
* **Latest Milestone:** Repository setup and "Hello, World!"

## 📅 The Rust Book Daily Log

### 🟢 Getting Started & Basics
- [x] **Day 1: Ch 1 - Getting Started**
    - Installed `rustup`, `rustc`, and `cargo`.
    - Wrote "Hello, World!" manually and via Cargo.
- [ ] **Day 2: Ch 2 - Programming a Guessing Game**
    - Built a CLI game using `std::io` and the `rand` crate.
    - Learned about variables, associated functions, and `match`.
- [ ] **Day 3: Ch 3 - Common Programming Concepts**
    - Studied variables, mutability, constants, and shadowing.
    - Explored scalar/compound data types, functions, and control flow.

### 🟡 Core Concepts
- [ ] **Day 4: Ch 4 - Understanding Ownership**
    - Mastered stack vs. heap allocation and ownership rules.
    - Experimented with memory moves, clones, and data copying.
- [ ] **Day 5: Ch 4 - References, Borrowing, & Slices**
    - Implemented references to prevent ownership transfer.
    - Studied data races, mutable reference restrictions, and string slices.
- [ ] **Day 6: Ch 5 - Using Structs to Structure Related Data**
    - Defined and instantiated classic, tuple, and unit-like structs.
    - Wrote methods and associated functions using `impl` blocks.
- [ ] **Day 7: Ch 6 - Enums and Pattern Matching**
    - Created enums and coupled data with variants.
    - Used `match` with the `Option<T>` enum and the `if let` syntax.

### 🔵 Projects & Organization
- [ ] **Day 8: Ch 7 - Managing Growing Projects**
    - Structured code using packages, crates, modules, and paths.
    - Brought paths into scope using the `use` keyword.
- [ ] **Day 9: Ch 8 - Common Collections**
    - Stored lists of data using Vectors (`Vec<T>`).
    - Stored UTF-8 encoded text with Strings and key-value pairs with HashMaps.
- [ ] **Day 10: Ch 9 - Error Handling**
    - Handled unrecoverable errors with `panic!`.
    - Recovered from errors using `Result<T, E>` and the `?` operator.

### 🟣 Advanced Features
- [ ] **Day 11: Ch 10 - Generic Types, Traits, and Lifetimes**
    - Removed code duplication using generic data types.
    - Defined shared behavior with Traits and validated references with Lifetimes.
- [ ] **Day 12: Ch 11 & 12 - Testing & Building a CLI Tool**
    - Wrote unit and integration tests.
    - Built a functional `minigrep` command-line tool.

## 📁 Repository Structure

- The 'rusty' root is a workspace with multiple packages
  - `hello/` - First package to test hello world
  - `guessing/` - Package for guessing game exercise in rust book

## 🛠️ Environment Setup

* **OS:** [e.g., macOS / Ubuntu / Windows]
* **Compiler:** `rustc 1.98.x`
* **Package Manager:** `cargo`
* **Editor:** Rust Rover 2026.1.3

## Useful commands

- **Create**: `cargo new app_two --bin`
- **Build**: `cargo build -p [package name]`
- **Run**: `cargo run -p [package name]`

## 📚 Written Resources

* [The Rust Programming Language Book]([https://rust-lang.org](https://doc.rust-lang.org/book/title-page.html))

## 🎬 Videos

- [Quick primer before learning](https://www.youtube.com/watch?v=br3GIIQeefY&t=83s)


