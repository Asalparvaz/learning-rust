# 📝 Rusty To-Do (CLI)  
  
A command-line to-do list application written in Rust.    
This project focuses on building a clean and extensible CLI program while practicing  
core Rust concepts and thoughtful project structure.  
  
Tasks are managed in memory and can be added, listed, marked as done, searched, and deleted  
through simple text commands.  
  
## Features 🦀  
  
- Add new tasks with auto-incremented IDs  
- List all tasks with status (Pending / Done)  
- Mark tasks as completed  
- Search for a task by ID  
- Delete individual tasks  
- Clear all tasks  
- Interactive command loop with helpful error messages  
  
## Concepts Practiced 🦀  
  
- Structs and encapsulation  
- Vectors and in-memory data management  
- Mutable state handling  
- Module separation (`main.rs` / `task.rs`)  
- Command parsing and input validation  
- Pattern matching and control flow  
- Designing extensible CLI programs  
  
## Future Implementations 🦀  
  
- Persist tasks using a database  
- Load tasks on startup and save changes automatically  
  
## How to Run  
  
Make sure you have Rust installed, then run:  
  
```bash  
cargo run
```
Once running, type `help` to see available commands.