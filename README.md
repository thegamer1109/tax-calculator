# tax-calculator
This program calculates both federal and Georgia state taxes for 2024

## About

This uses Slint for the UI and some simple Rust if-else statements for calculating federal and GA state income taxes. 

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up a sample project with this template
    ```
    cargo generate --git https://github.com/thegamer1109/tax-calculator --name my-project
    cd my-project
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```

## TODO

1. Implement multiple filing status
2. Add different states
3. Add deductions (first standard then others)
