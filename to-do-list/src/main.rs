mod task;
use task::TaskManager;

fn main() {
    process_input()
}

fn process_input() {
    println!("Welcome to Rusty To-Do!\nType 'help' to see available commands.");
    let mut manager = TaskManager::new();
    loop{
        let input: String = read_input("> ");
        let splits: Vec<&str> = input.split_whitespace().collect();
        let command = match splits.get(0) {
            Some(cmd) => cmd.to_lowercase(),
            None => continue,
        };

        let args = &splits[1..];

        match command.as_str() {
            "add" => {
                if args.is_empty() {
                    println!("ERROR: Missing task description.");
                } else {
                    let description = args.join(" ");
                    manager.add_task(description);
                }
            }
            "list" => {
                manager.show_list();
            }
            "done" => { 
                if let Some(id_str) = args.get(0) { 
                    match id_str.trim().parse::<usize>() { 
                        Ok(id) => manager.done_task(id), 
                        Err(_) => println!("ERROR: Task ID must be a number"), 
                    } 
                } else { 
                    println!("ERROR: Missing task ID"); 
                } 
            }
            "search" => {
                if let Some(id_str) = args.get(0) { 
                    match id_str.trim().parse::<usize>() { 
                        Ok(id) => manager.search_task(id), 
                        Err(_) => println!("ERROR: Task ID must be a number"), 
                    } 
                } else { 
                    println!("ERROR: Missing task ID"); 
                } 
            }
            "delete" => {
                if let Some(id_str) = args.get(0) { 
                    match id_str.trim().parse::<usize>() { 
                        Ok(id) => manager.delete_task(id), 
                        Err(_) => println!("ERROR: Task ID must be a number"), 
                    } 
                } else { 
                    println!("ERROR: Missing task ID"); 
                } 
            }
            "clear" => {
                manager.clear_all();
            }
            "help" => {
                print_help();
            }
            "exit" => {
                println!("... And in case I don't see ya, good afternoon, good evening, and goodnight.");
                return;
            }
            _ => {
                println!("Unknwon command : {}.", command);
            }
        }

    }
    
}

fn print_help() {
    println!(
        r#"Commands:
add "task"     Add a task
list           List all tasks
done <id>      Mark task as done
search <id>    Search for a task
delete <id>    Delete a task
clear          Remove all tasks
exit           Quit program"#
    );
}


fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}