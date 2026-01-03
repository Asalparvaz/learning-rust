enum Difficulty {
    Easy = 10,
    Medium = 5,
    Hard = 3,
}


fn main() {
    loop {
        run_game();
        if !continue_playing() {
            break
        }
    }
    println!("Thank you for playing. Goodbye.");
}

fn get_chances() -> u32 {
    loop {
        let difficulty: String = read_input("Please select the difficulty level:\n1. Easy (10 chances)\n2. Medium (5 chances)\n3. Hard (3 chances)\nEnter here: ");
        match difficulty.trim() {
            "1" => break Difficulty::Easy as u32,
            "2" => break Difficulty::Medium as u32,
            "3" => break Difficulty::Hard as u32,
            _ => println!("Invalid input."),
        }
    }
}

fn get_valid_guess() -> u32 {
    loop {
        let num: String = read_input("\nEnter your guess: ");
        match num.trim().parse::<u32>() {
            Ok(n) if n <= 100 => break n,
            Ok(_) => println!("Number must be between 0 and 100"),
            Err(_) => println!("Please enter a valid input."),
        }
    }
}

fn run_chances(chances: u32, actual_number: u32) {
    use std::time::Instant;

    let mut guessed : bool = false;
    let start = Instant::now();

    for i in 0..chances {
        let guess: u32 = get_valid_guess();
        if guess == actual_number {
            println!("Congratulations! You guessed the correct number in {} attempts", i+1);
            guessed = true;
            break;
        } else if guess < actual_number {
            println!("Incorrect! The number is greater than {}.", guess);
        } else {
            println!("Incorrect! The number is less than {}.", guess);
        }
        println!("You have {} chances left", chances - i -1);
    }

    if !guessed {
        println!("You Lost. The number was {}.", actual_number);
    }
    
    let elapsed = start.elapsed();
    println!("It took you {:.2?} to finish the game.", elapsed);

}

fn run_game() {
    use rand::Rng;
    println!("\nWelcome to the Number Guessing Game!\nI'm thinking of a number between 0 and 100.");
    let chances: u32 = get_chances();
    println!("\nYou have {} Chances!\nLet's start the game.", chances);

    let mut rng = rand::thread_rng();
    let actual_number: u32 = rng.gen_range(0..=100);
    run_chances(chances, actual_number);
}

fn continue_playing() -> bool {
    loop {
        let input: String = read_input("\nDo you want to continue playing? (yes/no) : ");
        match input.trim() {
            "yes" => break true,
            "no" => break false,
            _ => println!("Please enter a valid input."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
