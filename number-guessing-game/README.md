# 🎯 Number Guessing Game (Rust) 

A command-line number guessing game written in Rust.  
The program randomly generates a number between 0 and 100, and the player must guess
the number within a limited number of attempts based on the selected difficulty level.

## Features 🦀

- Three difficulty levels:
  - Easy (10 attempts)
  - Medium (5 attempts)
  - Hard (3 attempts)
- Input validation for guesses and menu selections
- Feedback after each guess (higher / lower)
- Game duration tracking using `std::time::Instant`
- Option to replay the game

## Concepts Practiced 🦀

- Enums (`Difficulty`)
- Loops and control flow
- Input handling and validation
- Pattern matching with `match`
- Random number generation using `rand`
- Basic performance timing
- Writing clean, readable functions

## How to run

Make sure you have Rust installed, then run:

```bash
cargo run