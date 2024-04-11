# Roman numbers converter

This is a simple program that converts Roman numbers to Arabic numbers and vice versa.

The first version of this program was written in CoffeeScript, but I decided to rewrite it in Rust to learn the language.

## Technologies

- [Rust](https://www.rust-lang.org)

### How to run

1. Install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).
2. Navigate into the project folder `rust` and run the following command:

Convert Roman to Arabic:

````sh
```sh
cargo run -- roman IV
````

Expected output is `4`.

Convert Arabic to Roman:

```sh
cargo run -- arabic 2024
```

Expected output is `MMXXIV`.
