# 🔐 Rusty Password Generator (CLI)

A command-line password generator written in Rust.
This project focuses on building a flexible and secure CLI tool while practicing
Rust's type system, error handling, and crate integration.

*While learning Linux I wanted to implement command-line flags into another world, and a simple password generator was a great match for it.*

Passwords are generated instantly based on user preferences for character types
and length, with sensible defaults for quick use.

## Features 🦀

- Generate random passwords with customizable length (default: 16 characters)
- Toggle character sets: lowercase letters (default), uppercase letters, numbers, and symbols
- Clean, formatted output with descriptive messages

**Command-Line Options**:

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--length` | `-L` | Length of the password | 16 |
| `--symbols` | `-s` | Include symbols (!@#$%^&*...) | false |
| `--numbers` | `-n` | Include numbers (0-9) | false |
| `--uppercase` | `-u` | Include uppercase letters (A-Z) | false |
| `--lowercase` | `-l` | Include lowercase letters (a-z) | true |

## Concepts Practiced 🦀

- Struct derivation with clap::Parser for CLI argument parsing
- Pattern matching with Result enum for error handling
- Vector and iterator manipulation for character pool generation
- Random number generation with the rand crate
- String manipulation and ownership rules

## Future Implementations 🦀

- Add password strength estimation
- Copy generated password to clipboard automatically
- Generate multiple passwords at once

---

## How to Run

Make sure you have Rust installed, then run with default settings:
```
cargo run
```

Or customize your password with various options:

```
# Generate 13-character password with all character types
cargo run -- -L 13 -s -n -u

# Generate password with symbols and numbers only
cargo run -- -s -n -l

# See all available options
cargo run -- --help
```