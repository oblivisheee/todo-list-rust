mod todo;

use todo::TodoList;

fn main() {
    loop {
        let mut todo = TodoList::new();
        println!("1. Add task\n2. Complete task\n3. Print tasks\n4. Exit\n\nWrite your choice: ");

        // Read user input as a string
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse input string to u32
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task:");
                let mut task = String::new();
                std::io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");

                // Trim whitespace
                let task = task.trim().to_string();

                todo.add(task);
                todo.print();
            }
            2 => {
                todo.print();

                println!("Enter index of task to complete:");
                let mut index = String::new();
                std::io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index");
                        continue;
                    }
                };

                todo.complete(index);
            }
            3 => {
                todo.print();
            }
            4 => {
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
