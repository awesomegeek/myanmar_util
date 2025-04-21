# Myanmar Util

A collection of Rust utilities for processing Myanmar (Burmese) text.

## Features

- **Syllable Breaking**: Accurately breaks Myanmar text into syllables following linguistic rules
- **Command Line Interface**: Easy-to-use CLI for text processing
- **Regular Expression Utilities**: Special regex patterns designed for Myanmar text analysis

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
myanmar_util = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install myanmar_util
```

## Usage

### As a Library

```rust
use myanmar_util::{syllable_break, syllable_break_phoneme};

fn main() {
    // Break text into syllables
    let text = "မင်္ဂလာပါ";


    let syllable_text = syllable_break(&text, Some("|"))
    println!("{}", syllable_text);  // မင်္ဂ|လာ|ပါ

    let syllable_text = syllable_break_phoneme(&text, Some("|"))
    println!("{}", syllable_text);  // မင်|ဂ|လာ|ပါ

}
```

### Command Line

```bash
# Break text into syllables with default separator (|)
myanmar_util syllablebreak -s "|" -i input.txt -o output.txt -t "M"

```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.