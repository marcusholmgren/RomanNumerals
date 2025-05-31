# Roman Numerals Converter (Rust)

This program is a command-line interface (CLI) tool written in Rust that converts Roman numerals to Arabic numbers and vice versa.

## Usage

### Building the Program

You can build the program using Cargo:

-   For a development build:
    ```bash
    cargo build
    ```
-   For a release (optimized) build:
    ```bash
    cargo build --release
    ```

### Running the Executable (CLI)

After building, you can run the executable. The program uses `clap` for command-line argument parsing.

-   To see the help message:
    ```bash
    cargo run -- --help
    ```

-   To convert an Arabic number to a Roman numeral:
    ```bash
    cargo run -- arabic <NUMBER>
    ```
    For example:
    ```bash
    cargo run -- arabic 2023
    ```
    This will output: `MMXXIII`

-   To convert a Roman numeral to an Arabic number:
    ```bash
    cargo run -- roman <NUMERAL>
    ```
    For example:
    ```bash
    cargo run -- roman MMXXIII
    ```
    This will output: `2023`

-   Running the release build directly (example):
    After building with `cargo build --release`, you can run the optimized executable:
    ```bash
    target/release/roman-numerals arabic 2023
    target/release/roman-numerals roman MMXXIII
    ```
    (Note: The exact path might vary based on your project structure if it's part of a workspace, but `target/release/roman-numerals` is typical for a standalone project named `roman-numerals`.)


### Running Tests

To run the test suite:
```bash
cargo test
```

## Technologies

- Rust
- `clap` for command-line argument parsing.
