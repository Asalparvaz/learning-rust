use clap::Parser;
use rand::Rng;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short = 'L', long, default_value_t = 16)]
    length: usize,

    /// Include symbols in the password
    #[arg(short('s'), long)]
    symbols: bool,

    /// Include numbers in the password
    #[arg(short('n'), long)]
    numbers: bool,

    /// Include uppercase letters in the password
    #[arg(short('u'), long("uppercase"))]
    uppercase: bool,

    /// Include lowercase letters in the password (default)
    #[arg(short('l'), long("lowercase"), default_value_t = true)]
    lowercase: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match generate_password(&args) {
        Ok(password) => {
            println!("Generated password: {}", password);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn generate_password(args: &Args) -> Result<String, String> {
    let lowercase_chars = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let number_chars = "0123456789";
    let symbol_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut char_pool = String::new();

    if args.lowercase {
        char_pool.push_str(lowercase_chars);
    }
    if args.uppercase {
        char_pool.push_str(uppercase_chars);
    }
    if args.numbers {
        char_pool.push_str(number_chars);
    }
    if args.symbols {
        char_pool.push_str(symbol_chars);
    }

    if char_pool.is_empty() {
        return Err("At least one character type must be selected".to_string());
    }

    let char_pool: Vec<char> = char_pool.chars().collect();

    let mut rng = rand::thread_rng();
    let password: String = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..char_pool.len());
            char_pool[idx]
        })
        .collect();

    Ok(password)
}