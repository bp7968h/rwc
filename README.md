# Rust Word Count Project (RWC)
This Rust project is designed to replicate basic functionality of the UNIX `wc` command with additional flexibility provided by Rust for handling I/O and command-line arguments. The project is also part of codingchallenge that I am going through to learn rust.

This is the first rust project that I built and I know this is a naive code, so any suggestions are welcomed.

## Project Structure

The project is divided into several modules, each responsible for a specific part of the application's functionality:

### `main.rs`

- **Purpose**: Serves as the entry point of the application.
- **Functionality**: Initializes configurations from command-line arguments, manages data input (from a file or standard input), processes this input to generate results, and handles the output based on these results.

### `lib.rs`

- **Purpose**: Acts as the core of the library, linking all other modules.
- **Functionality**: Exports the necessary functions and modules needed by `main.rs`, ensuring that all parts of the application can interact seamlessly.

### `io_utils.rs`

- **Purpose**: Contains all I/O related functions.
- **Functionality**: Includes functions to read data from the standard input and from files. This module ensures that data is correctly read into the application regardless of the source.

### `config.rs`

- **Purpose**: Handles the parsing and management of command-line arguments.
- **Functionality**: Provides a structured way to interpret command-line inputs into usable configuration options within the application, such as file names and flags.

## Features

- **Command-line Interface**: Allows users to specify input through command-line arguments, making it versatile for different use cases.
- **Flexible Input Handling**: Can process data from both files and standard input, providing flexibility in how data is supplied to the application.
- **Modular Design**: Each component of the application is isolated in its own module, promoting easier maintenance and scalability.


## Usage
The program can be run by providing a file name and/or flags directly through the command line. If no file is specified, it will attempt to read from standard input.

### Installation
Make sure you have Rust installed.
You can then build the project using the following command:

```
cargo build --release
```

### Command-line Usage
```
rwc [OPTIONS] [FILE]
```
### Options:
```
    -c, --bytes: Print the byte counts.
    -l, --lines: Print the newline counts.
    -w, --words: Print the word counts.
    -m, --chars: Print the character counts.
    [FILE]: Optional. The path to the input file. If not provided, wc reads from standard input.
```

## Examples

### Count lines, words, and characters in a file
```
rwc path/to/file.txt
```

```
rwc -c -l -w -m path/to/file.txt
```

```
rwc -c path/to/file.txt
rwc -l path/to/file.txt
rwc -w path/to/file.txt
rwc -m path/to/file.txt
```

### Count lines, words, and characters in standard input
```
cat path/to/file.txt | rwc 
```
```
cat path/to/file.txt | rwc -c
cat path/to/file.txt | rwc -l
cat path/to/file.txt | rwc -w
cat path/to/file.txt | rwc -m
```

## To Do
- Make it more efficient
- Support Multiple Files
- Use multiple thread to process multiple files

## License

This project is licensed under the MIT License.