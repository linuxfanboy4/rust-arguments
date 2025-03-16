# Rust Arguments Parser

## Overview

The **Rust Arguments Parser** is a robust and powerful library designed to facilitate the parsing of command-line arguments in Rust applications. It provides a highly configurable and extensible framework for defining, validating, and processing command-line inputs, making it an indispensable tool for developers who require advanced argument handling in their projects.

## Features

- **Flexible Argument Definition**: Define arguments with short (`-`), long (`--`), or positional formats.
- **Value Handling**: Supports arguments that require values, with optional validation.
- **Default Values**: Specify default values for arguments to ensure consistent behavior.
- **Validation**: Integrate custom validation logic to enforce constraints on argument values.
- **Subcommands**: Organize complex command-line interfaces with nested subcommands.
- **Error Handling**: Panics on invalid inputs, ensuring that only valid configurations are processed.

## Installation

To integrate the Rust Arguments Parser into your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rust-arguments = { git = "https://github.com/linuxfanboy4/rust-arguments.git" }
```

## Usage

### Basic Example

Below is a simple example demonstrating how to define and parse command-line arguments using the Rust Arguments Parser:

```rust
use rust_arguments::{ArgParser, ArgMatches};

fn main() {
    let parser = ArgParser::new()
        .arg("input")
        .short("input", 'i')
        .long("input", "input-file")
        .takes_value("input")
        .required("input")
        .default("input", "default.txt")
        .validator("input", |val| val.ends_with(".txt"));

    let args: Vec<String> = std::env::args().collect();
    let matches = parser.parse(&args);

    println!("Input file: {}", matches.values.get("input").unwrap());
}
```

### Advanced Example with Subcommands

For more complex applications, subcommands can be utilized to create a hierarchical command structure:

```rust
use rust_arguments::{ArgParser, ArgMatches};

fn main() {
    let sub_parser = ArgParser::new()
        .arg("output")
        .short("output", 'o')
        .long("output", "output-file")
        .takes_value("output")
        .required("output");

    let parser = ArgParser::new()
        .arg("input")
        .short("input", 'i')
        .long("input", "input-file")
        .takes_value("input")
        .required("input")
        .subcommand("process", sub_parser);

    let args: Vec<String> = std::env::args().collect();
    let matches = parser.parse(&args);

    if let Some(sub_matches) = matches.subcommand {
        println!("Processing with output file: {}", sub_matches.values.get("output").unwrap());
    } else {
        println!("Input file: {}", matches.values.get("input").unwrap());
    }
}
```

## API Documentation

### `ArgParser`

The `ArgParser` struct is the core component of the library, responsible for defining and parsing command-line arguments.

#### Methods

- **`new()`**: Initializes a new `ArgParser` instance.
- **`arg(name: &str)`**: Adds a new argument with the specified name.
- **`short(name: &str, short: char)`**: Assigns a short flag to the specified argument.
- **`long(name: &str, long: &str)`**: Assigns a long flag to the specified argument.
- **`takes_value(name: &str)`**: Specifies that the argument requires a value.
- **`required(name: &str)`**: Marks the argument as required.
- **`default(name: &str, default: &str)`**: Sets a default value for the argument.
- **`validator(name: &str, validator: F)`**: Attaches a custom validation function to the argument.
- **`subcommand(name: &str, parser: ArgParser)`**: Adds a subcommand to the parser.
- **`parse(args: &[String])`**: Parses the provided arguments and returns an `ArgMatches` instance.

### `ArgMatches`

The `ArgMatches` struct encapsulates the results of the argument parsing process.

#### Fields

- **`values: HashMap<String, String>`**: Contains the values of arguments that require them.
- **`flags: HashMap<String, bool>`**: Indicates the presence of flag arguments.
- **`positionals: Vec<String>`**: Holds positional arguments.

## Contributing

Contributions to the Rust Arguments Parser are welcome. Please ensure that your contributions adhere to the following guidelines:

1. **Code Style**: Follow the Rust coding standards.
2. **Testing**: Include tests for new features or bug fixes.
3. **Documentation**: Update the documentation to reflect any changes.

To contribute, fork the repository, create a branch for your feature, and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
