# CLI Util for Pattern Search in File

## Overview
This command-line utility allows you to search for a specific pattern in a file and display the lines that contain it. It is a simple and efficient tool for quickly identifying relevant information within a file.

## Usage
To use the utility, provide the pattern to search for and the path to the file as command-line arguments. The utility will then read the file, search for the specified pattern, and display the lines that contain it.

## Installation
To build and run the utility, you need to have Rust installed. If Rust is not installed, you can get it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Clone the repository:
```bash
git clone https://github.com/your/repository.git
cd repository
```

Build and run the utility:
```bash
cargo run --release -- -p <pattern> -f <file_path>
```

Replace `<pattern>` with the pattern you want to search for and `<file_path>` with the path to the file.

## Command-line Options
- `-p, --pattern`: Specify the pattern to search for in the file.
- `-f, --file`: Specify the path to the file to be searched.

## Example
```bash
cargo run --release -- -p "error" -f path/to/log.txt
```
This will search for the pattern "error" in the file located at "path/to/log.txt" and display the lines containing the specified pattern.

## Dependencies
- [clap](https://crates.io/crates/clap): Command line argument parsing.
- [anyhow](https://crates.io/crates/anyhow): Error handling.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions
Contributions are welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or create a pull request.

## Acknowledgments
- This utility was developed using Rust programming language.
- Special thanks to the creators and maintainers of the `clap` and `anyhow` crates for simplifying argument parsing and error handling in Rust.
